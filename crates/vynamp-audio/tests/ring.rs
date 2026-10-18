use vynamp_audio::ring;

fn ramp(start: usize, len: usize) -> Vec<f32> {
    (start..start + len).map(|v| v as f32).collect()
}

#[test]
fn push_pop_roundtrip_preserves_order() {
    let (mut tx, mut rx) = ring::channel(8);
    assert_eq!(tx.push(&ramp(0, 5)), 5);
    let mut out = [0.0; 5];
    assert_eq!(rx.pop_into(&mut out), 5);
    assert_eq!(out.to_vec(), ramp(0, 5));
    assert_eq!(rx.available(), 0);
}

#[test]
fn push_never_overwrites_unread() {
    let (mut tx, mut rx) = ring::channel(4);
    assert_eq!(tx.push(&ramp(0, 6)), 4, "only free space is written");
    assert_eq!(tx.free(), 0);
    assert_eq!(tx.push(&[99.0]), 0);
    let mut out = [0.0; 6];
    assert_eq!(rx.pop_into(&mut out), 4);
    assert_eq!(&out[..4], &ramp(0, 4)[..]);
}

#[test]
fn wraparound_keeps_ramp_contiguous() {
    let (mut tx, mut rx) = ring::channel(5);
    let mut next = 0usize;
    let mut expected = 0usize;
    let mut out = [0.0; 3];
    for _ in 0..100 {
        next += tx.push(&ramp(next, 3));
        let n = rx.pop_into(&mut out);
        for v in &out[..n] {
            assert_eq!(*v, expected as f32);
            expected += 1;
        }
    }
    assert!(expected > 250);
}

#[test]
fn threaded_ramp_has_no_loss_or_duplicate() {
    const TOTAL: usize = 1 << 20;
    let (mut tx, mut rx) = ring::channel(1024);
    let producer = std::thread::spawn(move || {
        let mut next = 0usize;
        while next < TOTAL {
            let len = 37.min(TOTAL - next);
            // f32 represents integers exactly below 2^24.
            next += tx.push(&ramp(next, len));
            if next < TOTAL {
                std::thread::yield_now();
            }
        }
        tx.finish();
    });
    let mut expected = 0usize;
    let mut out = [0.0f32; 61];
    loop {
        let n = rx.pop_into(&mut out);
        for v in &out[..n] {
            assert_eq!(*v, expected as f32, "loss or duplicate at {expected}");
            expected += 1;
        }
        if n == 0 {
            if rx.is_finished() && rx.available() == 0 {
                break;
            }
            std::thread::yield_now();
        }
    }
    producer.join().unwrap();
    assert_eq!(expected, TOTAL);
}

#[test]
fn finish_is_visible_after_data() {
    let (mut tx, rx) = ring::channel(8);
    tx.push(&[1.0, 2.0]);
    tx.finish();
    assert!(rx.is_finished());
    assert_eq!(rx.available(), 2, "finish does not discard queued samples");
}

#[test]
#[should_panic(expected = "capacity")]
fn zero_capacity_is_rejected() {
    let _ = ring::channel(0);
}
