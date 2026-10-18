//! The conversion path allocates nothing inside the callback.
use vynamp_audio::{
    callback::Renderer,
    clock, ring,
    rt::{CountingAllocator, rt_section, thread_rt_alloc_events},
    stats::CallbackStats,
};
use vynamp_output::convert::ConvertingRenderer;

#[global_allocator]
static ALLOC: CountingAllocator = CountingAllocator;

#[test]
fn render_into_allocates_nothing() {
    let (mut tx, rx) = ring::channel(16_384);
    let (cw, _cr) = clock::clock();
    let mut c = ConvertingRenderer::new(Renderer::new(rx, cw, CallbackStats::new(1), 2), 2, 256);
    let block = vec![0.1f32; 1024];
    let mut i16_out = vec![0i16; 1024];
    let mut i32_out = vec![0i32; 1024];
    let mut events = 0;
    for i in 0..2_000 {
        if i % 4 != 0 {
            tx.push(&block);
        }
        let before = thread_rt_alloc_events();
        rt_section(|| {
            c.render_into(&mut i16_out, 512);
            c.render_into(&mut i32_out, 512);
        });
        events += thread_rt_alloc_events() - before;
    }
    assert_eq!(events, 0);
}
