//! Device latency from backend timestamps.
use std::time::Duration;

/// Frames between the callback instant and the instant its first frame is
/// played, rounded to nearest. A playback instant before the callback (clock
/// skew in the backend) yields zero rather than a negative latency.
pub fn frames_between(callback: Duration, playback: Duration, sample_rate: u32) -> u64 {
    let ns = playback.saturating_sub(callback).as_nanos();
    ((ns * u128::from(sample_rate) + 500_000_000) / 1_000_000_000) as u64
}
