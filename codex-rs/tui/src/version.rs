/// The current Codex CLI version as embedded at compile time.
pub const CODEX_CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// User-visible identity for this CxLine build of the upstream Codex version.
///
/// Update checks and protocol metadata must continue to use [`CODEX_CLI_VERSION`].
pub const CODEX_BUILD_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "+cxline.1");

/// Version label used by UI renderers, fixed before layout in unit tests.
/// Keep release/update decisions on `CODEX_CLI_VERSION`, not this fixture.
#[cfg(not(test))]
pub(crate) const CODEX_DISPLAY_VERSION: &str = CODEX_BUILD_VERSION;
#[cfg(test)]
pub(crate) const CODEX_DISPLAY_VERSION: &str = "0.0.0";
