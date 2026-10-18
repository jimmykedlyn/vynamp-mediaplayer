use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::AtomicU64,
};
use vynamp_decode::{DecodeError, Limits, Samples, decode};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn manifest() -> Value {
    serde_json::from_slice(&fs::read(root().join("tests/fixtures/manifests/decode.json")).unwrap())
        .unwrap()
}
fn words(bytes: &[u8]) -> impl Iterator<Item = [u8; 4]> + '_ {
    let (samples, remainder) = bytes.as_chunks::<4>();
    assert!(
        remainder.is_empty(),
        "reference must contain complete samples"
    );
    samples.iter().copied()
}

#[test]
fn fixture_hashes_sizes_and_provenance_are_pinned() {
    let m = manifest();
    let schema: Value = serde_json::from_slice(
        &fs::read(root().join("tests/fixtures/manifests/decode.schema.json")).unwrap(),
    )
    .unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    validator.validate(&m).unwrap();
    let mut invalid = m.clone();
    invalid.as_object_mut().unwrap().remove("permission");
    assert!(!validator.is_valid(&invalid));
    assert!(m["origin"].as_str().unwrap().contains("Synthetic"));
    assert!(!m["permission"].as_str().unwrap().is_empty());
    for (path, meta) in m["files"].as_object().unwrap() {
        let bytes = fs::read(root().join(path)).unwrap();
        assert_eq!(
            bytes.len() as u64,
            meta["bytes"].as_u64().unwrap(),
            "{path}"
        );
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            meta["sha256"].as_str().unwrap(),
            "{path}"
        );
    }
    let mut ids = std::collections::HashSet::new();
    for fixture in m["fixtures"].as_array().unwrap() {
        assert!(ids.insert(fixture["id"].as_str().unwrap()));
        for key in ["path", "reference", "source"] {
            assert!(m["files"].get(fixture[key].as_str().unwrap()).is_some());
        }
        if fixture["format"] != "mp3" {
            assert_eq!(fixture["max_abs_error"], 0);
        }
    }
}

#[test]
fn real_corpus_matches_independent_reference_and_frame_counts() {
    for f in manifest()["fixtures"].as_array().unwrap() {
        let path = f["path"].as_str().unwrap();
        if f["expected_error"] == "Unsupported" {
            assert_eq!(
                decode(
                    fs::read(root().join(path)).unwrap(),
                    Limits::default(),
                    &AtomicU64::new(7),
                    7
                )
                .unwrap_err(),
                DecodeError::Unsupported
            );
            continue;
        }
        let d = decode(
            fs::read(root().join(path)).unwrap(),
            Limits::default(),
            &AtomicU64::new(7),
            7,
        )
        .unwrap_or_else(|e| panic!("{path}: {e:?}"));
        assert_eq!(d.rate as u64, f["rate"].as_u64().unwrap());
        assert_eq!(d.channels as u64, f["channels"].as_u64().unwrap());
        assert_eq!(d.frames as u64, f["frames"].as_u64().unwrap(), "{path}");
        let reference = fs::read(root().join(f["reference"].as_str().unwrap())).unwrap();
        match d.samples {
            Samples::Integer(samples) => {
                assert_eq!(d.bits.map(u64::from), f["bits"].as_u64());
                let expected: Vec<i32> = words(&reference).map(i32::from_le_bytes).collect();
                assert_eq!(samples, expected, "{path}: integer precision/channel order");
                assert_eq!(
                    reference,
                    fs::read(root().join(f["source"].as_str().unwrap())).unwrap()
                );
                println!("{path}: {} frames, exact integer match", d.frames);
            }
            Samples::Float(samples) => {
                assert_eq!(f["format"], "mp3");
                if f["trim_unknown"] == true {
                    assert_eq!(d.delay, None);
                    assert_eq!(d.padding, None);
                    assert!(
                        d.frames > 4417,
                        "missing metadata must not infer silence trim"
                    );
                } else {
                    assert!(d.delay.unwrap_or(0) > 0, "MP3 must expose trusted delay");
                    assert!(d.padding.is_some());
                }
                assert_eq!(samples.len() * 4, reference.len());
                let max_error = samples
                    .iter()
                    .zip(words(&reference).map(f32::from_le_bytes))
                    .map(|(a, b)| {
                        assert!(a.is_finite() && b.is_finite());
                        (a - b).abs()
                    })
                    .fold(0.0_f32, f32::max);
                println!(
                    "{path}: {} frames, delay={:?}, padding={:?}, max_abs_error={max_error:e}",
                    d.frames, d.delay, d.padding
                );
                assert!(
                    max_error <= f["max_abs_error"].as_f64().unwrap() as f32,
                    "{path}: {max_error}"
                );
            }
        }
    }
}

#[test]
fn gapless_pairs_match_concatenated_references_without_dropped_or_inserted_frames() {
    for (prefix, suffix) in [
        ("pcm24", ".wav"),
        ("pcm24", ".flac"),
        ("pcm32", ".wav"),
        ("tone-cbr", ".mp3"),
        ("tone-vbr", ".mp3"),
    ] {
        let m = manifest();
        let fixtures: Vec<_> = m["fixtures"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|f| {
                let name = Path::new(f["path"].as_str().unwrap())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();
                name.starts_with(prefix) && name.ends_with(suffix) && f["trim_unknown"] != true
            })
            .collect();
        assert_eq!(fixtures.len(), 2);
        let mut actual = Vec::new();
        let mut reference = Vec::new();
        for f in fixtures {
            let d = decode(
                fs::read(root().join(f["path"].as_str().unwrap())).unwrap(),
                Limits::default(),
                &AtomicU64::new(1),
                1,
            )
            .unwrap();
            assert_eq!(d.frames, 4417);
            match d.samples {
                Samples::Integer(v) => {
                    actual.extend(v.into_iter().map(|s| s as f64 / 2147483648.0))
                }
                Samples::Float(v) => actual.extend(v.into_iter().map(f64::from)),
            }
            let bytes = fs::read(root().join(f["reference"].as_str().unwrap())).unwrap();
            reference.extend(words(&bytes).map(|s| {
                if f["sample_type"] == "f32" {
                    f32::from_le_bytes(s) as f64
                } else {
                    i32::from_le_bytes(s) as f64 / 2147483648.0
                }
            }));
        }
        assert_eq!(actual.len(), 8834 * 2);
        assert_eq!(actual.len(), reference.len());
        assert!(
            actual
                .iter()
                .zip(reference)
                .all(|(a, b)| (a - b).abs() <= 0.00002)
        );
    }
}
