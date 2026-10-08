# CxLine migration to Codex 0.159.2

- Status: release preparation
- Base: `rust-v0.159.2` (`ff6aec96948b70d94983af2641a6b67c94faeff5`)
- Target branch: `cxline/0.159.2`
- Summary: pending

## Progress

- Foundation complete: legacy configuration parsing, built-in themes, segment collection, pure
  rendering, and snapshots pass on Codex 0.159.2.
- Storage and text editing complete: side-effect-free loading, atomic saves, portable theme-name
  validation, and Unicode-safe name/separator dialogs pass on Codex 0.159.2.
- Icon selection complete: Plain/Nerd Font lists, per-style selection memory, custom Unicode input,
  viewport clipping, and snapshots pass on Codex 0.159.2.
- Color picker state complete: 16/256-color navigation, RGB/HEX editing, field cycling, and
  selection preservation pass on Codex 0.159.2.
- Color picker rendering complete: popup layout, palette selection, viewport clipping, and compact
  mode snapshots pass on Codex 0.159.2.
- Configuration editor state complete: draft and exit baselines, theme selection, panel navigation,
  segment ordering, and field activation pass on Codex 0.159.2.
- Configuration editor rendering complete: live preview, theme and segment panels, responsive help,
  viewport clipping, focus styles, and wide/narrow snapshots pass on Codex 0.159.2.
- Configuration editor interactions complete: keyboard routing, lifecycle commands, child-dialog
  priority, commit/cancel behavior, Unicode input, and overlay rendering pass on Codex 0.159.2.
- Configuration editor persistence complete: theme selection/reset, config and theme writes,
  save-as no-clobber behavior, exit baselines, and failure reporting pass on Codex 0.159.2.
- Runtime configuration entry complete: `/cxline` dispatch, alternate-screen overlay lifecycle,
  redraw handling, persistence, and backtrack isolation pass on Codex 0.159.2.
- Live base footer integration complete: explicitly saved and enabled configurations render model,
  directory, and context data through the current composer status surface; missing or disabled
  configurations preserve the official status line.
- Live rate-limit integration complete: CxLine reuses the official Codex 5-hour and weekly window
  selection, updates with account-usage snapshots, and displays the localized weekly reset label.
- Live asynchronous Git integration complete: one bounded porcelain-v2 probe runs through the
  workspace-command boundary, stale cwd/request results are rejected, and official terminal-title
  Git refreshes remain independent.
- Build identity and fork automation complete: production binaries report
  `0.159.2+cxline.1`; the WSL/Linux quality gate and package-release workflow are restored without
  modifying upstream workflows.
- [Hosted branch CI](https://github.com/white795/codex/actions/runs/37718380023) passed for
  `c671644f35e5c7e7c6c48a5ab1b92b48c4f5865d`: formatting, locked CLI check, strict scoped TUI
  Clippy, and 5,647 TUI tests passed with 4 skipped.
- Release-tag test isolation complete: TUI unit tests use the upstream source-build identity
  `0.0.0`, while production update checks and protocol metadata retain the real Cargo version.
- Maintenance and historical release documentation restored for the release candidate.
- Next: build and inspect the complete 0.159.2 release package, run isolated and existing-config
  WSL checks, then publish `cxline-v0.159.2.1` if those checks pass.

## Goal

Reapply the maintained CxLine status bar, its complete configuration UI, and the existing input
visuals on top of the official Codex 0.159.2 release while preserving upstream behavior.

## Scope

- Keep the CxLine configuration format under `$CODEX_HOME/cxline` compatible with the 0.155.1
  custom release.
- Keep `/cxline`, live preview, theme editing, persistence, and runtime footer switching.
- Keep the previously selected composer/input visual changes.
- Keep the WSL/Linux x86_64 CI and release path and the `+cxline.1` build identity.
- Do not add translation, the Cometix CJK cursor, thread deletion, workflow deletion, or
  multi-platform release builds.

## Tasks

1. Port and verify the standalone CxLine configuration, rendering, storage, and editor modules.
2. Adapt `/cxline`, overlay events, persistence, and runtime footer integration to 0.159.2.
3. Reapply the selected composer/input visuals against the current footer and input state machines.
4. Restore build identity, fork CI/release workflows, and maintenance documentation.
5. Run focused tests, TUI regression, formatting/lint, build, and WSL manual validation.

Tasks 1–4 and the automated portion of task 5 are complete. Release packaging and manual WSL
validation remain open; no 0.159.2 release or installation is claimed by this document.

## Acceptance

- A missing saved CxLine configuration leaves the official status line active.
- Saving from `/cxline` switches the footer immediately and survives restart.
- Existing CxLine themes and custom TOML configuration still load and render correctly.
- Official 0.159.2 status surfaces and terminal-title behavior remain intact when CxLine is inactive.
- The branch builds and its scoped automated checks pass before release packaging.

## Risks / dependencies

- Official status-line and composer code changed substantially; integration must use current APIs
  instead of copying central 0.155.1 files.
- UI changes require reviewed `insta` snapshots.
- Official release tags carry a real Cargo version while upstream source snapshots are authored
  against `0.0.0`; keep the test-only version boundary in `tui/src/version.rs` when upgrading.
- Full snapshot runs on WSL render the platform-specific image-paste shortcut as `ctrl+alt+v`,
  whereas the Ubuntu CI snapshots use `ctrl+v`; do not accept those incidental WSL snapshot files.
- Final installation remains WSL/Linux x86_64 only.
