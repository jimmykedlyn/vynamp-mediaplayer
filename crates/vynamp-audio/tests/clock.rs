use vynamp_audio::clock;

#[test]
fn presented_subtracts_latency() {
    let (mut w, r) = clock::clock();
    w.publish(4800, 0, 960);
    let s = r.snapshot();
    assert_eq!(s.content_frames, 4800);
    assert_eq!(s.latency_frames, 960);
    assert_eq!(s.presented_frames, 3840);
    assert_eq!(s.callbacks, 1);
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
fn presented_is_monotonic_when_latency_drops() {
    let (mut w, r) = clock::clock();
    w.publish(1000, 0, 200); // presented 800
    w.publish(0, 0, 900); // latency grew: 100 < 800, must not regress
    assert_eq!(r.snapshot().presented_frames, 800);
    w.publish(500, 0, 100); // 1400
    assert_eq!(r.snapshot().presented_frames, 1400);
}

#[test]
fn concurrent_reader_sees_consistent_snapshot() {
    let (mut w, r) = clock::clock();
    let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = stop.clone();
    let reader = std::thread::spawn(move || {
        let mut reads = 0u64;
        let mut last = 0u64;
        while !flag.load(std::sync::atomic::Ordering::Relaxed) {
            let s = r.snapshot();
            // Writer always publishes content == 2 * callbacks and silence == callbacks.
            assert_eq!(s.content_frames, 2 * s.callbacks, "torn snapshot");
            assert_eq!(s.silence_frames, s.callbacks, "torn snapshot");
            assert!(s.presented_frames <= s.content_frames);
            assert!(s.presented_frames >= last, "presented regressed");
            last = s.presented_frames;
            reads += 1;
        }
        reads
    });
    for _ in 0..200_000 {
        w.publish(2, 1, 3);
    }
    stop.store(true, std::sync::atomic::Ordering::Relaxed);
    assert!(reader.join().unwrap() > 0);
}
