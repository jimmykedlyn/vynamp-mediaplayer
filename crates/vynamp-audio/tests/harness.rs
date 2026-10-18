use std::time::Duration;
use vynamp_audio::harness::{self, SimConfig};
use vynamp_audio::rt::CountingAllocator;

#[global_allocator]
static ALLOC: CountingAllocator = CountingAllocator;

fn config() -> SimConfig {
    SimConfig {
        sample_rate: 48_000,
        channels: 2,
        period_frames: 480,
        latency_frames: 960,
        ring_periods: 8,
        frames: 48_000,
        stall: None,
    }
}

#[test]
fn steady_run_has_no_underrun() {
    let r = harness::run(config());
    assert_eq!(r.ramp_errors, 0, "loss or replay detected");
    assert_eq!(r.stats.underruns, 0);
    assert_eq!(
        r.clock.content_frames, 48_000,
        "all produced content played"
    );
    assert_eq!(r.clock.presented_frames, 48_000 - 960);
    assert_eq!(r.rt_alloc_events, 0);
    assert!(r.stats.drained_frames > 0 && r.stats.drained_frames <= 480);
}

#[test]
fn stalled_producer_underruns_without_replay() {
    let r = harness::run(SimConfig {
        stall: Some((9_600, Duration::from_millis(200))),
        ..config()
    });
    // Ring holds 8 periods (80 ms); a 200 ms stall must starve the device.
    assert!(r.stats.underruns >= 1, "stall must starve the device");
    assert!(r.stats.underrun_frames >= 480);
    assert_eq!(r.ramp_errors, 0, "underrun must not replay or skip samples");
    assert_eq!(
        r.clock.content_frames, 48_000,
        "no sample lost across underrun"
    );
    assert_eq!(
        r.clock.silence_frames,
        r.stats.underrun_frames + r.stats.drained_frames
    );
    assert_eq!(r.rt_alloc_events, 0);
}
