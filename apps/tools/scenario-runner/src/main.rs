use std::{fs, path::PathBuf, process::ExitCode};

use core::algorithms::target5_to_target10::map_target5_status_to_target10_command;
use messages::{Target10Command, Target5Status};
use serde::Deserialize;

#[derive(Debug)]
struct Cli {
    scenario: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ScenarioDocument {
    scenario: ScenarioMetadata,
    profile: ScenarioProfile,
    events: Vec<ScenarioEvent>,
    expected: ScenarioExpected,
}

#[derive(Debug, Deserialize)]
struct ScenarioMetadata {
    id: String,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct ScenarioProfile {
    source: String,
    target: String,
}

#[derive(Debug, Deserialize)]
struct ScenarioEvent {
    kind: String,
    payload: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ScenarioExpected {
    outputs: Vec<ExpectedOutput>,
}

#[derive(Debug, Deserialize)]
struct ExpectedOutput {
    kind: String,
    payload: Target10Command,
}

#[derive(Debug)]
enum RunnerError {
    MissingScenarioArg,
    UnknownArg(String),
    ReadScenario {
        path: PathBuf,
        source: std::io::Error,
    },
    ParseScenario {
        path: PathBuf,
        source: serde_json::Error,
    },
    UnexpectedEventKind {
        kind: String,
    },
    UnexpectedExpectedKind {
        kind: String,
    },
    Mapping {
        source: core::algorithms::target5_to_target10::Target5ToTarget10Error,
    },
    Mismatch {
        expected: Vec<Target10Command>,
        actual: Vec<Target10Command>,
    },
}

fn parse_cli(args: impl IntoIterator<Item = String>) -> Result<Cli, RunnerError> {
    let mut iter = args.into_iter();
    let _program = iter.next();

    let mut scenario = None;
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--scenario" => {
                let value = iter.next().ok_or(RunnerError::MissingScenarioArg)?;
                scenario = Some(PathBuf::from(value));
            }
            _ => return Err(RunnerError::UnknownArg(arg)),
        }
    }

    scenario
        .map(|scenario| Cli { scenario })
        .ok_or(RunnerError::MissingScenarioArg)
}

fn run(cli: Cli) -> Result<(), RunnerError> {
    let scenario_raw =
        fs::read_to_string(&cli.scenario).map_err(|source| RunnerError::ReadScenario {
            path: cli.scenario.clone(),
            source,
        })?;

    let scenario_doc: ScenarioDocument =
        serde_json::from_str(&scenario_raw).map_err(|source| RunnerError::ParseScenario {
            path: cli.scenario.clone(),
            source,
        })?;

    println!("startup.scenario_path={}", cli.scenario.display());
    println!("startup.event_count={}", scenario_doc.events.len());
    println!(
        "startup.mapping_provider=core::algorithms::target5_to_target10::map_target5_status_to_target10_command"
    );
    println!("startup.mapping_source={}", scenario_doc.profile.source);

    println!(
        "scenario.id={} name={} description={}",
        scenario_doc.scenario.id, scenario_doc.scenario.name, scenario_doc.scenario.description
    );
    println!(
        "scenario.profile.source={} target={}",
        scenario_doc.profile.source, scenario_doc.profile.target
    );

    let mut actual = Vec::new();
    for event in scenario_doc.events {
        match event.kind.as_str() {
            "target5_status" => {
                let status: Target5Status =
                    serde_json::from_value(event.payload).map_err(|source| {
                        RunnerError::ParseScenario {
                            path: cli.scenario.clone(),
                            source,
                        }
                    })?;
                let mapped = map_target5_status_to_target10_command(&status)
                    .map_err(|source| RunnerError::Mapping { source })?;
                actual.push(mapped);
            }
            other => {
                return Err(RunnerError::UnexpectedEventKind {
                    kind: other.to_string(),
                })
            }
        }
    }

    let mut expected = Vec::new();
    for output in scenario_doc.expected.outputs {
        if output.kind != "target10_command" {
            return Err(RunnerError::UnexpectedExpectedKind { kind: output.kind });
        }
        expected.push(output.payload);
    }

    if actual != expected {
        return Err(RunnerError::Mismatch { expected, actual });
    }

    println!("summary.result=PASS outputs={} mismatches=0", actual.len());
    Ok(())
}

fn main() -> ExitCode {
    let cli = match parse_cli(std::env::args()) {
        Ok(cli) => cli,
        Err(err) => {
            eprintln!("error: {}", render_error(&err));
            return ExitCode::from(2);
        }
    };

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {}", render_error(&err));
            if let RunnerError::Mismatch { expected, actual } = err {
                eprintln!(
                    "summary.result=FAIL expected_outputs={} actual_outputs={}",
                    expected.len(),
                    actual.len()
                );
            }
            ExitCode::from(1)
        }
    }
}

fn render_error(err: &RunnerError) -> String {
    match err {
        RunnerError::MissingScenarioArg => {
            "missing required --scenario <path> argument".to_string()
        }
        RunnerError::UnknownArg(arg) => format!("unknown argument: {arg}"),
        RunnerError::ReadScenario { path, source } => {
            format!("failed to read scenario {}: {source}", path.display())
        }
        RunnerError::ParseScenario { path, source } => {
            format!(
                "failed to parse scenario {} as canonical json: {source}",
                path.display()
            )
        }
        RunnerError::UnexpectedEventKind { kind } => {
            format!("unsupported event kind in input scenario: {kind}")
        }
        RunnerError::UnexpectedExpectedKind { kind } => {
            format!("unsupported expected output kind in scenario: {kind}")
        }
        RunnerError::Mapping { source } => format!("target5->target10 mapping failed: {source:?}"),
        RunnerError::Mismatch { expected, actual } => {
            format!("output mismatch: expected={expected:?} actual={actual:?}")
        }
    }
}
