//! Simulated device harness: a producer thread feeds a sample-index ramp into
//! the ring while a device thread calls [`Renderer::render`] once per period
//! inside an RT section. Every rendered buffer is checked against the ramp,
//! so any lost, repeated or reordered sample is counted.
use crate::{
    callback::Renderer,
    clock::{self, ClockSnapshot},
    ring,
    rt::{rt_section, thread_rt_alloc_events},
    stats::{CallbackStats, StatsReport},
};
use std::time::{Duration, Instant};

/// Simulation parameters. Ramp values must stay exact in `f32`, so
/// `frames * channels` must stay below 2^24.
#[derive(Clone, Debug)]
pub struct SimConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub period_frames: usize,
    /// Latency the simulated device reports on every callback.
    pub latency_frames: u64,
    /// Ring capacity in periods.
    pub ring_periods: usize,
    /// Content frames the producer emits before finishing.
    pub frames: u64,
    /// After producing this many frames, the producer pauses for the duration.
    pub stall: Option<(u64, Duration)>,
}

/// Outcome of [`run`].
#[derive(Clone, Debug)]
pub struct SimReport {
    pub stats: StatsReport,
    pub clock: ClockSnapshot,
    /// Samples that broke the ramp (lost, repeated or non-zero silence).
    pub ramp_errors: u64,
    /// Heap events inside RT sections during the run; meaningful only when
    /// [`crate::rt::CountingAllocator`] is the global allocator.
    pub rt_alloc_events: u64,
}

/// Runs the simulation until all content is played and drained.
pub fn run(cfg: SimConfig) -> SimReport {
    let ch = cfg.channels as usize;
    let total = cfg.frames * ch as u64;
    assert!(total < 1 << 24, "ramp would lose f32 precision");
    let period = Duration::from_secs_f64(cfg.period_frames as f64 / cfg.sample_rate as f64);
    let (mut tx, rx) = ring::channel(cfg.ring_periods * cfg.period_frames * ch);
    let (cw, cr) = clock::clock();
    let stats = CallbackStats::new(period.as_nanos() as u64);
    let mut renderer = Renderer::new(rx, cw, stats.clone(), cfg.channels);

    let prefill = (2 * cfg.period_frames * ch).min(total as usize);
    let producer = std::thread::spawn(move || {
        let mut chunk: Vec<f32> = Vec::with_capacity(256);
        let mut next = 0u64;
        let mut stalled = false;
        while next < total {
            if let Some((at, pause)) = cfg.stall
                && !stalled
                && next >= at * ch as u64
            {
                stalled = true;
                std::thread::sleep(pause);
            }
            let len = 256.min(total - next);
            chunk.clear();
            chunk.extend((next..next + len).map(|v| (v + 1) as f32));
            let mut sent = 0;
            while sent < chunk.len() {
                sent += tx.push(&chunk[sent..]);
                if sent < chunk.len() {
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
            next += len;
        }
        tx.finish();
    });

    let mut out = vec![0.0f32; cfg.period_frames * ch];
    let mut expected = 1u64;
    let mut ramp_errors = 0u64;
    let mut alloc_events = 0u64;
    let started = Instant::now();
    while renderer.buffered_samples() < prefill {
        std::thread::sleep(Duration::from_millis(1));
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "producer never started"
        );
    }
    let mut deadline = Instant::now();
    loop {
        let before = thread_rt_alloc_events();
        let outcome = rt_section(|| renderer.render(&mut out, cfg.latency_frames));
        alloc_events += thread_rt_alloc_events() - before;
        let content = outcome.content_frames as usize * ch;
        for v in &out[..content] {
            if *v != expected as f32 {
                ramp_errors += 1;
            }
            expected = *v as u64 + 1;
        }
        ramp_errors += out[content..].iter().filter(|v| **v != 0.0).count() as u64;
        if outcome.silence_frames > 0 && !outcome.underrun {
            break;
        }
        deadline += period;
        if let Some(wait) = deadline.checked_duration_since(Instant::now()) {
            std::thread::sleep(wait);
        }
        assert!(
            started.elapsed() < Duration::from_secs(60),
            "simulation hung"
        );
    }
    producer.join().expect("producer panicked");
    SimReport {
        stats: stats.report(),
        clock: cr.snapshot(),
        ramp_errors,
        rt_alloc_events: alloc_events,
    }
}
