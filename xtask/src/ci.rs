use crate::{util::*, validate};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};
pub fn detect(root: &Path) -> Result<Value> {
    let cfg = read_json(root.join("config/project.json"))?;
    let source = root.join("apps/desktop/src/main.rs").is_file();
    let manifest = root.join("apps/desktop/Cargo.toml");
    let candidate = source
        && manifest.is_file()
        && toml::from_str::<toml::Value>(&text(&manifest)?)?
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(toml::Value::as_str)
            == cfg["desktop_package"].as_str();
    let core = if candidate {
        let meta: Value = serde_json::from_str(&capture(
            root,
            "cargo",
            &["metadata", "--no-deps", "--format-version", "1", "--locked"],
        )?)?;
        let members = meta["workspace_members"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Cargo metadata workspace members absent"))?;
        let expected = manifest.canonicalize()?;
        let binary = string(&cfg, "desktop_binary")?;
        meta["packages"].as_array().into_iter().flatten().any(|p| {
            p["name"] == cfg["desktop_package"]
                && members.contains(&p["id"])
                && p["manifest_path"].as_str().is_some_and(|path| {
                    std::path::Path::new(path)
                        .canonicalize()
                        .is_ok_and(|v| v == expected)
                })
                && p["targets"].as_array().into_iter().flatten().any(|t| {
                    t["name"] == binary
                        && t["kind"]
                            .as_array()
                            .is_some_and(|k| k.iter().any(|v| v == "bin"))
                })
        })
    } else {
        false
    };
    let android = root.join("apps/android/gradlew").is_file()
        && root.join("apps/android/gradlew.bat").is_file()
        && root.join("apps/android/app/build.gradle.kts").is_file();
    ensure!(
        cfg["application_ready"] != true || (core && android),
        "Application marked ready but actual entrypoints absent"
    );
    Ok(json!({"core":core.to_string(),"android":android.to_string()}))
}
pub fn emit_detect(root: &Path) -> Result<()> {
    let v = detect(root)?;
    if let Ok(p) = std::env::var("GITHUB_OUTPUT") {
        let mut f = fs::OpenOptions::new().append(true).open(p)?;
        for k in ["core", "android"] {
            writeln!(f, "{k}={}", string(&v, k)?)?;
        }
    }
    println!("{v}");
    Ok(())
}
pub fn docs(root: &Path) -> Result<()> {
    validate::spec(root)?;
    validate::delivery(root)?;
    run(root, "cargo", &["fmt", "--all", "--", "--check"])?;
    // Windows keeps the executing driver locked. Child builds must use a
    // different target directory, including when invoked from a check binary.
    let canonical_root = root.canonicalize()?;
    let executing = std::env::current_exe()?.canonicalize()?;
    let target = command_path(&docs_target(&canonical_root, &executing)?)?;
    run(
        root,
        "cargo",
        &[
            "clippy",
            "--target-dir",
            &target,
            "--workspace",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    // Workspace tests cover tooling and real library crates; features that
    // need system audio headers (e.g. vynamp-output/cpal) stay off here.
    run(
        root,
        "cargo",
        &["test", "--target-dir", &target, "--workspace", "--locked"],
    )?;
    Ok(())
}
pub fn toolchains(root: &Path, android: bool) -> Result<Value> {
    let cfg = read_json(root.join("config/toolchains.json"))?;
    ensure!(
        cfg["rust"].is_string()
            && root.join("rust-toolchain.toml").is_file()
            && root.join("Cargo.lock").is_file(),
        "Rust pins/lock missing"
    );
    if android {
        for k in [
            "android_ndk",
            "android_compile_sdk",
            "android_target_sdk",
            "android_build_tools",
        ] {
            ensure!(!cfg[k].is_null(), "Pin {k} in M0 before Android builds");
        }
    }
    run(
        root,
        "rustup",
        &[
            "toolchain",
            "install",
            string(&cfg, "rust")?,
            "--profile",
            "minimal",
            "--component",
            "rustfmt",
            "--component",
            "clippy",
        ],
    )?;
    Ok(cfg)
}
pub fn core(root: &Path, target: &str) -> Result<()> {
    ensure!(detect(root)?["core"] == "true", "Actual desktop app absent");
    let cfg = toolchains(root, false)?;
    run(
        root,
        "rustup",
        &[
            "target",
            "add",
            target,
            "--toolchain",
            string(&cfg, "rust")?,
        ],
    )?;
    run(root, "cargo", &["fmt", "--all", "--", "--check"])?;
    run(
        root,
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--locked",
            "--target",
            target,
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run(
        root,
        "cargo",
        &[
            "test",
            "--workspace",
            "--all-targets",
            "--locked",
            "--target",
            target,
        ],
    )
}
pub fn gradle(root: &Path, tasks: &[&str], envs: &[(String, String)]) -> Result<()> {
    let cfg = read_json(root.join("config/toolchains.json"))?;
    let mut c = std::process::Command::new(if cfg!(windows) {
        "apps/android/gradlew.bat"
    } else {
        "bash"
    });
    if !cfg!(windows) {
        c.arg("apps/android/gradlew");
    }
    c.args(["-p", "apps/android", "--no-daemon"])
        .args(tasks)
        .current_dir(root)
        .envs(envs.iter().cloned());
    if let Ok(java) = std::env::var(format!("JAVA_HOME_{}_X64", string(&cfg, "java")?)) {
        c.env("JAVA_HOME", java);
    }
    ensure!(c.status()?.success(), "Gradle failed");
    Ok(())
}
pub fn android(root: &Path) -> Result<()> {
    ensure!(
        detect(root)?["android"] == "true",
        "Actual Android app absent"
    );
    let cfg = toolchains(root, true)?;
    required("ANDROID_HOME")?;
    run(
        root,
        "rustup",
        &[
            "target",
            "add",
            "aarch64-linux-android",
            "--toolchain",
            string(&cfg, "rust")?,
        ],
    )?;
    let sdk = cfg["android_compile_sdk"]
        .as_str()
        .map(str::to_owned)
        .unwrap_or(cfg["android_compile_sdk"].to_string());
    run(
        root,
        "sdkmanager",
        &[
            &format!("ndk;{}", string(&cfg, "android_ndk")?),
            &format!("platforms;android-{sdk}"),
        ],
    )?;
    gradle(
        root,
        &["testDebugUnitTest", "lintDebug", "assembleDebug"],
        &[],
    )
}
pub fn quality(root: &Path) -> Result<()> {
    ensure!(
        read_json(root.join("config/project.json"))?["application_ready"] == true,
        "Hardware gate blocked: application not implemented"
    );
    ensure!(
        root.join("tests/hardware/Cargo.toml").is_file(),
        "Hardware harness missing; implement Rust harness before acceptance"
    );
    run(
        root,
        "cargo",
        &[
            "run",
            "--locked",
            "--manifest-path",
            "tests/hardware/Cargo.toml",
            "--",
            "--soak",
            "--device-loss",
            "--gapless",
            "--transparent",
            "--report-dir",
            "dist/evidence",
        ],
    )
}
pub fn gate() -> Result<()> {
    ensure!(
        required("DOCS")? == "success",
        "Documentation/tooling checks failed"
    );
    for (check, active) in [("CORE", "HAS_CORE"), ("ANDROID", "HAS_ANDROID")] {
        if required(active)? == "true" {
            ensure!(required(check)? == "success", "{check} failed/cancelled");
        }
    }
    println!("CI Gate passed; hardware acceptance remains separate");
    Ok(())
}

fn docs_target(root: &Path, executing: &Path) -> Result<std::path::PathBuf> {
    for name in ["xtask-ci", "xtask-ci-alt"] {
        let candidate = root.join("target").join(name);
        let mut existing = candidate.as_path();
        while !existing.try_exists()? {
            existing = existing
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Cache parent missing"))?;
        }
        ensure!(
            existing.canonicalize()?.starts_with(root),
            "Check cache parent resolves outside the worktree; use a local cache"
        );
        fs::create_dir_all(&candidate)?;
        let actual = candidate.canonicalize()?;
        if !executing.starts_with(&actual) {
            return Ok(actual);
        }
    }
    anyhow::bail!(
        "Check caches resolve to the running executable directory; use separate cache directories"
    )
}

#[cfg(test)]
mod docs_target_tests {
    use super::*;
    #[test]
    fn checks_do_not_rebuild_the_running_driver() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().canonicalize().unwrap();
        for exe in [
            root.join("target/debug/xtask.exe"),
            root.join("target/xtask-ci/debug/xtask.exe"),
            root.join("target/xtask-ci-alt/debug/xtask.exe"),
        ] {
            let target = docs_target(&root, &exe).unwrap();
            assert!(
                !exe.starts_with(&target),
                "check target contains running executable"
            );
        }
    }
    #[test]
    fn separate_worktrees_have_separate_check_caches() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().canonicalize().unwrap();
        let exe = root.join("external-cache/debug/xtask.exe");
        fs::create_dir_all(root.join("codex-worktree")).unwrap();
        fs::create_dir_all(root.join("claude-worktree")).unwrap();
        let a = docs_target(&root.join("codex-worktree"), &exe).unwrap();
        let b = docs_target(&root.join("claude-worktree"), &exe).unwrap();
        assert_ne!(a, b);
        assert!(a.starts_with(root.join("codex-worktree/target")));
        assert!(b.starts_with(root.join("claude-worktree/target")));
    }
    #[cfg(unix)]
    #[test]
    fn cache_alias_cannot_resolve_to_running_driver_directory() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().canonicalize().unwrap();
        let shared = root.join("shared-cache");
        fs::create_dir_all(shared.join("debug")).unwrap();
        fs::create_dir_all(root.join("target")).unwrap();
        std::os::unix::fs::symlink(&shared, root.join("target/xtask-ci")).unwrap();
        let exe = shared.join("debug/xtask.exe");
        fs::write(&exe, "fixture").unwrap();
        let selected = docs_target(&root, &exe).unwrap();
        fs::create_dir_all(&selected).unwrap();
        assert!(!exe.starts_with(selected.canonicalize().unwrap()));
    }
    #[cfg(unix)]
    #[test]
    fn outside_cache_parent_is_rejected_before_creating_files() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("project");
        let outside = temp.path().join("outside");
        fs::create_dir_all(&root).unwrap();
        fs::create_dir_all(&outside).unwrap();
        std::os::unix::fs::symlink(&outside, root.join("target")).unwrap();
        assert!(docs_target(&root, &root.join("driver.exe")).is_err());
        assert_eq!(fs::read_dir(outside).unwrap().count(), 0);
    }
}
