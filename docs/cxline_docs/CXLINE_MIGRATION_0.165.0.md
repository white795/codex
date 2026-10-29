# CxLine migration to Codex 0.165.0

- Status: local validation complete (full run plus focused snapshot rechecks); commit/push/hosted CI pending
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
  acceptance were still pending at that checkpoint.
- Foundation is committed as `bc72b2dbdb268cf2bcdd27f6ea6934fbab360ec3`.
- The configuration UI restores 21 published files: editor/dialog implementations, 50 behavioral
  tests, and six layout snapshots. All 21 files match `cxline-v0.161.0.1` exactly, and recursive
  module formatting passes. The module was still test-only at that checkpoint.
- Combined foundation/UI validation passes: 82 tests pass with no failures or ignored tests;
  5,831 unrelated tests are filtered out. All ten published snapshots pass unchanged. The cached,
  offline, single-job `dev-small` build takes 4m 33s, and the tests finish in 0.21s. Only the
  runtime loader `load_saved` was still unused at that checkpoint; no lint exemptions were added.
- Configuration UI is committed as `64872e2e2ddf8a5e4fc1b39ac6cef32022ad5ef3`.
- Runtime integration restores `/cxline`, alternate-screen editor handoff, explicit saved-config
  selection, live model/reasoning/Fast, directory/context/usage data, and asynchronous Git caching
  with cwd/request guards. Terminal-title selections and official footer fallback are preserved.
  The module participates in normal builds; input visuals were still upstream at that checkpoint.
- Reuse the published runtime contracts rather than introducing new behavior. Adapt the Git
  request to `WorkspaceCommand::new(argv, cwd)` and assert the current runner's five-second timeout
  and capped-output flag; the retired optional cwd/output-cap fields are not reintroduced.
  Of the 15 restored runtime/test/snapshot files, 13 match the published source exactly; only the
  Git adapter and its request-shape test require API adaptation. All 24 changed Rust files pass
  formatting checks.
- Runtime validation passes: 277 tests pass with no failures or ignored tests, covering 128 CxLine
  tests and 149 related official status-line, terminal-title, slash-command, and pager tests;
  5,682 unrelated tests are filtered out. The offline, single-job `dev-small` build takes 4m 47s
  without compiler warnings, and the tests finish in 3.34s. The ten prior CxLine snapshots and
  four live-footer snapshots pass unchanged. No snapshot regeneration or lint exemptions are needed.
- Runtime checks cover saved-config opt-in/fallback, child-dialog and page exit, draft restoration
  in inline and owned sessions, live token notifications, effective catalog/account Fast state,
  official usage-window selection, Git caching/refresh/failure handling, stale cwd/request results,
  and terminal-title coexistence. Composer visuals and the later build/release gates were still
  pending at that checkpoint.
- Runtime integration is committed as `89a8e53496b21d47f3fe8718dcdca74f6f8e7057`.
- Composer visuals restore the published transparent top/bottom rules and normal `❯` marker while
  preserving shell `!`, Max `›`, Ultra `»`, Reserve color, layout/cursor geometry, and input state.
  Astra stars blend against the terminal background while leaving transparent cells unchanged.
  The seven changed Rust files pass formatting checks; four published visual contracts are restored,
  with additional Persistent/custom-effort cases in the prompt matrix.
- Adapt the former in-composer WSL test fixture to upstream's new `footer_state.rs` location.
  Generic test snapshots use the non-WSL help layout; real WSL detection remains unchanged.
  Regenerate affected snapshots from 0.165 rather than copying previous release snapshots.
- Composer verification passes: 1,341 tests pass with no failures, one upstream Windows-only AltGr
  test is ignored on Linux, and 4,621 unrelated tests are filtered out. The single-job offline
  `dev-small` build takes 2m 24s; the snapshot-generation run finishes in 22.76s. Review all 143
  changed snapshots, then repeat the same scope with `INSTA_UPDATE=no`: cached preparation takes
  1.93s and the tests pass in 22.83s without updating snapshots.
