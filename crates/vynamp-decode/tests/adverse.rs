use std::{
    fs,
    io::{Cursor, Read, Seek, SeekFrom},
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
};
use symphonia::core::io::MediaSource;
use vynamp_decode::{DecodeError, Limits, Samples, decode, decode_source, registry};

fn fixture(name: &str) -> Vec<u8> {
    fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/audio/decode")
            .join(name),
    )
    .unwrap()
}

#[test]
fn registry_describes_only_experimental_probed_bindings() {
    let r = registry();
    assert_eq!(r.len(), 3);
    assert_eq!(
        r.iter().map(|b| b.container).collect::<Vec<_>>(),
        ["WAV", "FLAC", "MPEG audio"]
    );
    for b in r {
        assert_eq!(b.backend, "Symphonia");
        assert_eq!(b.version, "0.6.1");
        assert_eq!(b.status, "experimental");
    }
    assert_eq!(r[0].profile, "PCM signed little-endian 24/32");
    assert_eq!(r[1].profile, "PCM24 native FLAC");
}

#[test]
fn stale_generation_never_returns_pcm_and_can_restart_with_new_generation() {
    let generation = AtomicU64::new(42);
    let bytes = fixture("pcm32-a.wav");
    assert_eq!(
        decode(bytes.clone(), Limits::default(), &generation, 41).unwrap_err(),
        DecodeError::Cancelled
    );
    generation.store(43, Ordering::Release);
    assert_eq!(
        decode(bytes.clone(), Limits::default(), &generation, 42).unwrap_err(),
        DecodeError::Cancelled
    );
    assert_eq!(
        decode(bytes, Limits::default(), &generation, 43)
            .unwrap()
            .frames,
        4417
    );
}

#[test]
fn byte_frame_rate_and_channel_budgets_reject_before_returning_pcm() {
    for limits in [
        Limits {
            max_bytes: 100,
            ..Limits::default()
        },
        Limits {
            max_frames: 4416,
            ..Limits::default()
        },
        Limits {
            max_rate: 44099,
            ..Limits::default()
        },
        Limits {
            max_channels: 1,
            ..Limits::default()
        },
        Limits {
            max_frames: 0,
            ..Limits::default()
        },
    ] {
        assert_eq!(
            decode(fixture("pcm24-a.flac"), limits, &AtomicU64::new(1), 1).unwrap_err(),
            DecodeError::Limit
        );
    }
}

#[test]
fn invalid_and_truncated_streams_are_errors_not_partial_success() {
    for bytes in [Vec::new(), b"not audio".to_vec()] {
        assert!(decode(bytes, Limits::default(), &AtomicU64::new(1), 1).is_err());
    }
    for name in ["pcm24-a.wav", "pcm24-a.flac", "tone-cbr-a.mp3"] {
        let mut bytes = fixture(name);
        bytes.truncate(bytes.len() / 2);
        assert!(
            decode(bytes, Limits::default(), &AtomicU64::new(1), 1).is_err(),
            "{name}"
        );
    }
}

#[test]
fn mp3_without_trustworthy_trim_is_rejected_instead_of_partial_success() {
    for name in ["tone-no-xing.mp3", "tone-vbr-long-no-xing.mp3"] {
        let bytes = fixture(name);
        assert_eq!(
            decode(bytes.clone(), Limits::default(), &AtomicU64::new(1), 1).unwrap_err(),
            DecodeError::Unsupported,
            "{name}"
        );
        for cut in [bytes.len() / 2, bytes.len() - 1] {
            assert!(
                decode(
                    bytes[..cut].to_vec(),
                    Limits::default(),
                    &AtomicU64::new(1),
                    1
                )
                .is_err(),
                "{name} cut={cut}"
            );
        }
    }
}

#[test]
fn wav32_lsb_extremes_and_negative_one_survive_without_float_conversion() {
    let d = decode(
        fixture("pcm32-a.wav"),
        Limits::default(),
        &AtomicU64::new(1),
        1,
    )
    .unwrap();
    let Samples::Integer(s) = d.samples else {
        panic!("integer path required")
    };
    assert_eq!(
        &s[..8],
        &[i32::MIN, i32::MIN, i32::MAX, i32::MAX, -1, -1, 1, 1]
    );
}

