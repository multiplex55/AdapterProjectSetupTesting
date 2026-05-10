use std::{fs, path::PathBuf};

use plugins_loader::{
    load_plugin, platform_library_extension, Capability, LoadErrorKind, LoadOutcome,
    PluginLoadRequest,
};

fn touch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("plugin-contract-tests");
    fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join(name);
    fs::write(&path, b"stub").expect("write");
    path
}

#[test]
fn valid_plugin_accepted() {
    let ext = platform_library_extension();
    let path = touch(&format!("compute-valid.{ext}"));
    let req = PluginLoadRequest {
        path,
        expected_capability: Capability::Compute,
        required: true,
    };
    let outcome = load_plugin(&req).expect("plugin should load");
    assert!(matches!(outcome, LoadOutcome::Loaded(_)));
}

#[test]
fn abi_mismatch_rejected() {
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
fn missing_optional_plugin_falls_back() {
    let ext = platform_library_extension();
    let path = std::env::temp_dir().join(format!("missing-optional.{ext}"));
    let req = PluginLoadRequest {
        path: path.clone(),
        expected_capability: Capability::Compute,
        required: false,
    };
    let outcome = load_plugin(&req).expect("optional missing should not fail");
    assert!(
        matches!(outcome, LoadOutcome::OptionalMissing { attempted_path } if attempted_path == path)
    );
}

#[test]
fn missing_required_plugin_fails_startup() {
    let ext = platform_library_extension();
    let path = std::env::temp_dir().join(format!("missing-required.{ext}"));
    let req = PluginLoadRequest {
        path: path.clone(),
        expected_capability: Capability::Compute,
        required: true,
    };
    let err = load_plugin(&req).expect_err("required missing should fail");
    assert_eq!(err.plugin_path_attempted, path);
    assert!(matches!(err.reason, LoadErrorKind::NotFound));
}