- Snapshot review confirms unchanged text, recorded rectangles/cursors, and row counts except one
  previously cropped final blank row now displaying its border. Differences are limited to rules,
  prompt glyphs, transparent backgrounds, and starfield placement on the remaining blank cells.
  Fixed-width snapshot rows retain their required trailing spaces; source/document whitespace
  checks and snapshot checks allowing those row endings pass. No pending snapshots are left.
- The 14 existing CxLine snapshots and its configuration/runtime implementation remain unchanged.
  Other application/transcript snapshots are outside this focused scope and will be reviewed in
  the final full TUI regression. Build identity, CI/release adaptation, strict Clippy, non-test CLI
  checks, and manual acceptance remain later gates.
- Composer visuals are committed as `1d2cbba6ba7873df64df765f3830f347d4d70cd9`.
- Build identity restores the published CLI/help contracts and separates upstream compatibility
  version `0.165.0` from display identity `0.165.0+cxline.1`. Headers, status, update banners, and
  the optional official version item use the display path; machine compatibility inputs retain
  the upstream version. TUI unit fixtures use `0.0.0` before layout, and snapshot sanitizers follow
  the display fixture, including the new Daybreak replay case.
- Restore `just test-tui-unit` and both fork workflows with upstream's pinned Rust 1.99.1 action.
  Retain WSL/Linux GNU x86_64, tag/binary revision checks, finalized bwrap digest ordering, complete
  package staging, and tag-only publication. The current upstream builder validates required
  resources without a patched zsh. Other upstream workflows, daemon policy, and updater policy
  remain unchanged.
- Restore the five historical migration/release records byte-for-byte from `cxline-v0.161.0.1`.
  Update the maintenance guide and index for this branch and its separate remaining gates.
  Restored workflow definitions do not imply hosted CI, publication, package-download verification,
  or installation has run.
- Build-identity checkpoint verification passes 78 TUI unit tests covering version fixtures,
  update comparison/prompt behavior, and status cards; 5,888 unrelated tests are skipped.
  The restored `just test-tui-unit` forwards arguments and normalizes the terminal correctly.
  The offline single-job `dev-small` build takes 2m 40s, and nextest finishes in 3.414s with
  `INSTA_UPDATE=no`. No snapshots change and no pending snapshot files remain.
- Nextest resolves full workspace metadata before filtering tests. Its first offline attempt
  requires two additional locked cache entries (`deno_core_icudata` 0.78.0 and `v8` 152.2.0).
  Populate those caches separately, then rerun offline; Cargo manifests and the lockfile stay
  unchanged. The three version contracts pass. The two CLI parser contracts also pass: output is
  `codex-cxline 0.165.0+cxline.1`, and help keeps the `codex` invocation. The single-job offline
  `dev-small` CLI test build takes 17m 00s, including dependencies not used by the focused TUI build;
  the two tests finish in 0.01s, with 313 unrelated tests filtered out. Normal-mode TUI code is
  compiled as a dependency of those tests; the separate non-test CLI check remains a final gate.
- Package validation passes all 27 existing upstream Python tests without adding dependencies.
  Parse both fork workflow YAML files and syntax-check ten shell steps. Execute seven tag cases
  and the actual artifact-staging shell against fixture executables and the current package builder,
  with Cargo/network tripwires: complete archive layout, display identity, bwrap byte preservation,
  checksums, and revision-mismatch rejection pass. This exercises staging without a Rust release
  build and is not acceptance of a real release package.
- Both fork workflows differ from the published source only in the pinned Rust toolchain action,
  matching upstream 0.165.0. Formatting for all 19 changed Rust files and the justfile passes.
  All 30 changed/new files pass source/document whitespace and final-newline checks; local document
  links resolve, and all five historical records match the source tag exactly. The focused TUI,
  CLI, and package suites pass 107 tests in total. Both Rust builds finish without warnings.
  No local release-profile build, real configuration change, installation change, or Git write
  occurs at this checkpoint. Final full TUI regression and strict Clippy remain pending.
- Build identity and WSL workflow restoration are committed as
  `d61da29d1f06ab7c784d28d65293408e3ea0c644`; the final local gate starts from a clean worktree.
