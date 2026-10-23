//! Experimental M0 offline decoder probe. Never call from an audio callback.
use std::{
    io::Cursor,
    sync::atomic::{AtomicU64, Ordering},
};
use symphonia::core::{
    audio::GenericAudioBufferRef,
    codecs::audio::{
        AudioDecoderOptions,
        well_known::{CODEC_ID_FLAC, CODEC_ID_MP3, CODEC_ID_PCM_S24LE, CODEC_ID_PCM_S32LE},
    },
    errors::Error,
    formats::{TrackType, probe::Hint},
    io::{MediaSource, MediaSourceStream},
};

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_frames: usize,
    pub max_rate: u32,
    pub max_channels: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_bytes: 16 * 1024 * 1024,
            max_frames: 480_000,
            max_rate: 192_000,
            max_channels: 8,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Samples {
    Integer(Vec<i32>),
    Float(Vec<f32>),
}

#[derive(Debug)]
pub struct Decoded {
    pub rate: u32,
    pub channels: usize,
    pub bits: Option<u32>,
    pub frames: usize,
    pub delay: Option<u32>,
    pub padding: Option<u32>,
    pub samples: Samples,
}

#[derive(Debug, PartialEq)]
pub enum DecodeError {
    Cancelled,
    Limit,
    Invalid,
    Unsupported,
    Changed,
}

#[derive(Debug)]
pub struct Binding {
    pub container: &'static str,
    pub profile: &'static str,
    pub backend: &'static str,
    pub version: &'static str,
    /// Feasibility evidence only: never a product RuntimeCapability.
    pub status: &'static str,
}

pub fn registry() -> &'static [Binding] {
    &[
        Binding {
            container: "WAV",
            profile: "PCM signed little-endian 24/32",
            backend: "Symphonia",
            version: "0.6.1",
            status: "experimental",
        },
        Binding {
            container: "FLAC",
            profile: "PCM24 native FLAC",
            backend: "Symphonia",
            version: "0.6.1",
            status: "experimental",
        },
        Binding {
            container: "MPEG audio",
            profile: "Layer III CBR/VBR with trusted Xing/LAME delay and padding",
            backend: "Symphonia",
            version: "0.6.1",
            status: "experimental",
        },
    ]
}

/// Decode one complete, small file. This is a bounded feasibility probe, not a
/// streaming player. Integer samples use the full i32 range (24-bit LSBs are 0).
/// The caller must recheck its generation when handing this result to a consumer.
pub fn decode(
    bytes: Vec<u8>,
    limits: Limits,
    generation: &AtomicU64,
    expected_generation: u64,
) -> Result<Decoded, DecodeError> {
    let check_generation = || {
        if generation.load(Ordering::Acquire) == expected_generation {
            Ok(())
        } else {
            Err(DecodeError::Cancelled)
        }
    };
    check_generation()?;
    if bytes.len() > limits.max_bytes {
        return Err(DecodeError::Limit);
    }
    decode_source(
        Box::new(Cursor::new(bytes)),
        limits,
        generation,
        expected_generation,
    )
}

