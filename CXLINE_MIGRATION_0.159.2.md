# CxLine migration to Codex 0.159.2

Status: active  
Base: `rust-v0.159.2` (`ff6aec96948b70d94983af2641a6b67c94faeff5`)  
Target branch: `cxline/0.159.2`  
Summary: pending

## Progress

- Foundation complete: legacy configuration parsing, built-in themes, segment collection, pure
  rendering, and snapshots pass on Codex 0.159.2.
- Next: configuration storage and the reusable editor/input components.

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
- `Cargo.lock` and generated version snapshots are deferred until build identity is restored.
- Final installation remains WSL/Linux x86_64 only.