- Reproduce the old voice-prompt and Reserve-palette assertions with the existing test binary:
  both fail for the intended visual differences (`›` lookup versus `❯`, gray versus `Reset`).
  Adapt nine test-source files using the published visual contracts, preserving 0.165.0's
  `f9 mute` hint and current model/session/permission expectations. Cover full composer borders,
  draft and Vim restoration, question-queue slicing, voice prompts, and transparent Reserve
  backgrounds without changing production code. Keep history/menu markers and Max/Ultra intact.
- Run the outer App/ChatWidget/startup snapshot scope with updates enabled, then review its
  differences before a full TUI run with `INSTA_UPDATE=no`. This generation pass runs 2,299 tests
  in 455.945s: 2,298 pass and one new 0.165.0 inline snapshot needs the visible border update.
  Preserve that subagent prompt's text and menu choices while updating the expected surrounding
  composer; the ordinary menu `›` remains unchanged.
- Review all 156 changed snapshots. Their text remains equivalent after the accepted rule,
  normal-prompt, and three Astra starfield changes; quoted grids keep their dimensions and
  recorded cursor text is unchanged. Metadata follows current upstream test paths/normalizers
  and the restored display fixture. Restore the full four-row multiline Vim sample rather than
  accepting a crop that hides `saved line`. Ten test-source files now carry visual expectations;
  production code and the 14 CxLine snapshots remain unchanged. No pending snapshots remain.
- Strict scoped Clippy passes with `--no-deps -D warnings`, using offline single-job `dev-small`
  in 12m 36s without warnings. Full workspace Rust formatting passes; stable rustfmt's
  nightly-only import-option notices are expected. Non-test CLI checking also passes without
  warnings in 7m 38s.
- The full TUI run uses `INSTA_UPDATE=no` and a new isolated `CODEX_HOME`. Cached compilation
  takes 1m 25s; all 5,962 selected tests run in 803.310s. Of these, 5,960 pass and two report old
  composer snapshots outside the outer-screen generation scope: shortcut clipboard paste and
  mapped Shift+Enter rendering. Both differences contain only the expected rules/prompt and
  current snapshot metadata; pasted text, line breaks, and fixed-grid geometry are preserved.
- Review and regenerate those two snapshots, then rerun exactly the two failed tests with
  `INSTA_UPDATE=no`: both pass in 0.741s after 0.80s cached preparation. All 5,962 selected cases
  are verified across the full run and this focused recheck; the original full run was not green
  and is not presented as a second clean full run. Preserve its report separately because the
  focused recheck replaces nextest's default JUnit output. Hosted CI must still run the complete
  suite for the exact final commit.
- Four pre-existing entries are skipped by the normal Linux suite: `startup_typeahead_pty_child`,
  `caught_panic_pty_child`, and `owned_screen_lifecycle_child` are isolated helpers invoked by
  their parent tests; `altgr_ctrl_alt_char_inserts_literal` applies only on Windows. Add no new
  ignore or lint-allow rules.
- The final checkpoint changes ten test-source files, 158 reviewed snapshots, and maintenance
  documentation. Source and the earlier 156 snapshots remain byte-identical through the full run
  and focused rechecks; only the two additionally reviewed snapshots change after that run.
  Production implementation, Cargo files, workflow policy, real configuration, installation, and
  this repository's Git history/index/refs are untouched. No local release build runs.

The foundation and UI reuse the published behavior contracts without changing production behavior.
Run their focused compatibility checks from `codex-rs`, using Rust 1.99.1, an isolated `CODEX_HOME`,
normalized terminal hints, and the verified V8 archive/bindings selected by upstream's
`setup-rusty-v8` action. With the dependencies cached, use:

```bash
CARGO_BUILD_JOBS=1 INSTA_UPDATE=no cargo test --profile dev-small --locked --offline \
  -p codex-tui --lib statusline:: -- --test-threads=1
```

The runtime checkpoint additionally covers the related upstream paths in the same invocation:

