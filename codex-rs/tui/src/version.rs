/// The current Codex CLI version as embedded at compile time.
#[cfg(not(test))]
pub const CODEX_CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Unit tests model an upstream source checkout so snapshots do not depend on
/// the release tag used as the branch base.
#[cfg(test)]
pub const CODEX_CLI_VERSION: &str = "0.0.0";

/// User-visible identity for this CxLine build of the upstream Codex version.
///
/// Update checks and protocol metadata must continue to use [`CODEX_CLI_VERSION`].
pub const CODEX_BUILD_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "+cxline.2");

/// Version label used by UI renderers, fixed before layout in unit tests.
/// Keep release/update decisions on `CODEX_CLI_VERSION`, not this fixture.
#[cfg(not(test))]
pub(crate) const CODEX_DISPLAY_VERSION: &str = CODEX_BUILD_VERSION;
#[cfg(test)]
pub(crate) const CODEX_DISPLAY_VERSION: &str = "0.0.0";
