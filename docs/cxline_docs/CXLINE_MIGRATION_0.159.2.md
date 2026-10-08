# CxLine migration to Codex 0.159.2

- Status: complete and archived on 2026-10-08
- Base: `rust-v0.159.2` (`ff6aec96948b70d94983af2641a6b67c94faeff5`)
- Target branch: `cxline/0.159.2`
- Summary: [0.159.2 release handoff](CXLINE_RELEASE_0.159.2.md)

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
- Initial build identity and fork automation complete: the published `.1` binary reports
  `0.159.2+cxline.1`; the WSL/Linux quality gate and package-release workflow are restored without
  modifying upstream workflows.
- [Hosted branch CI](https://github.com/white795/codex/actions/runs/37718380023) passed for
  `c671644f35e5c7e7c6c48a5ab1b92b48c4f5865d`: formatting, locked CLI check, strict scoped TUI
  Clippy, and 5,647 TUI tests passed with 4 skipped.
- Release-tag test isolation complete: TUI unit tests use the upstream source-build identity
  `0.0.0`, while production update checks and protocol metadata retain the real Cargo version.
- `0.159.2+cxline.1` published from `cxline-v0.159.2.1` at
  `4582156b97a60b92af29de2628dedd8a93f1249b` and was accepted with the existing WSL
  configuration.
- The `.2` follow-up implementation landed in `0ade5fb25`: it restores the transparent composer,
  horizontal borders and `❯` prompt; preserves Astra sparkles on transparent cells; and appends
  `fast` to the CxLine model segment only when the official effective Fast-state predicate is true.
- The `.2` build identity was finalized as `0.159.2+cxline.2` without changing the upstream
  compatibility version used by update checks and protocol metadata.
- The `.2` tag commit passed
  [branch CI](https://github.com/white795/codex/actions/runs/37741875025), and the
  [release workflow](https://github.com/white795/codex/actions/runs/37746114670) published
  `cxline-v0.159.2.2` successfully.
- The downloaded archive matched `SHA256SUMS`, reported `0.159.2+cxline.2`, contained the complete
  package layout, and was installed under the versioned WSL directory. `current`, `codex`, and `cx`
  now resolve to `.2`; `.1` remains available for rollback.
- Manual acceptance confirmed the transparent borders and Fast display. Max `›` and Ultra `»` are
  retained official effort-tier prompt glyphs; the normal prompt remains `❯`.

## Goal

Reapply the maintained CxLine status bar, its complete configuration UI, and the existing input
visuals on top of the official Codex 0.159.2 release while preserving upstream behavior.

## Scope

- Keep the CxLine configuration format under `$CODEX_HOME/cxline` compatible with the 0.155.1
  custom release.
- Keep `/cxline`, live preview, theme editing, persistence, and runtime footer switching.
- Keep the selected composer/input visuals, including the transparent bordered composer and normal
  `❯` prompt, while retaining official Max/Ultra prompt accents.
- Show the official effective Fast-mode state in CxLine without changing service-tier resolution.
- Keep the WSL/Linux x86_64 CI and release path and use the `+cxline.2` build identity for this
  follow-up release.
- Do not add translation, the Cometix CJK cursor, thread deletion, workflow deletion, or
  multi-platform release builds.

## Tasks

1. Port and verify the standalone CxLine configuration, rendering, storage, and editor modules.
2. Adapt `/cxline`, overlay events, persistence, and runtime footer integration to 0.159.2.
3. Reapply the selected composer/input visuals against the current footer and input state machines.
4. Restore build identity, fork CI/release workflows, and maintenance documentation.
5. Run focused tests, TUI regression, formatting/lint, build, and WSL manual validation.
6. Restore the composer visuals lost during the 0.159.2 migration and expose effective Fast mode in
   the CxLine model segment.

Tasks 1–5 completed for the published `.1` release. Task 6 and all hosted CI, release packaging,
checksum, installation, and representative WSL acceptance work completed for `.2`.

## Acceptance

- A missing saved CxLine configuration leaves the official status line active.
- Saving from `/cxline` switches the footer immediately and survives restart.
- Existing CxLine themes and custom TOML configuration still load and render correctly.
- Official 0.159.2 status surfaces and terminal-title behavior remain intact when CxLine is inactive.
- Fast-capable sessions render `<model> · <reasoning> fast`; other sessions omit `fast`.
- The composer uses a transparent background, horizontal top and bottom borders, and the normal
  `❯` prompt without breaking Astra sparkles or official Max/Ultra accents.
- The branch builds and its scoped automated checks pass before release packaging.

## Risks / dependencies

- Official status-line and composer code changed substantially; integration must use current APIs
  instead of copying central 0.155.1 files.
- UI changes require reviewed `insta` snapshots.
- Official release tags carry a real Cargo version while upstream source snapshots are authored
  against `0.0.0`; keep the test-only version boundary in `tui/src/version.rs` when upgrading.
- Generic snapshots keep the non-WSL `ctrl+v` shortcut deterministic under tests; explicit WSL
  shortcut tests cover `ctrl+alt+v`, and production WSL detection is unchanged.
- The `.2` release and installation remain WSL/Linux x86_64 only.