```bash
CARGO_BUILD_JOBS=1 INSTA_UPDATE=no cargo test --profile dev-small --locked --offline \
  -p codex-tui --lib -- statusline:: cxline status_line terminal_title \
  slash_command:: pager_overlay:: --test-threads=1
```

The composer checkpoint includes all bottom-pane input, paste, Vim, prompt and effect tests:

```bash
CARGO_BUILD_JOBS=1 INSTA_UPDATE=no cargo test --profile dev-small --locked --offline \
  -p codex-tui --lib -- bottom_pane:: statusline:: cxline status_line terminal_title \
  slash_command:: pager_overlay:: --test-threads=1 --quiet
```

Rust 1.99.1 is installed alongside 1.95.0; the default toolchain remains 1.95.0. Foundation module
formatting, Cargo workspace metadata, the semantic lockfile audit, and copied-file identity checks
pass. The 23 copied source/test/snapshot files match the published source exactly; only the module
entry is narrowed to this checkpoint. The single-job cold build took 26m 21s; the focused tests
finished in 0.09s. That foundation build reported two expected dead-code warning groups for
`load_saved`, `get_segment_config_mut`, and `THEME_NAMES`; the UI now uses the latter two interfaces.
At that foundation checkpoint, strict Clippy, the full TUI regression, and the non-test CLI
check were still pending.

The build-identity checkpoint uses the same isolated environment, from `codex-rs`:

```bash
CARGO_BUILD_JOBS=1 INSTA_UPDATE=no just test-tui-unit --cargo-profile dev-small --offline \
  --test-threads 1 --retries 0 --failure-output final --status-level fail \
  -E 'test(/^version::tests::/) | test(/^update_versions::tests::/) | test(/^update_prompt::tests::/) | test(/^status::/)'

CARGO_BUILD_JOBS=1 cargo test --profile dev-small --locked --offline -p codex-cli --bin codex -- \
  tests::cli_version_identifies_the_cxline_build tests::cli_help_keeps_the_codex_command_name \
  --test-threads=1 --quiet

CODEX_REPO_ROOT="$(pwd)/.." python3 -m unittest discover -s ../scripts/codex_package -p 'test_*.py'
```

The final local gate uses the same Rust/V8 setup and cached locked dependencies, from `codex-rs`:

```bash
CARGO_BUILD_JOBS=1 just clippy --profile dev-small --locked --offline \
  -p codex-tui --lib --no-deps -- -D warnings
CARGO_BUILD_JOBS=1 cargo check --profile dev-small --locked --offline -p codex-cli --bin codex

cxline_test_home="$(mktemp -d /tmp/cxline-tui-check.XXXXXX)"
CODEX_HOME="$cxline_test_home" CARGO_BUILD_JOBS=1 INSTA_UPDATE=no \
  just test-tui-unit --cargo-profile dev-small --offline --test-threads 1 --retries 0 \
  --failure-output final --status-level fail

cargo fmt -- --config imports_granularity=Item --check
```

The snapshot-only follow-up reruns the two failed cases with updates disabled:

```bash
CODEX_HOME="$cxline_test_home" CARGO_BUILD_JOBS=1 INSTA_UPDATE=no \
  just test-tui-unit --cargo-profile dev-small --offline \
  --test-threads 1 --retries 0 --failure-output final --status-level fail \
  -E 'test(/^clipboard_copy::worker::tests::right_click_paste_uses_normal_input_and_discards_stale_reads$/) | test(/^tui::windows_key_sequence::tests::mapped_shift_enter_renders_newlines_in_the_composer$/)'
```

## Next checkpoint

The final local checkpoint is ready for maintainer review and commit. Suggested subject:
`test(tui): 完成 0.165.0 CxLine 视觉适配与本地验收`.
Push `cxline/0.165.0` and verify `cxline-ci` for that exact commit. Only a green hosted candidate
proceeds to the planned `cxline-v0.165.0.1` tag. Publication, downloaded-package checksum/layout
verification, isolated and existing-config WSL acceptance, and installation remain separate gates.
Keep the installed 0.161.0 client and real configuration unchanged during development.
