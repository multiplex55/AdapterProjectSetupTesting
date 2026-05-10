use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

use serde_json::Value;

#[test]
fn readme_package_examples_reference_workspace_packages_only() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root path");

    let metadata_output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(&repo_root)
        .output()
        .expect("run cargo metadata");
    assert!(
        metadata_output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&metadata_output.stderr)
    );

    let metadata: Value = serde_json::from_slice(&metadata_output.stdout).expect("metadata json");
    let package_names: HashSet<String> = metadata["packages"]
        .as_array()
        .expect("packages array")
        .iter()
        .filter_map(|pkg| pkg["name"].as_str().map(ToOwned::to_owned))
        .collect();

    let readme_path = repo_root.join("README.md");
    let readme = std::fs::read_to_string(&readme_path).expect("read README.md");

    let mut unknown_examples = Vec::new();
    for line in readme.lines() {
        let marker = "cargo run -p ";
        if let Some(start) = line.find(marker) {
            let rest = &line[start + marker.len()..];
            let name: String = rest
                .chars()
                .take_while(|c| !c.is_whitespace())
                .collect();
            if !name.is_empty() && !package_names.contains(&name) {
                unknown_examples.push(name);
            }
        }
    }

    assert!(
        unknown_examples.is_empty(),
        "README contains unknown cargo run package example(s): {:?}",
        unknown_examples
    );
}
