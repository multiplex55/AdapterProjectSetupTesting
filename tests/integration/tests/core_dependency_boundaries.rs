use std::collections::HashSet;
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
    let metadata = cargo_metadata();

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

#[test]
fn target_apps_do_not_link_sim_only_adapters() {
    let metadata = cargo_metadata();

    let target5_app = package_by_manifest(&metadata, "/apps/target5-app/Cargo.toml");
    let target10_app = package_by_manifest(&metadata, "/apps/target10-app/Cargo.toml");

    assert_dependencies(
        target5_app,
        &["adapter-target5"],
        &["adapter-windows-sim", "adapter-sim-transport"],
    );

    assert_dependencies(
        target10_app,
        &["adapter-target10"],
        &["adapter-windows-sim", "adapter-sim-transport"],
    );
}

fn cargo_metadata() -> Metadata {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .expect("failed to execute cargo metadata");

    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout).expect("failed to parse cargo metadata json")
}

fn package_by_manifest<'a>(metadata: &'a Metadata, suffix: &str) -> &'a Package {
    metadata
        .packages
        .iter()
        .find(|pkg| pkg.manifest_path.ends_with(suffix))
        .unwrap_or_else(|| panic!("workspace package for {suffix} not found in cargo metadata"))
}

fn assert_dependencies(pkg: &Package, required: &[&str], forbidden: &[&str]) {
    let dependency_names: HashSet<&str> = pkg
        .dependencies
        .iter()
        .map(|dep| dep.name.as_str())
        .collect();

    for required_dep in required {
        assert!(
            dependency_names.contains(required_dep),
            "package '{}' is missing required dependency '{}'",
            pkg.name,
            required_dep
        );
    }

    for forbidden_dep in forbidden {
        assert!(
            !dependency_names.contains(forbidden_dep),
            "package '{}' must not depend on sim-only adapter '{}'",
            pkg.name,
            forbidden_dep
        );
    }
}
