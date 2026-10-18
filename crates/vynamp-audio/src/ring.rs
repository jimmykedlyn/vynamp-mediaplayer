//! Single-producer/single-consumer ring of interleaved `f32` samples.
//!
//! Storage is allocated once in [`channel`]. `head` (written by the consumer)
//! and `tail` (written by the producer) are monotonic `u64` sample counters (no wraparound in practice); the
//! slot of counter `n` is `n % capacity`. The producer publishes samples with
//! a Release store of `tail` after writing them, and the consumer frees slots
//! with a Release store of `head` after reading them, so each side only ever
//! touches slots the other side has handed over. Neither side waits.
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[repr(align(128))]
struct Padded<T>(T);

struct Shared {
    buf: Box<[UnsafeCell<f32>]>,
    head: Padded<AtomicU64>,
    tail: Padded<AtomicU64>,
    finished: AtomicBool,
}

// SAFETY: slots are accessed only by the side that currently owns them, as
// established by the head/tail Acquire/Release handoff described above, and
// `Producer`/`Consumer` are not `Clone`, so there is exactly one of each.
unsafe impl Sync for Shared {}

/// Writing half; owned by the decode/producer thread.
pub struct Producer {
    shared: Arc<Shared>,
}

/// Reading half; owned by the audio callback.
pub struct Consumer {
    shared: Arc<Shared>,
}

/// Allocates a ring holding `capacity` samples. This is the only allocation.
pub fn channel(capacity: usize) -> (Producer, Consumer) {
    assert!(capacity > 0, "ring capacity must be non-zero");
    let buf = (0..capacity).map(|_| UnsafeCell::new(0.0)).collect();
    let shared = Arc::new(Shared {
        buf,
        head: Padded(AtomicU64::new(0)),
        tail: Padded(AtomicU64::new(0)),
        finished: AtomicBool::new(false),
    });
    (
        Producer {
            shared: shared.clone(),
        },
        Consumer { shared },
    )
}

impl Producer {
    /// Free slots right now.
    pub fn free(&self) -> usize {
        let s = &self.shared;
        let tail = s.tail.0.load(Ordering::Relaxed);
        let head = s.head.0.load(Ordering::Acquire);
        s.buf.len() - (tail - head) as usize
    }

    /// Copies as many samples as fit and returns how many were written.
    /// Never overwrites unread samples.
    pub fn push(&mut self, samples: &[f32]) -> usize {
        let s = &self.shared;
        let tail = s.tail.0.load(Ordering::Relaxed);
        let n = samples.len().min(self.free());
        let cap = s.buf.len() as u64;
        for (i, v) in samples[..n].iter().enumerate() {
            // SAFETY: slots tail..tail+n are free (checked above) and only the
            // producer writes them until `tail` is published below.
            unsafe { *s.buf[((tail + i as u64) % cap) as usize].get() = *v };
        }
        s.tail.0.store(tail + n as u64, Ordering::Release);
        n
    }

    /// Marks end of stream. Samples already pushed stay readable.
    pub fn finish(&self) {
        self.shared.finished.store(true, Ordering::Release);
    }
}

// RT-BEGIN: consumer side runs inside the audio callback.
impl Consumer {
    /// Samples ready to read.
    pub fn available(&self) -> usize {
        let s = &self.shared;
        (s.tail.0.load(Ordering::Acquire) - s.head.0.load(Ordering::Relaxed)) as usize
    }

    /// True once the producer called `finish`.
    pub fn is_finished(&self) -> bool {
        self.shared.finished.load(Ordering::Acquire)
    }

    /// Moves up to `out.len()` samples into `out`; returns the count.
    pub fn pop_into(&mut self, out: &mut [f32]) -> usize {
        let s = &self.shared;
        let head = s.head.0.load(Ordering::Relaxed);
        let n = out.len().min(self.available());
        let cap = s.buf.len() as u64;
        for (i, slot) in out[..n].iter_mut().enumerate() {
            // SAFETY: slots head..head+n were published by the producer's
            // Release store of `tail`, observed by `available`'s Acquire load.
            *slot = unsafe { *s.buf[((head + i as u64) % cap) as usize].get() };
        }
        s.head.0.store(head + n as u64, Ordering::Release);
        n
    }
}
// RT-END
