# CxLine migration to Codex 0.165.0

- Status: configuration UI complete (82 focused tests passed; runtime and release pending)
- Base: `rust-v0.165.0` (`360319d8dee62d87e2d59fc0c21805d79519cfed`)
- Branch: `cxline/0.165.0`
- Source: `cxline-v0.161.0.1` (`b8c56678d1f1c9277a58d50c615d165be3d3da9e`)
- Planned display version: `0.165.0+cxline.1`
- Planned release tag: `cxline-v0.165.0.1`
- Summary: pending

## Goal

Reapply the accepted 0.161.0 CxLine customization on the official 0.165.0 release,
preserving existing configuration and upstream behavior outside the customization.
There is no intermediate 0.164 customization release.

## Scope

- Preserve the full CxLine configuration UI, themes, preview, persistence, and saved enable setting.
- Preserve model/reasoning/Fast, directory, context, official usage windows, and asynchronous Git.
- Preserve transparent composer borders, normal `❯`, Astra sparkles, and official Max/Ultra accents.
- Keep the official status line when CxLine configuration is missing, disabled, or unusable.
- Keep WSL/Linux x86_64 GNU as the fork target and leave upstream workflows as reference.
- Exclude translation, CJK cursor changes, thread deletion, new platforms, and a new master-switch UI.
- Leave real client configuration, the installed version, daemon policy, and updater policy untouched.

## Tasks

1. Align release-tag workspace lockfile versions and restore standalone configuration, themes,
   pure segments/rendering, and storage with their published regression tests.
2. Restore the full configuration editor and its text, icon, color, preview, and persistence tests.
3. Adapt `/cxline`, overlay lifecycle, saved-config loading, live footer data, and asynchronous Git
   against current App and ChatWidget APIs, preserving official fallback and terminal-title behavior.
4. Adapt transparent composer visuals to the current input/footer modules and preserve upstream
   keymap, input, session, permission, and supported reasoning-override behavior.
5. Restore build identity and deterministic test fixtures; adapt the fork CI/release workflow to
   Rust 1.99.1 and the current complete-package layout without the retired patched zsh.
6. Complete relevant local checks and final TUI regression, maintainer commit/push, hosted CI for
   that exact commit, tag publication, checksum verification, isolated and existing-config WSL
   acceptance, and installation/rollback handoff as separate gates.

Git writes, branch changes, commits, tags, and pushes remain maintainer actions. Pause at small
reviewable commit checkpoints. Use the hosted tag workflow for release builds; do not start a
local release-profile build without an explicit diagnostic need.

## Acceptance

- Existing TOML and nine built-in themes retain their behavior; reads do not create or rewrite files.
- Only an explicitly saved, enabled main configuration selects CxLine; other cases use upstream.
- Model output retains `<model> · <effort> fast` when the effective official Fast predicate is true.
- Git probes remain bounded, asynchronous, cached, and guarded against stale cwd/request results.
- Composer input, normal/disabled/Bash and effort-tier prompts preserve their intended appearance.
- Targeted tests pass at each relevant checkpoint; the final candidate passes hosted branch CI
  before an immutable release tag is created. No new release or installation is implied by a port.

## Risks and dependencies

- Rust 1.99.1 is required by upstream; keep the existing 1.95.0 installation and default selection.
- App, widget initialization, commands, footer layout, and snapshots are high-churn integration seams.
  Source-only text conflict counts are not a compilation or behavioral compatibility guarantee.
- The official release lockfile still records workspace packages as 0.0.0. Change only verified
  workspace versions; preserve external dependency identities, checksums, and dependency records.
  Include implicit path-dependency workspace members reported by Cargo metadata, not only the
  explicit `workspace.members` list.
- Package checks must follow the new upstream layout rather than require the retired zsh resource.
- The shared daemon is independent of the CLI. Preserve an isolated/no-daemon acceptance path and
  the previous installation; this migration does not promise to resolve upstream daemon issues.

## Progress and verification

- The branch is clean at the official base before migration. Annotated tags are compared using
  their peeled commit identities, not their tag-object IDs.
- Workspace lockfile alignment changes 167 internal package versions from 0.0.0 to 0.165.0;
  all 1,308 sourced external records are unchanged. An isolated Cargo resolution confirmed five
  implicit workspace members omitted by the initial explicit-member audit; their manifests also
  inherit the release version, and no external dependency adjustment is required.
- The initial foundation restores the published configuration/storage and five pure segments,
  nine themes, 32 tests, and four rendering snapshots. Only the module declaration is enabled
  under `cfg(test)` until runtime integration; normal CLI behavior is unchanged at this checkpoint.
- Foundation validation passes: 32 tests pass with no failures, and 5,831 unrelated tests are
  filtered out. All four published rendering snapshots pass unchanged; there are no new snapshots.
  Runtime wiring, input visuals, build identity, fork CI, release publication, and manual
  acceptance remain pending on this branch.
- Foundation is committed as `bc72b2dbdb268cf2bcdd27f6ea6934fbab360ec3`.
- The configuration UI restores 21 published files: editor/dialog implementations, 50 behavioral
  tests, and six layout snapshots. All 21 files match `cxline-v0.161.0.1` exactly, and recursive
  module formatting passes. The module remains test-only; `/cxline` and runtime integration are
  the next checkpoint.
- Combined foundation/UI validation passes: 82 tests pass with no failures or ignored tests;
  5,831 unrelated tests are filtered out. All ten published snapshots pass unchanged. The cached,
  offline, single-job `dev-small` build takes 4m 33s, and the tests finish in 0.21s. Only the
  runtime loader `load_saved` remains unused; no lint exemptions are added.

The foundation and UI reuse the published behavior contracts without changing production behavior.
Run their focused compatibility checks from `codex-rs`, using Rust 1.99.1, an isolated `CODEX_HOME`,
normalized terminal hints, and the verified V8 archive/bindings selected by upstream's
`setup-rusty-v8` action. With the dependencies cached, use:

```bash
CARGO_BUILD_JOBS=1 INSTA_UPDATE=no cargo test --profile dev-small --locked --offline \
  -p codex-tui --lib statusline:: -- --test-threads=1
```

Rust 1.99.1 is installed alongside 1.95.0; the default toolchain remains 1.95.0. Foundation module
formatting, Cargo workspace metadata, the semantic lockfile audit, and copied-file identity checks
pass. The 23 copied source/test/snapshot files match the published source exactly; only the module
entry is narrowed to this checkpoint. The single-job cold build took 26m 21s; the focused tests
finished in 0.09s. That foundation build reported two expected dead-code warning groups for
`load_saved`, `get_segment_config_mut`, and `THEME_NAMES`; the UI now uses the latter two interfaces.
Strict Clippy, the full TUI regression, and the non-test CLI check remain later gates.

## Next checkpoint

The maintainer reviews and commits the complete configuration UI and its passing regression tests.
Suggested commit subject: `feat(tui): 迁移 0.165.0 CxLine 完整配置 UI`.
The next implementation checkpoint connects `/cxline` and its overlay lifecycle to runtime data.
Keep the installed 0.161.0 client and real configuration unchanged during development.
