use vynamp_audio::stats::CallbackStats;

#[test]
fn p999_bucket_bounds_true_value() {
    let stats = CallbackStats::new(10_000_000);
    for _ in 0..999 {
        stats.record(1_000);
    }
    stats.record(5_000_000);
    let r = stats.report();
    assert_eq!(r.callbacks, 1000);
    assert_eq!(r.max_ns, 5_000_000);
    // p99.9 of 1000 samples is the 999th value: 1000 ns, reported as its bucket's upper bound.
    assert!(
        r.p999_ns >= 1_000 && r.p999_ns < 2_048,
        "p999={}",
        r.p999_ns
    );
    assert_eq!(r.period_ns, 10_000_000);
}

#[test]
fn p999_reports_tail_when_tail_is_large_enough() {
    let stats = CallbackStats::new(1);
    for _ in 0..990 {
        stats.record(100);
    }
    for _ in 0..10 {
        stats.record(1_000_000);
    }
    let r = stats.report();
    assert!(r.p999_ns >= 1_000_000, "p999={}", r.p999_ns);
}

#[test]
fn underruns_and_drain_are_counted_separately() {
    let stats = CallbackStats::new(1);
    stats.record_underrun(64);
    stats.record_underrun(32);
    stats.record_drained(128);
    let r = stats.report();
    assert_eq!(r.underruns, 2);
    assert_eq!(r.underrun_frames, 96);
    assert_eq!(r.drained_frames, 128);
}
