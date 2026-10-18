//! Audio callback renderer: moves whole frames from the ring to the device
//! buffer, zero-fills what is missing, and publishes clock and statistics.
use crate::{clock::ClockWriter, ring::Consumer, stats::CallbackStats};
use std::sync::Arc;
use std::time::Instant;

/// What one callback wrote.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderOutcome {
    pub content_frames: u64,
    pub silence_frames: u64,
    /// Silence was written while the stream had not finished.
    pub underrun: bool,
}

/// Owns everything the callback touches; built outside the callback.
pub struct Renderer {
    consumer: Consumer,
    clock: ClockWriter,
    stats: Arc<CallbackStats>,
    channels: usize,
}

impl Renderer {
    /// `channels` is the interleaved channel count of ring and device buffer.
    pub fn new(
        consumer: Consumer,
        clock: ClockWriter,
        stats: Arc<CallbackStats>,
        channels: u16,
    ) -> Self {
        assert!(channels > 0, "channel count must be non-zero");
        Self {
            consumer,
            clock,
            stats,
            channels: channels as usize,
        }
    }
}

// RT-BEGIN: render is the audio callback body.
impl Renderer {
    /// Fills `out` (interleaved) and records `latency_frames`, the delay the
    /// device reports between this callback and playback of its first frame.
    pub fn render(&mut self, out: &mut [f32], latency_frames: u64) -> RenderOutcome {
        let start = Instant::now();
        let frames = out.len() / self.channels;
        // Read `finished` first: once it is true every sample has been
        // published, so a short read afterwards is a drain, not an underrun.
        let finished = self.consumer.is_finished();
        let content = frames.min(self.consumer.available() / self.channels);
        let written = self.consumer.pop_into(&mut out[..content * self.channels]);
        out[written..].fill(0.0);

        let silence = (frames - content) as u64;
        let underrun = silence > 0 && !finished;
        if underrun {
            self.stats.record_underrun(silence);
        } else if silence > 0 {
            self.stats.record_drained(silence);
        }
        self.clock.publish(content as u64, silence, latency_frames);
        self.stats.record(start.elapsed().as_nanos() as u64);
        RenderOutcome {
            content_frames: content as u64,
            silence_frames: silence,
            underrun,
        }
    }
}
// RT-END
