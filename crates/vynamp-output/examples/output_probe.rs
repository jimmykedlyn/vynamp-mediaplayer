//! TASK-E01-02 hardware probe: plays a generated signal on the default output
//! device in shared mode and records callback allocations, duration versus
//! period, underruns, device latency and the output clock as JSON.
//!
//! cargo run -p vynamp-output --features cpal --example output_probe --release -- \
//!     --seconds 30 [--amplitude 0.0] [--buffer-frames N] [--stall-at 10 --stall-ms 300] \
//!     [--out probe.json]
//!
//! The default amplitude 0.0 sends real content frames of digital silence, so
//! the probe is inaudible; pass e.g. 0.05 for a quiet 440 Hz tone.
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};
use vynamp_audio::rt::{CountingAllocator, rt_alloc_events};
use vynamp_output::shared::{self, OpenOptions};

#[global_allocator]
static ALLOC: CountingAllocator = CountingAllocator;

struct Args {
    seconds: f64,
    amplitude: f32,
    buffer_frames: Option<u32>,
    stall_at: Option<f64>,
    stall_ms: u64,
    out: Option<String>,
}

fn parse() -> Args {
    let mut a = Args {
        seconds: 30.0,
        amplitude: 0.0,
        buffer_frames: None,
        stall_at: None,
        stall_ms: 0,
        out: None,
    };
    let mut it = std::env::args().skip(1);
    while let Some(k) = it.next() {
        let v = it.next().unwrap_or_else(|| panic!("missing value for {k}"));
        match k.as_str() {
            "--seconds" => a.seconds = v.parse().expect("--seconds"),
            "--amplitude" => a.amplitude = v.parse().expect("--amplitude"),
            "--buffer-frames" => a.buffer_frames = Some(v.parse().expect("--buffer-frames")),
            "--stall-at" => a.stall_at = Some(v.parse().expect("--stall-at")),
            "--stall-ms" => a.stall_ms = v.parse().expect("--stall-ms"),
            "--out" => a.out = Some(v),
            _ => panic!("unknown argument {k}"),
        }
    }
    a
}

fn json_opt<T: std::fmt::Display>(v: Option<T>) -> String {
    v.map_or_else(|| "null".to_string(), |v| v.to_string())
}

