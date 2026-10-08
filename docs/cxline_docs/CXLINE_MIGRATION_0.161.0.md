# CxLine migration to Codex 0.161.0

- Status: in progress
- Base: `rust-v0.161.0` (`979011409de0a60b52f179721948e65531d26144`)
- Branch: `cxline/0.161.0`
- Source: `cxline-v0.159.2.2` (`be99489eaeaaab6a0e3cef2bbca69a1b67a01e10`)
- Planned display version: `0.161.0+cxline.1`
- Planned tag: `cxline-v0.161.0.1`
- Summary: pending

## Goal

Reapply the accepted CxLine customization from 0.159.2 on the official 0.161.0 tag,
keeping the existing configuration format and preserving current upstream behavior.

## Scope

- Keep the full configuration UI, themes, preview, persistence, and saved enable setting.
- Keep model and reasoning, effective Fast state, directory, context, official rate limits,
  and asynchronous Git data in the CxLine footer.
- Keep transparent composer borders and the normal `❯` prompt, Astra sparkles, and the
  official Max `›` and Ultra `»` accents.
- Preserve the official status line when CxLine is missing, disabled, or unusable.
- Keep the existing WSL/Linux x86_64 release path. Git writes remain maintainer actions.
- Do not add translation, CJK cursor changes, thread-deletion changes, or new platforms.

## Tasks

1. Align the release-tag internal lockfile versions and port standalone configuration,
   themes, segments, rendering, and storage with their existing tests.
2. Port text, icon, color, and configuration editors with their persistence and UI tests.
3. Adapt command dispatch, overlays, live footer data, asynchronous Git, and composer visuals
   against current APIs rather than replacing upstream central files.
4. Restore build identity, deterministic test entry, fork workflows, and maintenance records;
   verify hosted branch CI, tag release, checksum, and WSL installation in separate stages.

## Progress

- Baseline matches the official tag; Rust remains 1.95.0.
- The lockfile correction changes only 160 source-free workspace package versions from
  0.0.0 to 0.161.0. All 1,314 official external dependency records remain unchanged.
- Foundation and storage complete: legacy TOML, nine themes, five segments, pure rendering,
  safe saves, and explicit saved-config loading pass the focused suite on 0.161.0.
  The model segment retains effective Fast-state input and the accepted reasoning spacing.
  They remain test-only until runtime integration; `/cxline` is not available yet.
- Next: port text, icon, color, and configuration editors before runtime wiring.

## Verification

- Locked Linux dependency resolution passed after downloading uncached upstream dependencies.
- The focused suite passed 32 tests, with 5,633 unrelated tests skipped. This behavior-preserving
  port reuses the existing regression tests and adds a Fast-display matrix for model/effort data.
  Run from the repository root with Cargo and just on PATH:

  ```bash
  env -u TERM_PROGRAM -u TERM_PROGRAM_VERSION -u NO_COLOR -u TMUX -u TMUX_PANE \
    -u STY -u ZELLIJ -u ZELLIJ_SESSION_NAME TERM=xterm-256color COLORTERM=truecolor \
    CARGO_BUILD_JOBS=1 CARGO_NET_OFFLINE=true just test --locked -p codex-tui --lib \
    -E 'test(statusline)' --test-threads 1 --retries 0 --failure-output final --status-level fail
  ```

- All four imported rendering snapshots passed unchanged. Rust formatting and `git diff --check`
  passed; the imported Rust and migration files also passed whitespace and final-newline checks.
- Strict Clippy, full TUI regression, runtime behavior, hosted CI, and release checks remain for
  later stages. Unused editor/runtime helpers (`load_saved`, `get_segment_config_mut`, and
  `THEME_NAMES`) currently produce two warnings while those callers are not yet ported.
- No local release build, active configuration change, installation change, or Git write occurred.

## Acceptance

- Existing CxLine TOML, custom themes, and colors remain compatible; reads do not create or
  overwrite files, and save-as does not replace an existing theme.
- Saving an enabled configuration switches the footer and survives restart; disabling it
  preserves the official 0.161.0 status surfaces and terminal-title behavior.
- Fast sessions show `<model> · <reasoning> fast`; other sessions omit `fast`.
- Transparent input visuals preserve paste, disabled, Bash, Max, Ultra, and sparkle behavior.
- Focused checks pass before each agreed commit point, followed by final hosted CI.
- The release identity, package contents, checksum, and representative WSL use pass before
  changing the installed version. No local release build is required by default.

## Risks and dependencies

- The official tags diverge through release backports; transfer the customization delta,
  not the old branch wholesale.
- Startup, status surfaces, Daybreak indicators, and daemon behavior changed upstream.
  Preserve those paths and derive Fast mode from the current official predicate.
- Keep source-build test version and WSL shortcut fixtures deterministic without changing
  production version, platform detection, configuration, or daemon settings.
- New upstream dependencies may require cache downloads before locked tests can run.
