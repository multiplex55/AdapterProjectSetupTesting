use std::{fs, path::PathBuf};

use plugins_loader::{
    load_plugin, platform_library_extension, Capability, LoadErrorKind, LoadOutcome,
    PluginLoadRequest,
};

fn touch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("plugin-contract-tests-vslice");
    fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join(name);
    fs::write(&path, b"stub").expect("write");
    path
}

#[test]
fn required_plugin_mismatch_is_detectable() {
    let ext = platform_library_extension();
    let path = touch(&format!("compute-abi2.{ext}"));
    let req = PluginLoadRequest {
        path: path.clone(),
        expected_capability: Capability::Compute,
        required: true,
    };
    let err = load_plugin(&req).expect_err("abi mismatch expected");
    assert_eq!(err.plugin_path_attempted, path);
    assert!(matches!(err.reason, LoadErrorKind::AbiMismatch { .. }));
}

#[test]
fn optional_plugin_fallback_is_detectable() {
    let ext = platform_library_extension();
    let path = std::env::temp_dir().join(format!("missing-vslice-optional.{ext}"));
    let req = PluginLoadRequest {
        path: path.clone(),
        expected_capability: Capability::Compute,
        required: false,
    };

    let outcome = load_plugin(&req).expect("fallback should be non-fatal");
    assert!(
        matches!(outcome, LoadOutcome::OptionalMissing { attempted_path } if attempted_path == path)
    );
}