struct Tripwire {
    inner: Cursor<Vec<u8>>,
    reads: Arc<AtomicUsize>,
    trip_at: usize,
    generation: Arc<AtomicU64>,
}

impl Read for Tripwire {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Small reads make packet boundaries observable and deterministic.
        let n = self.reads.fetch_add(1, Ordering::SeqCst) + 1;
        if n == self.trip_at {
            self.generation.fetch_add(1, Ordering::Release);
        }
        let cap = buf.len().min(512);
        self.inner.read(&mut buf[..cap])
    }
}
impl Seek for Tripwire {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }
}
impl MediaSource for Tripwire {
    fn is_seekable(&self) -> bool {
        true
    }
    fn byte_len(&self) -> Option<u64> {
        Some(self.inner.get_ref().len() as u64)
    }
}

fn run_tripwire(
    name: &str,
    trip_at: usize,
) -> (Result<vynamp_decode::Decoded, DecodeError>, usize) {
    let generation = Arc::new(AtomicU64::new(5));
    let reads = Arc::new(AtomicUsize::new(0));
    let source = Tripwire {
        inner: Cursor::new(fixture(name)),
        reads: reads.clone(),
        trip_at,
        generation: generation.clone(),
    };
    let r = decode_source(Box::new(source), Limits::default(), &generation, 5);
    (r, reads.load(Ordering::SeqCst))
}

#[test]
fn generation_change_during_decode_cancels_at_every_read_and_stops_early() {
    for name in ["pcm24-a.wav", "pcm24-a.flac", "tone-cbr-a.mp3"] {
        let (ok, total) = run_tripwire(name, usize::MAX);
        assert_eq!(ok.unwrap().frames, 4417, "{name}");
        assert!(total >= 8, "{name}: need many reads, got {total}");
        for trip in 1..=total {
            let (r, reads) = run_tripwire(name, trip);
            assert_eq!(
                r.map(|d| d.frames).unwrap_err(),
                DecodeError::Cancelled,
                "{name} trip={trip}/{total}"
            );
            // A change in the first half must stop the read loop early; later
            // trips may legitimately have consumed the whole input already.
            if trip <= total / 2 {
                assert!(
                    reads < total,
                    "{name} trip={trip}: kept reading ({reads}/{total})"
                );
            }
        }
    }
}

#[test]
fn generation_change_on_the_last_read_is_never_returned_as_pcm() {
    // For at least one container the last read happens when the packet reader
    // hits end of stream, so only the pre-return check can observe the change.
    for name in ["pcm24-a.wav", "pcm24-a.flac", "tone-cbr-a.mp3"] {
        let (ok, total) = run_tripwire(name, usize::MAX);
        ok.unwrap();
        let (r, reads) = run_tripwire(name, total);
        assert_eq!(reads, total, "{name}");
        assert_eq!(r.map(|d| d.frames), Err(DecodeError::Cancelled), "{name}");
    }
}

#[test]
fn running_frame_total_is_limited_when_the_container_declares_no_length() {
    // STREAMINFO total_samples (36 bits at file bytes 21..26) = 0 means unknown, so
    // the early declared-length budget cannot fire; only the per-packet running
    // total can stop a stream larger than max_frames.
    let mut bytes = fixture("pcm24-a.flac");
    bytes[21] &= 0xF0;
    bytes[22..26].fill(0);
    let unlimited = decode(bytes.clone(), Limits::default(), &AtomicU64::new(1), 1).unwrap();
    assert_eq!(unlimited.frames, 4417);
    let limits = Limits {
        max_frames: 4416,
        ..Limits::default()
    };
    assert_eq!(
        decode(bytes, limits, &AtomicU64::new(1), 1).unwrap_err(),
        DecodeError::Limit
    );
}
