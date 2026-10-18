use std::time::Duration;
use vynamp_audio::{callback::Renderer, clock, ring, stats::CallbackStats};
use vynamp_output::convert::{ConvertingRenderer, DeviceSample};
use vynamp_output::latency::frames_between;

#[test]
fn i16_conversion_clamps_and_rounds() {
    assert_eq!(i16::from_f32(0.0), 0);
    assert_eq!(i16::from_f32(1.0), 32767);
    assert_eq!(i16::from_f32(-1.0), -32767);
    assert_eq!(i16::from_f32(2.0), 32767, "clamped");
    assert_eq!(i16::from_f32(-3.0), -32767, "clamped");
    assert_eq!(i16::from_f32(0.5), 16384, "rounded half away from zero");
    assert_eq!(i16::from_f32(f32::NAN), 0, "NaN never reaches the device");
}

#[test]
fn i32_conversion_full_scale() {
    assert_eq!(i32::from_f32(1.0), i32::MAX);
    assert_eq!(i32::from_f32(-1.0), -i32::MAX);
    assert_eq!(i32::from_f32(0.25), 536_870_912);
    assert_eq!(f32::from_f32(0.25), 0.25);
}

#[test]
fn latency_frames_from_timestamps() {
    let cb = Duration::from_millis(1000);
    let pb = Duration::from_millis(1020);
    assert_eq!(frames_between(cb, pb, 48_000), 960);
    assert_eq!(
        frames_between(cb, cb + Duration::from_nanos(10_417), 96_000),
        1
    );
}

#[test]
fn negative_latency_is_zero() {
    assert_eq!(
        frames_between(Duration::from_millis(5), Duration::from_millis(4), 48_000),
        0
    );
}

fn converting(
    capacity: usize,
    scratch_frames: usize,
) -> (ring::Producer, ConvertingRenderer, clock::ClockReader) {
    let (tx, rx) = ring::channel(capacity);
    let (cw, cr) = clock::clock();
    let r = Renderer::new(rx, cw, CallbackStats::new(1), 2);
    (tx, ConvertingRenderer::new(r, 2, scratch_frames), cr)
}

#[test]
fn oversized_device_buffer_is_rendered_in_chunks() {
    let (mut tx, mut c, clock) = converting(64, 2);
    let src: Vec<f32> = (1..=10).map(|v| v as f32 / 16.0).collect();
    tx.push(&src);
    let mut dst = [i16::MIN; 12];
    c.render_into(&mut dst, 0);
    let expected: Vec<i16> = src.iter().map(|v| i16::from_f32(*v)).collect();
    assert_eq!(&dst[..10], &expected[..], "all content converted in order");
    assert_eq!(&dst[10..], &[0, 0], "underrun tail is silence");
    assert_eq!(clock.snapshot().content_frames, 5);
}

#[test]
fn f32_device_buffer_passes_samples_unchanged() {
    let (mut tx, mut c, _clock) = converting(64, 1);
    tx.push(&[0.5, -0.5, 0.25, -0.25]);
    let mut dst = [f32::NAN; 4];
    c.render_into(&mut dst, 0);
    assert_eq!(dst, [0.5, -0.5, 0.25, -0.25]);
}
