use std::sync::Arc;
use vynamp_audio::{callback::Renderer, clock, ring, stats::CallbackStats};

fn ramp(start: usize, len: usize) -> Vec<f32> {
    (start..start + len).map(|v| v as f32 + 1.0).collect()
}

fn setup(
    capacity: usize,
) -> (
    ring::Producer,
    Renderer,
    clock::ClockReader,
    Arc<CallbackStats>,
) {
    let (tx, rx) = ring::channel(capacity);
    let (cw, cr) = clock::clock();
    let stats = CallbackStats::new(1_000_000);
    (tx, Renderer::new(rx, cw, stats.clone(), 2), cr, stats)
}

#[test]
fn underrun_zero_fills_without_replay() {
    let (mut tx, mut r, clock, stats) = setup(64);
    tx.push(&ramp(0, 6));
    let mut out = [f32::NAN; 8];
    let o = r.render(&mut out, 0);
    assert_eq!(&out[..6], &ramp(0, 6)[..]);
    assert_eq!(&out[6..], &[0.0, 0.0], "missing frame is silence");
    assert_eq!(
        (o.content_frames, o.silence_frames, o.underrun),
        (3, 1, true)
    );

    // Starved again: whole block is zeros, nothing previously played is repeated.
    let mut out = [f32::NAN; 4];
    r.render(&mut out, 0);
    assert_eq!(out, [0.0; 4]);

    tx.push(&ramp(6, 4));
    let mut out = [f32::NAN; 4];
    let o = r.render(&mut out, 0);
    assert_eq!(
        &out[..],
        &ramp(6, 4)[..],
        "resumes at the next unplayed sample"
    );
    assert!(!o.underrun);

    let s = stats.report();
    assert_eq!((s.underruns, s.underrun_frames), (2, 3));
    assert_eq!(clock.snapshot().content_frames, 5);
    assert_eq!(clock.snapshot().silence_frames, 3);
}

#[test]
fn drain_after_finish_is_not_underrun() {
    let (mut tx, mut r, _clock, stats) = setup(64);
    tx.push(&ramp(0, 2));
    tx.finish();
    let mut out = [f32::NAN; 4];
    let o = r.render(&mut out, 0);
    assert_eq!(&out[..], &[1.0, 2.0, 0.0, 0.0]);
    assert!(!o.underrun);
    let s = stats.report();
    assert_eq!((s.underruns, s.drained_frames), (0, 1));
}

#[test]
fn partial_frame_tail_is_zeroed() {
    let (mut tx, mut r, clock, _stats) = setup(64);
    tx.push(&ramp(0, 8));
    let mut out = [f32::NAN; 7];
    let o = r.render(&mut out, 0);
    assert_eq!(&out[..6], &ramp(0, 6)[..]);
    assert_eq!(
        out[6], 0.0,
        "incomplete trailing frame never takes a sample"
    );
    assert_eq!(o.content_frames, 3);
    let mut out = [f32::NAN; 2];
    r.render(&mut out, 0);
    assert_eq!(&out[..], &ramp(6, 2)[..], "channel alignment preserved");
    assert_eq!(clock.snapshot().content_frames, 4);
}

#[test]
fn partial_frame_in_ring_is_not_consumed() {
    let (mut tx, mut r, _clock, _stats) = setup(64);
    tx.push(&ramp(0, 3)); // one and a half stereo frames
    let mut out = [f32::NAN; 4];
    let o = r.render(&mut out, 0);
    assert_eq!(&out[..], &[1.0, 2.0, 0.0, 0.0]);
    assert_eq!(o.content_frames, 1);
    tx.push(&ramp(3, 1));
    let mut out = [f32::NAN; 2];
    r.render(&mut out, 0);
    assert_eq!(&out[..], &[3.0, 4.0]);
}

#[test]
fn render_updates_clock_with_device_latency() {
    let (mut tx, mut r, clock, stats) = setup(4096);
    tx.push(&vec![0.25; 1920]);
    let mut out = vec![0.0; 1920];
    r.render(&mut out, 480);
    let s = clock.snapshot();
    assert_eq!(
        (s.content_frames, s.latency_frames, s.presented_frames),
        (960, 480, 480)
    );
    assert_eq!(stats.report().callbacks, 1);
}
