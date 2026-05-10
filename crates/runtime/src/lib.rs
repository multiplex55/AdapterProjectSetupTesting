//! Runtime orchestration for profile/app startup.

pub mod app_host;
pub mod app_profile;
pub mod diagnostics;
pub mod health;
pub mod provider_registry;
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
