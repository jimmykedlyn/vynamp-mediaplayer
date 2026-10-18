//! Complementary static guard: code between `RT-BEGIN` and `RT-END` in the
//! callback-path modules must not name blocking, allocating, I/O or logging
//! facilities. The allocator test proves the allocation part dynamically.
use std::path::Path;

// Paths relative to this crate; the output adapter shares the same rules.
const RT_FILES: &[&str] = &[
    "src/ring.rs",
    "src/clock.rs",
    "src/stats.rs",
    "src/callback.rs",
    "../vynamp-output/src/convert.rs",
    "../vynamp-output/src/shared.rs",
];

const FORBIDDEN: &[&str] = &[
    "Mutex",
    "RwLock",
    "Condvar",
    "Barrier",
    "park(",
    "sleep(",
    "println!",
    "eprintln!",
    "print!",
    "dbg!",
    "format!",
    "panic!",
    "std::fs",
    "std::io",
    "std::net",
    "File::",
    "Vec",
    "vec!",
    "Box::",
    "String",
    "to_string",
    "to_owned",
    "clone()",
    "collect",
    "Arc::new",
    "log::",
    "tracing::",
];

fn rt_regions(src: &str) -> Vec<&str> {
    let mut regions = Vec::new();
    let mut rest = src;
    while let Some(b) = rest.find("// RT-BEGIN") {
        let after = &rest[b..];
        let e = after.find("// RT-END").expect("RT-BEGIN without RT-END");
        regions.push(&after[..e]);
        rest = &after[e..];
    }
    regions
}

fn code_only(region: &str) -> String {
    region
        .lines()
        .map(|l| l.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn rt_regions_use_no_forbidden_facilities() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for file in RT_FILES {
        let src = std::fs::read_to_string(dir.join(file)).unwrap();
        let regions = rt_regions(&src);
        assert!(!regions.is_empty(), "{file} has no RT region");
        for region in regions {
            let code = code_only(region);
            for token in FORBIDDEN {
                assert!(!code.contains(token), "{file}: RT region uses `{token}`");
            }
        }
    }
}

#[test]
fn guard_rejects_a_forbidden_token() {
    let sample = "// RT-BEGIN\nfn f() { let m = std::sync::Mutex::new(0); }\n// RT-END\n";
    let code = code_only(rt_regions(sample)[0]);
    assert!(code.contains("Mutex"));
    let commented = "// RT-BEGIN\nfn f() {} // no Mutex here\n// RT-END\n";
    assert!(!code_only(rt_regions(commented)[0]).contains("Mutex"));
}
