//! Shared-mode output on the host's default device through CPAL.
//!
//! Everything the callback touches (ring consumer, clock writer, statistics,
//! conversion scratch) is built here, before the stream starts. The callback
//! body runs inside [`rt_section`] so tests and probes using
//! `CountingAllocator` count any heap event made by Vynamp code in it. Heap
//! use by CPAL or the OS around the closure is outside that measurement.
use crate::{
    convert::{ConvertingRenderer, DeviceSample},
    latency::frames_between,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::{
    fmt,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};
use vynamp_audio::{
    callback::Renderer,
    clock::{self, ClockReader},
    ring::{self, Producer},
    rt::rt_section,
    stats::CallbackStats,
};

/// Parameters chosen before opening.
#[derive(Clone, Copy, Debug)]
pub struct OpenOptions {
    /// Ring capacity in frames.
    pub ring_frames: usize,
    /// Fixed device buffer request; `None` keeps the host default.
    pub buffer_frames: Option<u32>,
}

/// Configuration actually negotiated with the device.
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub host: String,
    pub device: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub sample_format: String,
    /// Buffer range the device advertises.
    pub supported_buffer: String,
    pub requested_buffer_frames: Option<u32>,
}

/// Failure to open or start the shared stream.
#[derive(Debug)]
pub struct OutputError(pub String);

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for OutputError {}

fn err(context: &str, e: impl fmt::Display) -> OutputError {
    OutputError(format!("{context}: {e}"))
}

/// A running shared-mode stream. Dropping it stops playback.
pub struct SharedOutput {
    _stream: cpal::Stream,
    pub info: DeviceInfo,
    pub clock: ClockReader,
    pub stats: Arc<CallbackStats>,
    /// Stream errors reported by the backend (device loss, xruns it reports).
    pub errors: Arc<AtomicU64>,
    /// Largest device buffer seen in a callback, in frames.
    pub max_callback_frames: Arc<AtomicU64>,
    /// Smallest device buffer seen in a callback, in frames.
    pub min_callback_frames: Arc<AtomicU64>,
    /// Whole device-callback durations (the renderer stats count each
    /// conversion chunk separately).
    pub device_callbacks: Arc<CallbackStats>,
}

/// Opens the default output device in shared mode and starts the stream.
/// Returns the stream and the producer half of its ring.
pub fn open(opts: OpenOptions) -> Result<(SharedOutput, Producer), OutputError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| OutputError("no default output device".into()))?;
    let supported = device
        .default_output_config()
        .map_err(|e| err("default output config", e))?;
    let mut config = supported.config();
    if let Some(frames) = opts.buffer_frames {
        config.buffer_size = cpal::BufferSize::Fixed(frames);
    }
    let rate = config.sample_rate;
    let channels = config.channels;
    let scratch_frames = match (opts.buffer_frames, supported.buffer_size()) {
        (Some(frames), _) => frames as usize,
        (None, cpal::SupportedBufferSize::Range { max, .. }) => (*max).min(16_384) as usize,
        (None, cpal::SupportedBufferSize::Unknown) => 8_192,
    };
    let info = DeviceInfo {
        host: host.id().name().to_string(),
        device: device.to_string(),
        sample_rate: rate,
        channels,
        sample_format: supported.sample_format().to_string(),
        supported_buffer: format!("{:?}", supported.buffer_size()),
        requested_buffer_frames: opts.buffer_frames,
    };

    let (producer, consumer) = ring::channel(opts.ring_frames * channels as usize);
    let (clock_writer, clock) = clock::clock();
    let period_ns = opts
        .buffer_frames
        .map_or(0, |f| u64::from(f) * 1_000_000_000 / u64::from(rate));
    let stats = CallbackStats::new(period_ns);
    let renderer = Renderer::new(consumer, clock_writer, stats.clone(), channels);
    let conv = ConvertingRenderer::new(renderer, channels, scratch_frames);
    let errors = Arc::new(AtomicU64::new(0));
    let max_callback_frames = Arc::new(AtomicU64::new(0));
    let min_callback_frames = Arc::new(AtomicU64::new(u64::MAX));
    let device_callbacks = CallbackStats::new(period_ns);
    let parts = Parts {
        conv,
        rate,
        channels,
        errors: errors.clone(),
        max_callback_frames: max_callback_frames.clone(),
        min_callback_frames: min_callback_frames.clone(),
        device_callbacks: device_callbacks.clone(),
    };

    use cpal::SampleFormat as F;
    let stream = match supported.sample_format() {
        F::F32 => build::<f32>(&device, &config, parts),
        F::I16 => build::<i16>(&device, &config, parts),
        F::I32 => build::<i32>(&device, &config, parts),
        other => Err(OutputError(format!("unsupported device format {other}"))),
    }?;
    stream.play().map_err(|e| err("play", e))?;
    Ok((
        SharedOutput {
            _stream: stream,
            info,
            clock,
            stats,
            errors,
            max_callback_frames,
            min_callback_frames,
            device_callbacks,
        },
        producer,
    ))
}

struct Parts {
    conv: ConvertingRenderer,
    rate: u32,
    channels: u16,
    errors: Arc<AtomicU64>,
    max_callback_frames: Arc<AtomicU64>,
    min_callback_frames: Arc<AtomicU64>,
    device_callbacks: Arc<CallbackStats>,
}

fn build<T: DeviceSample + cpal::SizedSample>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    parts: Parts,
) -> Result<cpal::Stream, OutputError> {
    let Parts {
        mut conv,
        rate,
        channels,
        errors,
        max_callback_frames,
        min_callback_frames,
        device_callbacks,
    } = parts;
    let channels = u64::from(channels);
    device
        .build_output_stream::<T, _, _>(
            *config,
            move |data: &mut [T], info: &cpal::OutputCallbackInfo| {
                // RT-BEGIN: device callback body.
                rt_section(|| {
                    let start = Instant::now();
                    let ts = info.timestamp();
                    let latency = ts.playback.duration_since(ts.callback);
                    let latency_frames = frames_between(Duration::ZERO, latency, rate);
                    let frames = data.len() as u64 / channels;
                    max_callback_frames.fetch_max(frames, Ordering::Relaxed);
                    min_callback_frames.fetch_min(frames, Ordering::Relaxed);
                    conv.render_into(data, latency_frames);
                    device_callbacks.record(start.elapsed().as_nanos() as u64);
                });
                // RT-END
            },
            move |_e| {
                errors.fetch_add(1, Ordering::Relaxed);
            },
            None,
        )
        .map_err(|e| err("build output stream", e))
}
