//! Runtime orchestration for profile/app startup.

pub mod app_host;
pub mod app_profile;
pub mod diagnostics;
pub mod effects;
pub mod health;
pub mod provider_registry;
pub mod replay_host;
pub mod startup;

pub use app_host::AppHost;
pub use app_profile::{AppProfile, CommType, InputMode, ProfileId};
pub use diagnostics::{FallbackStatus, StartupDiagnostics};
pub use health::{HealthReport, HealthState};
pub use provider_registry::{
    CapabilityKind, ProviderCandidate, ProviderError, ProviderRegistry, ProviderResolution,
    ProviderSourceSpec,
};
pub use startup::{StartupConfig, StartupError, StartupResult};

pub use effects::{dispatch_target5_to_target10_effects, EffectDispatchError, EffectDispatchState};
pub use replay_host::{
    map_target5_statuses_to_target10_commands, run_target5_to_target10_replay_flow,
    ReplayFlowError, ReplayFlowSummary,
};
