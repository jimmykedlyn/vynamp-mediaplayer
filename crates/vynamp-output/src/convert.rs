//! Conversion from the engine's `f32` frames to the device sample format,
//! through a scratch buffer allocated before the stream starts.
use vynamp_audio::callback::Renderer;

/// Device sample formats the shared adapter accepts.
pub trait DeviceSample: Copy {
    /// Converts a nominal [-1, 1] sample; out of range clamps, NaN is silence.
    fn from_f32(x: f32) -> Self;
}

impl DeviceSample for f32 {
    fn from_f32(x: f32) -> Self {
        x
    }
}

impl DeviceSample for i16 {
    fn from_f32(x: f32) -> Self {
        if x.is_nan() {
            return 0;
        }
        (x.clamp(-1.0, 1.0) * i16::MAX as f32).round() as i16
    }
}

impl DeviceSample for i32 {
    fn from_f32(x: f32) -> Self {
        if x.is_nan() {
            return 0;
        }
        (f64::from(x.clamp(-1.0, 1.0)) * f64::from(i32::MAX)).round() as i32
    }
}

/// Renderer plus pre-allocated `f32` scratch for non-`f32` devices.
pub struct ConvertingRenderer {
    renderer: Renderer,
    scratch: Box<[f32]>,
}

impl ConvertingRenderer {
    /// Allocates `scratch_frames * channels` samples; call before streaming.
    pub fn new(renderer: Renderer, channels: u16, scratch_frames: usize) -> Self {
        let len = scratch_frames.max(1) * channels.max(1) as usize;
        Self {
            renderer,
            scratch: vec![0.0; len].into_boxed_slice(),
        }
    }
}

// RT-BEGIN: render_into is the device callback body.
impl ConvertingRenderer {
    /// Fills `dst`. A device buffer larger than the scratch is rendered in
    /// frame-aligned chunks; each chunk reports the latency of its first frame.
    pub fn render_into<S: DeviceSample>(&mut self, dst: &mut [S], latency_frames: u64) {
        // Scratch length is a whole number of frames, so every chunk but the
        // last starts on a frame boundary.
        let mut offset_frames = 0u64;
        for chunk in dst.chunks_mut(self.scratch.len()) {
            let src = &mut self.scratch[..chunk.len()];
            let outcome = self.renderer.render(src, latency_frames + offset_frames);
            for (d, s) in chunk.iter_mut().zip(src.iter()) {
                *d = S::from_f32(*s);
            }
            offset_frames += outcome.content_frames + outcome.silence_frames;
        }
    }
}
// RT-END
