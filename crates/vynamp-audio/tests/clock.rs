use vynamp_audio::clock;

#[test]
fn presented_subtracts_latency() {
    // Latency is reported for the first frame of the buffer being written, so
    // it applies to content consumed before this callback.
    let (mut w, r) = clock::clock();
    w.publish(4800, 0, 960);
    assert_eq!(
        r.snapshot().presented_frames,
        0,
        "nothing played before first callback"
    );
    w.publish(4800, 0, 960);
    let s = r.snapshot();
    assert_eq!(s.content_frames, 9600);
    assert_eq!(s.latency_frames, 960);
    assert_eq!(s.presented_frames, 3840);
    assert_eq!(s.callbacks, 2);
}

#[test]
fn silence_does_not_advance_presented() {
    let (mut w, r) = clock::clock();
    w.publish(1000, 0, 0);
    w.publish(0, 512, 0);
    let s = r.snapshot();
    assert_eq!(s.presented_frames, 1000);
    assert_eq!(s.silence_frames, 512);
    assert_eq!(s.callbacks, 2);
}

#[test]
fn latency_larger_than_content_clamps_to_zero() {
    let (mut w, r) = clock::clock();
    w.publish(100, 0, 960);
    assert_eq!(r.snapshot().presented_frames, 0);
}

#[test]
fn presented_is_monotonic_when_latency_grows() {
    let (mut w, r) = clock::clock();
    w.publish(1000, 0, 0);
    w.publish(0, 0, 200); // 1000 - 200
    assert_eq!(r.snapshot().presented_frames, 800);
    w.publish(0, 0, 900); // latency grew: 100 < 800, must not regress
    assert_eq!(r.snapshot().presented_frames, 800);
    w.publish(500, 0, 100); // 1000 - 100
    assert_eq!(r.snapshot().presented_frames, 900);
    w.publish(0, 0, 100); // 1500 - 100
    assert_eq!(r.snapshot().presented_frames, 1400);
}

#[test]
fn concurrent_reader_sees_consistent_snapshot() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
    // The writer keeps publishing until the reader has completed MIN_READS
    // (bounded by MAX_PUBLISHES), so the reader is guaranteed to run no matter
    // how the scheduler orders the threads.
    const MIN_READS: u64 = 1_000;
    const MAX_PUBLISHES: u64 = 50_000_000;
    let (mut w, r) = clock::clock();
    let stop = Arc::new(AtomicBool::new(false));
    let reads_done = Arc::new(AtomicU64::new(0));
    let (flag, progress) = (stop.clone(), reads_done.clone());
    let reader = std::thread::spawn(move || {
        let mut last = 0u64;
        while !flag.load(Ordering::Relaxed) {
            let s = r.snapshot();
            // Writer always publishes content == 2 * callbacks and silence == callbacks.
            assert_eq!(s.content_frames, 2 * s.callbacks, "torn snapshot");
            assert_eq!(s.silence_frames, s.callbacks, "torn snapshot");
            assert!(s.presented_frames <= s.content_frames);
            assert!(s.presented_frames >= last, "presented regressed");
            last = s.presented_frames;
            progress.fetch_add(1, Ordering::Release);
        }
    });
    let mut published = 0u64;
    while published < MAX_PUBLISHES
        && (published < 200_000 || reads_done.load(Ordering::Acquire) < MIN_READS)
    {
        w.publish(2, 1, 3);
        published += 1;
        if published.is_multiple_of(1024) {
            std::thread::yield_now();
        }
    }
    stop.store(true, Ordering::Relaxed);
    reader.join().unwrap();
    assert!(
        reads_done.load(Ordering::Acquire) >= MIN_READS,
        "reader starved: {} reads after {published} publishes",
        reads_done.load(Ordering::Acquire)
    );
}

/// Characterizes a known limitation (deferred to TASK-E01-05): `presented`
/// subtracts the full reported latency from content written, assuming the
/// device queue holds only content. Queued underrun/drain silence therefore
/// biases `presented` low by at most `latency_frames`.
#[test]
fn presented_bias_with_queued_silence_is_bounded_by_latency() {
    let latency = 1_000u64;
    // Steady state: bias-free.
    let (mut w, r) = clock::clock();
    for _ in 0..10 {
        w.publish(500, 0, latency);
    }
    assert_eq!(r.snapshot().presented_frames, 4_500 - latency); // 9 * 500 before last
    // Underrun: silence is queued; the device still plays earlier content, but
    // the estimate keeps subtracting the full latency, so it lags.
    for _ in 0..4 {
        w.publish(0, 500, latency);
    }
    let s = r.snapshot();
    assert_eq!(s.content_frames, 5_000);
    assert_eq!(s.presented_frames, 5_000 - latency);
    // Truth after the queue drains would be 5_000; the shortfall equals latency.
    assert!(s.content_frames - s.presented_frames <= latency);
    // Drain: after the final content, presented never reaches content_frames.
    for _ in 0..10 {
        w.publish(0, 500, latency);
    }
    let s = r.snapshot();
    assert_eq!(s.presented_frames, s.content_frames - latency);
}