fn main() {
    let args = parse();
    let (out, mut tx) = shared::open(OpenOptions {
        ring_frames: 24_000,
        buffer_frames: args.buffer_frames,
    })
    .expect("open shared output");
    let rate = out.info.sample_rate;
    let ch = out.info.channels as usize;
    eprintln!("opened {:?}", out.info);

    let seconds = args.seconds;
    let amplitude = args.amplitude;
    let stall = args
        .stall_at
        .map(|at| (at, Duration::from_millis(args.stall_ms)));
    let producer = std::thread::spawn(move || {
        let total = (seconds * f64::from(rate)) as u64;
        let mut frame = 0u64;
        let mut block = Vec::with_capacity(1024 * ch);
        let started = Instant::now();
        let mut stalled = false;
        while frame < total {
            if let Some((at, pause)) = stall
                && !stalled
                && started.elapsed().as_secs_f64() >= at
            {
                stalled = true;
                std::thread::sleep(pause);
            }
            let n = 1024.min(total - frame);
            block.clear();
            for f in frame..frame + n {
                let phase = 2.0 * std::f64::consts::PI * 440.0 * f as f64 / f64::from(rate);
                let v = amplitude * phase.sin() as f32;
                block.extend(std::iter::repeat_n(v, ch));
            }
            let mut sent = 0;
            while sent < block.len() {
                sent += tx.push(&block[sent..]);
                if sent < block.len() {
                    std::thread::sleep(Duration::from_millis(2));
                }
            }
            frame += n;
        }
        tx.finish();
        frame
    });

    let alloc_before = rt_alloc_events();
    let started = Instant::now();
    let mut samples: Vec<(f64, u64)> = Vec::new();
    let mut regressions = 0u64;
    let mut last = 0u64;
    let mut max_latency = 0u64;
    let mut min_latency = u64::MAX;
    loop {
        std::thread::sleep(Duration::from_millis(100));
        let s = out.clock.snapshot();
        if s.presented_frames < last {
            regressions += 1;
        }
        last = s.presented_frames;
        if s.callbacks > 0 {
            max_latency = max_latency.max(s.latency_frames);
            min_latency = min_latency.min(s.latency_frames);
        }
        samples.push((started.elapsed().as_secs_f64(), s.presented_frames));
        let drained = out.stats.report().drained_frames > 0;
        if (producer.is_finished() && drained) || started.elapsed().as_secs_f64() > seconds + 10.0 {
            break;
        }
    }
    let produced = producer.join().expect("producer");
    std::thread::sleep(Duration::from_millis(200));
    let rt_allocs = rt_alloc_events() - alloc_before;
    let stats = out.stats.report();
    let clock = out.clock.snapshot();
    let device = out.device_callbacks.report();
    let max_frames = out.max_callback_frames.load(Ordering::Relaxed);
    let min_frames = match out.min_callback_frames.load(Ordering::Relaxed) {
        u64::MAX => 0,
        f => f,
    };
    // Gate against the shortest callback period seen: the strictest budget.
    let period_ns = min_frames * 1_000_000_000 / u64::from(rate);

    // Clock rate over the steady part: first sample with presented > 0 until
    // the last sample before content ran out.
    let steady: Vec<_> = samples
        .iter()
        .filter(|(_, p)| *p > 0 && *p < clock.content_frames.saturating_sub(u64::from(rate)))
        .collect();
    let clock_rate = match (steady.first(), steady.last()) {
        (Some(a), Some(b)) if b.0 > a.0 => (b.1 - a.1) as f64 / (b.0 - a.0),
        _ => 0.0,
    };

    let json = format!(
        concat!(
            "{{\n",
            "  \"host\": {:?},\n  \"device\": {:?},\n  \"sample_rate\": {},\n  \"channels\": {},\n",
            "  \"sample_format\": {:?},\n  \"supported_buffer\": {:?},\n  \"requested_buffer_frames\": {},\n",
            "  \"seconds\": {},\n  \"amplitude\": {},\n  \"stall_at_s\": {},\n  \"stall_ms\": {},\n",
            "  \"produced_frames\": {},\n  \"content_frames\": {},\n  \"silence_frames\": {},\n",
            "  \"presented_frames_final\": {},\n  \"presented_regressions\": {},\n",
            "  \"latency_frames_min\": {},\n  \"latency_frames_max\": {},\n",
            "  \"clock_rate_frames_per_s\": {:.1},\n  \"clock_rate_ratio\": {:.5},\n",
            "  \"device_callbacks\": {},\n  \"render_chunks\": {},\n",
            "  \"min_callback_frames\": {},\n  \"max_callback_frames\": {},\n  \"min_period_ns\": {},\n",
            "  \"device_callback_max_ns\": {},\n  \"device_callback_p999_ns_upper_bound\": {},\n",
            "  \"p999_over_min_period\": {:.4},\n",
            "  \"underruns\": {},\n  \"underrun_frames\": {},\n  \"drained_frames\": {},\n",
            "  \"rt_alloc_events\": {},\n  \"stream_errors\": {}\n}}\n"
        ),
        out.info.host,
        out.info.device,
        rate,
        ch,
        out.info.sample_format,
        out.info.supported_buffer,
        json_opt(out.info.requested_buffer_frames),
        seconds,
        amplitude,
        json_opt(args.stall_at),
        args.stall_ms,
        produced,
        clock.content_frames,
        clock.silence_frames,
        clock.presented_frames,
        regressions,
        if min_latency == u64::MAX {
            0
        } else {
            min_latency
        },
        max_latency,
        clock_rate,
        clock_rate / f64::from(rate),
        device.callbacks,
        stats.callbacks,
        min_frames,
        max_frames,
        period_ns,
        device.max_ns,
        device.p999_ns,
        if period_ns > 0 {
            device.p999_ns as f64 / period_ns as f64
        } else {
            0.0
        },
        stats.underruns,
        stats.underrun_frames,
        stats.drained_frames,
        rt_allocs,
        out.errors.load(Ordering::Relaxed),
    );
    print!("{json}");
    if let Some(path) = args.out {
        std::fs::write(&path, &json).expect("write --out");
        eprintln!("wrote {path}");
    }
}
