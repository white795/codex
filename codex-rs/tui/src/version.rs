/// The current Codex CLI version as embedded at compile time.
pub const CODEX_CLI_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Version label used by UI renderers, fixed before layout in unit tests.
/// Keep release/update decisions on `CODEX_CLI_VERSION`, not this fixture.
#[cfg(not(test))]
pub(crate) const CODEX_DISPLAY_VERSION: &str = CODEX_CLI_VERSION;
#[cfg(test)]
pub(crate) const CODEX_DISPLAY_VERSION: &str = "0.0.0";
