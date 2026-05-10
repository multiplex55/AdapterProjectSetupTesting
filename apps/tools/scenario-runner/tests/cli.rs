use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn bin_path() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_scenario-runner"))
}

fn temp_file(name: &str, content: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("scenario_runner_{name}_{nanos}.json"));
    fs::write(&path, content).expect("write fixture");
    path
}

#[test]
fn requires_scenario_argument() {
    let output = Command::new(bin_path()).output().expect("run binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing required --scenario <path> argument"));
}

#[test]
fn reports_pass_for_matching_expected_outputs() {
    let scenario = r#"{
  "scenario": {"id":"ok-1","name":"happy","description":"pass case"},
  "profile": {"source":"windows-target5-sim","target":"windows-target10-sim"},
  "events": [
    {"timestamp_ms":1000,"kind":"target5_status","payload":{"device_id":5,"online":true,"sequence":1}},
    {"timestamp_ms":1200,"kind":"target5_status","payload":{"device_id":10,"online":false,"sequence":2}}
  ],
  "expected": {"outputs": [
    {"kind":"target10_command","payload":{"command_id":1005,"action":"arm","priority":1}},
    {"kind":"target10_command","payload":{"command_id":2010,"action":"standby","priority":5}}
  ]}
}"#;
    let path = temp_file("pass", scenario);

    let output = Command::new(bin_path())
        .arg("--scenario")
        .arg(&path)
        .output()
        .expect("run binary");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("startup.scenario_path="));
    assert!(stdout.contains("startup.event_count=2"));
    assert!(stdout.contains("startup.mapping_provider=core::algorithms::target5_to_target10::map_target5_status_to_target10_command"));
    assert!(stdout.contains("startup.mapping_source=windows-target5-sim"));
    assert!(stdout.contains("summary.result=PASS outputs=2 mismatches=0"));
}

#[test]
fn reports_failure_and_non_zero_exit_on_mismatch() {
    let scenario = r#"{
  "scenario": {"id":"bad-1","name":"mismatch","description":"fail case"},
  "profile": {"source":"windows-target5-sim","target":"windows-target10-sim"},
  "events": [
    {"timestamp_ms":1000,"kind":"target5_status","payload":{"device_id":5,"online":true,"sequence":1}}
  ],
  "expected": {"outputs": [
    {"kind":"target10_command","payload":{"command_id":9999,"action":"arm","priority":1}}
  ]}
}"#;
    let path = temp_file("mismatch", scenario);

    let output = Command::new(bin_path())
        .arg("--scenario")
        .arg(&path)
        .output()
        .expect("run binary");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("summary.result=FAIL expected_outputs=1 actual_outputs=1"));
    assert!(stderr.contains("output mismatch"));
}
