//! Allocator instrumentation proof: the render path performs zero heap
//! (de)allocations. Runs in its own test binary so the counting allocator is
//! the global allocator only here.
use vynamp_audio::{
    callback::Renderer,
    clock, ring,
    rt::{CountingAllocator, rt_alloc_events, rt_section, thread_rt_alloc_events},
    stats::CallbackStats,
};

#[global_allocator]
static ALLOC: CountingAllocator = CountingAllocator;

#[test]
fn counter_detects_allocation_inside_section() {
    let before = rt_alloc_events();
    rt_section(|| {
        let v = std::hint::black_box(vec![1u8; 32]);
        drop(v);
    });
    assert!(
        rt_alloc_events() >= before + 2,
        "alloc + dealloc must be counted"
    );
}

#[test]
fn section_marker_nests_and_unwinds() {
    use vynamp_audio::rt::in_rt_section;
    assert!(!in_rt_section());
    rt_section(|| {
        rt_section(|| assert!(in_rt_section()));
        assert!(in_rt_section(), "inner exit keeps outer section");
    });
    assert!(!in_rt_section());
    let _ = std::panic::catch_unwind(|| rt_section(|| panic!("unwind")));
    assert!(
        !in_rt_section(),
        "panic inside a section must not leak the marker"
    );
}

#[test]
fn render_allocates_nothing() {
    let (mut tx, rx) = ring::channel(8192);
    let (cw, cr) = clock::clock();
    let stats = CallbackStats::new(10_000_000);
    let mut r = Renderer::new(rx, cw, stats.clone(), 2);
    let block = vec![0.1f32; 512];
    let mut out = vec![0.0f32; 512];
    let mut counted = 0;
    for i in 0..10_000u32 {
        // Every third callback starves to exercise the underrun path too.
        if i % 3 != 0 {
            tx.push(&block);
        }
        let before = thread_rt_alloc_events();
        rt_section(|| {
            r.render(&mut out, 256);
        });
        counted += thread_rt_alloc_events() - before;
    }
    tx.finish();
    rt_section(|| {
        r.render(&mut out, 256);
    });
    assert_eq!(counted, 0, "render path allocated");
    assert!(stats.report().underruns > 0, "underrun path exercised");
    assert!(cr.snapshot().content_frames > 0);
}

#[test]
fn thread_counter_ignores_other_threads_sections() {
    // Regression for CI run 39156053036: the process-wide counter picked up
    // a concurrent test's alloc+dealloc. Measurements of one callback thread
    // must use the per-thread counter.
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = stop.clone();
    let noisy = std::thread::spawn(move || {
        while !flag.load(std::sync::atomic::Ordering::Relaxed) {
            rt_section(|| drop(std::hint::black_box(vec![0u8; 16])));
        }
    });
    let global_before = rt_alloc_events();
    while rt_alloc_events() == global_before {
        std::thread::yield_now();
    }
    let mine = thread_rt_alloc_events();
    for _ in 0..1_000 {
        rt_section(|| std::hint::black_box(1u64 + 1));
    }
    let leaked = thread_rt_alloc_events() - mine;
    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    noisy.join().unwrap();
    assert!(
        rt_alloc_events() > global_before,
        "other thread counted globally"
    );
    assert_eq!(leaked, 0, "per-thread counter must ignore other threads");
}
