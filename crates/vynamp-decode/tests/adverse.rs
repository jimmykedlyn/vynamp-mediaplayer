use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};
use vynamp_decode::{DecodeError, Limits, Samples, decode, registry};

fn fixture(name: &str) -> Vec<u8> {
    fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/audio/decode")
            .join(name),
    )
    .unwrap()
}

#[test]
fn registry_describes_only_experimental_probed_bindings() {
    let r = registry();
    assert_eq!(r.len(), 3);
    assert_eq!(
        r.iter().map(|b| b.container).collect::<Vec<_>>(),
        ["WAV", "FLAC", "MPEG audio"]
    );
    for b in r {
        assert_eq!(b.backend, "Symphonia");
        assert_eq!(b.version, "0.6.1");
        assert_eq!(b.status, "experimental");
    }
    assert_eq!(r[0].profile, "PCM signed little-endian 24/32");
    assert_eq!(r[1].profile, "PCM24 native FLAC");
}

#[test]
fn stale_generation_never_returns_pcm_and_can_restart_with_new_generation() {
    let generation = AtomicU64::new(42);
    let bytes = fixture("pcm32-a.wav");
    assert_eq!(
        decode(bytes.clone(), Limits::default(), &generation, 41).unwrap_err(),
        DecodeError::Cancelled
    );
    generation.store(43, Ordering::Release);
    assert_eq!(
        decode(bytes.clone(), Limits::default(), &generation, 42).unwrap_err(),
        DecodeError::Cancelled
    );
    assert_eq!(
        decode(bytes, Limits::default(), &generation, 43)
            .unwrap()
            .frames,
        4417
    );
}

#[test]
fn byte_frame_rate_and_channel_budgets_reject_before_returning_pcm() {
    for limits in [
        Limits {
            max_bytes: 100,
            ..Limits::default()
        },
        Limits {
            max_frames: 4416,
            ..Limits::default()
        },
        Limits {
            max_rate: 44099,
            ..Limits::default()
        },
        Limits {
            max_channels: 1,
            ..Limits::default()
        },
        Limits {
            max_frames: 0,
            ..Limits::default()
        },
    ] {
        assert_eq!(
            decode(fixture("pcm24-a.flac"), limits, &AtomicU64::new(1), 1).unwrap_err(),
            DecodeError::Limit
        );
    }
}

#[test]
fn invalid_and_truncated_streams_are_errors_not_partial_success() {
    for bytes in [Vec::new(), b"not audio".to_vec()] {
        assert!(decode(bytes, Limits::default(), &AtomicU64::new(1), 1).is_err());
    }
    for name in ["pcm24-a.wav", "pcm24-a.flac", "tone-cbr-a.mp3"] {
        let mut bytes = fixture(name);
        bytes.truncate(bytes.len() / 2);
        assert!(
            decode(bytes, Limits::default(), &AtomicU64::new(1), 1).is_err(),
            "{name}"
        );
    }
}

#[test]
fn mp3_without_trustworthy_trim_is_rejected_instead_of_partial_success() {
    for name in ["tone-no-xing.mp3", "tone-vbr-long-no-xing.mp3"] {
        let bytes = fixture(name);
        assert_eq!(
            decode(bytes.clone(), Limits::default(), &AtomicU64::new(1), 1).unwrap_err(),
            DecodeError::Unsupported,
            "{name}"
        );
        for cut in [bytes.len() / 2, bytes.len() - 1] {
            assert!(
                decode(
                    bytes[..cut].to_vec(),
                    Limits::default(),
                    &AtomicU64::new(1),
                    1
                )
                .is_err(),
                "{name} cut={cut}"
            );
        }
    }
}

#[test]
fn wav32_lsb_extremes_and_negative_one_survive_without_float_conversion() {
    let d = decode(
        fixture("pcm32-a.wav"),
        Limits::default(),
        &AtomicU64::new(1),
        1,
    )
    .unwrap();
    let Samples::Integer(s) = d.samples else {
        panic!("integer path required")
    };
    assert_eq!(
        &s[..8],
        &[i32::MIN, i32::MIN, i32::MAX, i32::MAX, -1, -1, 1, 1]
    );
}
