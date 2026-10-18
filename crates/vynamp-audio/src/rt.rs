//! Allocation instrumentation for the real-time path.
//!
//! [`CountingAllocator`] wraps the system allocator and counts every
//! allocation, reallocation and deallocation made by a thread while it is
//! inside [`rt_section`]. Install it as `#[global_allocator]` only in tests
//! and probes; the library never installs it.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};

thread_local! {
    // const-initialized and without Drop: reading it never allocates.
    static DEPTH: Cell<u32> = const { Cell::new(0) };
    static THREAD_EVENTS: Cell<u64> = const { Cell::new(0) };
}

static EVENTS: AtomicU64 = AtomicU64::new(0);

/// Global allocator wrapper that counts heap events inside RT sections.
pub struct CountingAllocator;

fn note() {
    if in_rt_section() {
        EVENTS.fetch_add(1, Ordering::Relaxed);
        let _ = THREAD_EVENTS.try_with(|n| n.set(n.get() + 1));
    }
}

// SAFETY: every call is forwarded unchanged to `System`.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        note();
        unsafe { System.alloc(layout) }
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        note();
        unsafe { System.alloc_zeroed(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        note();
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        note();
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

struct Exit;

impl Drop for Exit {
    fn drop(&mut self) {
        let _ = DEPTH.try_with(|d| d.set(d.get() - 1));
    }
}

/// Runs `f` marked as real-time on this thread; heap events are counted.
pub fn rt_section<R>(f: impl FnOnce() -> R) -> R {
    let _ = DEPTH.try_with(|d| d.set(d.get() + 1));
    let _exit = Exit;
    f()
}

/// True while this thread is inside [`rt_section`].
pub fn in_rt_section() -> bool {
    DEPTH.try_with(|d| d.get() > 0).unwrap_or(false)
}

/// Heap events counted inside RT sections since process start, all threads.
/// Suits a probe whose only RT section is the device callback thread.
pub fn rt_alloc_events() -> u64 {
    EVENTS.load(Ordering::Relaxed)
}

/// Heap events counted inside RT sections on the calling thread only. Use
/// this when the measured callback runs on the measuring thread, so other
/// threads' sections (e.g. concurrent tests) cannot leak into the result.
pub fn thread_rt_alloc_events() -> u64 {
    THREAD_EVENTS.try_with(Cell::get).unwrap_or(0)
}
