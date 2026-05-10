use std::path::Path;
use std::process::Command;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    manifest_path: String,
    dependencies: Vec<Dependency>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    name: String,
    path: Option<String>,
}

#[test]
fn core_has_no_forbidden_workspace_edges() {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .expect("failed to execute cargo metadata");

    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let metadata: Metadata =
        serde_json::from_slice(&output.stdout).expect("failed to parse cargo metadata json");

    let core_pkg = metadata
        .packages
        .iter()
        .find(|pkg| pkg.manifest_path.ends_with("/crates/core/Cargo.toml"))
        .expect("workspace package for crates/core not found in cargo metadata");

    let forbidden: Vec<&str> = core_pkg
        .dependencies
        .iter()
        .filter(|dep| {
            dep.path
                .as_ref()
                .is_some_and(|path| Path::new(path).is_absolute())
                && (dep.name == "runtime"
                    || dep.name.starts_with("adapter-")
                    || dep.name.starts_with("plugins-")
                    || dep.name.starts_with("ffi-"))
        })
        .map(|dep| dep.name.as_str())
        .collect();

    assert!(
        forbidden.is_empty(),
        "core package '{}' has forbidden workspace dependencies: {:?}",
        core_pkg.name,
        forbidden
    );
}