/// Test seam: decode from an arbitrary source so tests can change the
/// generation deterministically while the decode is in progress. The caller
/// is responsible for the `max_bytes` budget.
#[doc(hidden)]
pub fn decode_source(
    source: Box<dyn MediaSource>,
    limits: Limits,
    generation: &AtomicU64,
    expected_generation: u64,
) -> Result<Decoded, DecodeError> {
    let check_generation = || {
        if generation.load(Ordering::Acquire) == expected_generation {
            Ok(())
        } else {
            Err(DecodeError::Cancelled)
        }
    };
    check_generation()?;
    let stream = MediaSourceStream::new(source, Default::default());
    let mut format = symphonia::default::get_probe()
        .probe(&Hint::new(), stream, Default::default(), Default::default())
        .map_err(map_error)?;
    check_generation()?;
    let track = format
        .default_track(TrackType::Audio)
        .ok_or(DecodeError::Unsupported)?;
    let params = track
        .codec_params
        .as_ref()
        .and_then(|p| p.audio())
        .ok_or(DecodeError::Unsupported)?;
    let integer = match params.codec {
        CODEC_ID_FLAC if params.bits_per_sample == Some(24) => true,
        CODEC_ID_PCM_S24LE | CODEC_ID_PCM_S32LE => true,
        CODEC_ID_MP3
            if track.delay.is_some() && track.padding.is_some() && track.num_frames.is_some() =>
        {
            false
        }
        _ => return Err(DecodeError::Unsupported),
    };
    let rate = params.sample_rate.ok_or(DecodeError::Invalid)?;
    let channels = params
        .channels
        .as_ref()
        .ok_or(DecodeError::Invalid)?
        .count();
    if rate == 0 || channels == 0 {
        return Err(DecodeError::Invalid);
    }
    if rate > limits.max_rate
        || channels > limits.max_channels
        || track
            .num_frames
            .is_some_and(|n| n > limits.max_frames as u64)
        || params
            .max_frames_per_packet
            .is_some_and(|n| n > limits.max_frames as u64)
    {
        return Err(DecodeError::Limit);
    }
    let expected_frames = track.num_frames;
    let track_id = track.id;
    let mut result = Decoded {
        rate,
        channels,
        bits: if integer {
            params.bits_per_sample
        } else {
            None
        },
        frames: 0,
        delay: track.delay,
        padding: track.padding,
        samples: if integer {
            Samples::Integer(Vec::new())
        } else {
            Samples::Float(Vec::new())
        },
    };
    let options = AudioDecoderOptions::default().gapless(true).verify(true);
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(params, &options)
        .map_err(map_error)?;
    let mut initial_spec = None;
    loop {
        check_generation()?;
        let Some(packet) = format.next_packet().map_err(map_error)? else {
            break;
        };
        if packet.track_id != track_id {
            continue;
        }
        let audio = decoder.decode(&packet).map_err(map_error)?;
        check_generation()?;
        if audio.spec().rate() != rate
            || audio.spec().channels().count() != channels
            || initial_spec.as_ref().is_some_and(|s| s != audio.spec())
        {
            return Err(DecodeError::Changed);
        }
        initial_spec.get_or_insert_with(|| audio.spec().clone());
        let frames = result
            .frames
            .checked_add(audio.frames())
            .ok_or(DecodeError::Limit)?;
        if frames > limits.max_frames {
            return Err(DecodeError::Limit);
        }
        let count = audio
            .frames()
            .checked_mul(channels)
            .ok_or(DecodeError::Limit)?;
        match &mut result.samples {
            Samples::Integer(samples) => {
                // Reject a float decoder result rather than silently lose PCM32 LSBs.
                if matches!(
                    audio,
                    GenericAudioBufferRef::F32(_) | GenericAudioBufferRef::F64(_)
                ) {
                    return Err(DecodeError::Changed);
                }
                let start = samples.len();
                samples.try_reserve(count).map_err(|_| DecodeError::Limit)?;
                samples.resize(start + count, 0);
                audio.copy_to_slice_interleaved(&mut samples[start..]);
            }
            Samples::Float(samples) => {
                if !matches!(audio, GenericAudioBufferRef::F32(_)) {
                    return Err(DecodeError::Changed);
                }
                let start = samples.len();
                samples.try_reserve(count).map_err(|_| DecodeError::Limit)?;
                samples.resize(start + count, 0.0);
                audio.copy_to_slice_interleaved(&mut samples[start..]);
                if samples[start..].iter().any(|s| !s.is_finite()) {
                    return Err(DecodeError::Invalid);
                }
            }
        }
        result.frames = frames;
    }
    if result.frames == 0
        || expected_frames.is_some_and(|n| n != result.frames as u64)
        || decoder.finalize().verify_ok == Some(false)
    {
        return Err(DecodeError::Invalid);
    }
    check_generation()?;
    Ok(result)
}

fn map_error(error: Error) -> DecodeError {
    match error {
        Error::Unsupported(_) => DecodeError::Unsupported,
        Error::ResetRequired => DecodeError::Changed,
        _ => DecodeError::Invalid,
    }
}
