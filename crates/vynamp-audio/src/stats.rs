//! Lock-free callback statistics: duration histogram, underrun and drain
//! counters. Recording is wait-free; `report` runs outside the callback.
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

const BUCKETS: usize = 64;

/// Counters shared between the callback (writer) and observers.
pub struct CallbackStats {
    period_ns: u64,
    // Bucket b holds durations d with floor(log2(max(d, 1))) == b.
    histogram: [AtomicU64; BUCKETS],
    callbacks: AtomicU64,
    max_ns: AtomicU64,
    underruns: AtomicU64,
    underrun_frames: AtomicU64,
    drained_frames: AtomicU64,
}

/// Point-in-time summary of [`CallbackStats`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StatsReport {
    pub callbacks: u64,
    pub underruns: u64,
    pub underrun_frames: u64,
    pub drained_frames: u64,
    pub max_ns: u64,
    /// Upper bound of the histogram bucket holding the 99.9th percentile.
    pub p999_ns: u64,
    /// Nominal callback period the durations are compared against.
    pub period_ns: u64,
}

impl CallbackStats {
    /// `period_ns` is the negotiated callback period, kept for the report.
    pub fn new(period_ns: u64) -> Arc<Self> {
        Arc::new(Self {
            period_ns,
            histogram: std::array::from_fn(|_| AtomicU64::new(0)),
            callbacks: AtomicU64::new(0),
            max_ns: AtomicU64::new(0),
            underruns: AtomicU64::new(0),
            underrun_frames: AtomicU64::new(0),
            drained_frames: AtomicU64::new(0),
        })
    }

    pub fn report(&self) -> StatsReport {
        let counts: [u64; BUCKETS] =
            std::array::from_fn(|b| self.histogram[b].load(Ordering::Relaxed));
        let total: u64 = counts.iter().sum();
        // Smallest bucket whose cumulative count reaches ceil(0.999 * total).
        let rank = (total * 999).div_ceil(1000);
        let mut seen = 0;
        let mut p999_ns = 0;
        for (b, c) in counts.iter().enumerate() {
            seen += c;
            if total > 0 && seen >= rank {
                p999_ns = if b == BUCKETS - 1 {
                    u64::MAX
                } else {
                    (1u64 << (b + 1)) - 1
                };
                break;
            }
        }
        StatsReport {
            callbacks: self.callbacks.load(Ordering::Relaxed),
            underruns: self.underruns.load(Ordering::Relaxed),
            underrun_frames: self.underrun_frames.load(Ordering::Relaxed),
            drained_frames: self.drained_frames.load(Ordering::Relaxed),
            max_ns: self.max_ns.load(Ordering::Relaxed),
            p999_ns,
            period_ns: self.period_ns,
        }
    }
}

// RT-BEGIN: recording runs inside the audio callback.
impl CallbackStats {
    /// Records one callback duration.
    pub fn record(&self, duration_ns: u64) {
        let bucket = (63 - duration_ns.max(1).leading_zeros()) as usize;
        self.histogram[bucket].fetch_add(1, Ordering::Relaxed);
        self.callbacks.fetch_add(1, Ordering::Relaxed);
        self.max_ns.fetch_max(duration_ns, Ordering::Relaxed);
    }

    /// Records one underrun that zero-filled `frames` frames.
    pub fn record_underrun(&self, frames: u64) {
        self.underruns.fetch_add(1, Ordering::Relaxed);
        self.underrun_frames.fetch_add(frames, Ordering::Relaxed);
    }

    /// Records silence written after end of stream (not an underrun).
    pub fn record_drained(&self, frames: u64) {
        self.drained_frames.fetch_add(frames, Ordering::Relaxed);
    }
}
// RT-END
