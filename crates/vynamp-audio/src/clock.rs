//! Output clock published by the audio callback.
//!
//! The callback is the only writer and never waits: it publishes a snapshot
//! through a sequence lock. Readers (control/UI threads) retry while a write
//! is in progress. Presented position is content frames consumed by earlier device
//! callbacks minus the latency the device reports for the current one; silence written during an
//! underrun or drain is counted separately and never advances the position.
//!
//! Known limitation (correction deferred to E01-05, drain): the estimate
//! assumes the device queue holds only content. Silence queued by an underrun
//! or drain is also inside the reported latency, so `presented` is biased low
//! by at most `latency_frames`: after an underrun it lags until content flows
//! again, and during drain it settles at `content_frames - latency_frames`
//! instead of reaching `content_frames`. Characterized by
//! `presented_bias_with_queued_silence_is_bounded_by_latency`.
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering, fence};

/// Consistent view of the output clock, in frames.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClockSnapshot {
    /// Content frames handed to the device since start.
    pub content_frames: u64,
    /// Silent frames written for underrun or drain.
    pub silence_frames: u64,
    /// Device latency reported at the last callback.
    pub latency_frames: u64,
    /// Content frames played at the last callback instant: content written
    /// before it minus device latency. Monotonic; never above `content_frames`.
    pub presented_frames: u64,
    /// Render chunks published. The output adapter renders once per scratch
    /// chunk, so a device callback larger than the scratch buffer counts more
    /// than once.
    pub callbacks: u64,
}

#[derive(Default)]
struct Shared {
    seq: AtomicU64,
    content: AtomicU64,
    silence: AtomicU64,
    latency: AtomicU64,
    presented: AtomicU64,
    callbacks: AtomicU64,
}

/// Writer half, owned by the audio callback.
pub struct ClockWriter {
    shared: Arc<Shared>,
    local: ClockSnapshot,
}

/// Reader half; cheap to clone.
#[derive(Clone)]
pub struct ClockReader {
    shared: Arc<Shared>,
}

/// Creates a clock at position zero.
pub fn clock() -> (ClockWriter, ClockReader) {
    let shared = Arc::new(Shared::default());
    (
        ClockWriter {
            shared: shared.clone(),
            local: ClockSnapshot::default(),
        },
        ClockReader { shared },
    )
}

// RT-BEGIN: publish runs inside the audio callback.
impl ClockWriter {
    /// Records one callback: content and silence frames written to the device
    /// and the device-reported delay between this callback and playback of the
    /// buffer's first frame.
    pub fn publish(&mut self, content_delta: u64, silence_delta: u64, latency_frames: u64) {
        let l = &mut self.local;
        // `latency_frames` is the delay until the first frame of this buffer
        // plays, so it is measured against content written before it.
        let estimate = l.content_frames.saturating_sub(latency_frames);
        l.content_frames += content_delta;
        l.silence_frames += silence_delta;
        l.latency_frames = latency_frames;
        l.callbacks += 1;
        l.presented_frames = l.presented_frames.max(estimate);

        let s = &self.shared;
        let seq = s.seq.load(Ordering::Relaxed);
        s.seq.store(seq + 1, Ordering::Relaxed);
        fence(Ordering::Release);
        s.content.store(l.content_frames, Ordering::Relaxed);
        s.silence.store(l.silence_frames, Ordering::Relaxed);
        s.latency.store(l.latency_frames, Ordering::Relaxed);
        s.presented.store(l.presented_frames, Ordering::Relaxed);
        s.callbacks.store(l.callbacks, Ordering::Relaxed);
        s.seq.store(seq + 2, Ordering::Release);
    }
}
// RT-END

impl ClockReader {
    /// Returns a consistent snapshot, retrying while the callback writes.
    pub fn snapshot(&self) -> ClockSnapshot {
        let s = &self.shared;
        loop {
            let before = s.seq.load(Ordering::Acquire);
            if before % 2 == 1 {
                std::hint::spin_loop();
                continue;
            }
            let snap = ClockSnapshot {
                content_frames: s.content.load(Ordering::Relaxed),
                silence_frames: s.silence.load(Ordering::Relaxed),
                latency_frames: s.latency.load(Ordering::Relaxed),
                presented_frames: s.presented.load(Ordering::Relaxed),
                callbacks: s.callbacks.load(Ordering::Relaxed),
            };
            fence(Ordering::Acquire);
            if s.seq.load(Ordering::Relaxed) == before {
                return snap;
            }
        }
    }
}
