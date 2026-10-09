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
- Configuration UI complete internally: Unicode name/separator dialogs, emoji/Nerd Font icons,
  16/256-color and RGB/HEX selection, preview, panel navigation, child-dialog routing, theme
  switching/reset, save/write/save-as, and exit baselines pass the focused suite on 0.161.0.
  All 21 UI files are reused unchanged from the published 0.159.2 customization.
- Runtime editor entry complete: `/cxline` is registered and dispatched to the alternate-screen
  editor. The published overlay and its five lifecycle tests are reused unchanged; shared pager
  routing restores input policy on exit and keeps transcript backtracking separate.
- CxLine modules are now included in normal builds. Widget startup loads the saved configuration
  without creating files; editor exit returns its existing save/theme baseline for subsequent
  openings. Missing or malformed main configuration does not opt in, and a disabled saved setting
  remains intact. Tests cover save/reopen, discarded drafts, child-dialog cancellation, and
  composer restoration in both inline and owned-screen sessions.
- Live base footer complete: explicitly saved, enabled CxLine renders the effective model and
  reasoning, Fast label, current directory, context tokens, and official Codex usage windows.
  Fast visibility uses the current official effective-tier/catalog/account path, including a
  catalog-default Fast tier whose request id is not `priority`. Seven published runtime tests
  and three live-data snapshots are reused unchanged.
- Missing, malformed, or disabled main configuration uses the official footer without modifying
  its item selection. Terminal-title selections and shared inputs remain on the upstream path;
  hidden official workspace-headline items do not start fetches or periodic footer refreshes
  while CxLine is active. Usage data keeps upstream window selection and reset formatting.
- Saving and closing the editor now applies CxLine to the live footer. App-level token
  notifications update it through the existing post-notification refresh, without changes to
  protocol handling.
- Asynchronous Git complete: branch, clean/dirty/conflicted worktree status, and ahead/behind
  come from the published bounded porcelain-v2 adapter through the active workspace runner.
  The adapter and its three tests are reused unchanged. Pending, successful, and failed lookups
  are cached instead of probing on each render. Directory changes and request identity reject
  stale results, including an old result after returning to the same directory or re-enabling Git.
- Git refresh follows the existing turn-completion and interruption entry, without changing replay
  behavior or suppressing the official terminal-title branch refresh. App events apply only the
  pending request and schedule a frame; disabling CxLine or its Git segment clears its lookup state.
- Composer visuals complete: the existing geometry now draws transparent dim top/bottom rules
  and the normal/disabled `❯` marker. Shell `!`, official Max `›` and Ultra `»`, Luna Reserve's
  yellow accent, effort animations, and the voice strip keep their existing precedence and behavior.
- Astra's published transparent-cell adapter is reused: stars blend against the terminal background
  without painting a cell background. Eligibility, protected placeholder/cursor, input cancellation,
  redraw gating, and the original 15-second deadline are unchanged. New tests cover transparent
  blending, prompt precedence, and clipped/offset areas without changing drafts or desired height.
- Composer snapshots preserve 0.161.0's compact key labels, MCP login help, fullscreen Plan cycle
  hint, and new blockquote/Vim paste scenarios. Only generic test builds skip host WSL detection;
  explicit WSL shortcut tests remain included, and production WSL detection is unchanged.
- Foundation, full editor UI, runtime handoff, live footer including Git, and composer visuals are
  connected. The installed version is unchanged.
- Build identity complete: `--version` reports `codex-cxline 0.161.0+cxline.1`, while help keeps the
  `codex` invocation. Startup/loading/session headers, `/clear`, `/status`, the update banner, and
  the optional official Codex-version footer item use the published customization's display path.
  Update comparisons, server compatibility notices, and protocol metadata keep the upstream
  version without the CxLine suffix; daemon and updater policies are unchanged.
- Deterministic test entry restored: source-checkout unit fixtures use `0.0.0` before layout, and
  UI snapshot sanitizers reference the display version. `just test-tui-unit` normalizes color and
  terminal hints, clears tmux/screen/Zellij hints, and forwards arguments to locked TUI unit tests.
  Ordinary `just test`, upstream recipes, and CI remain unchanged. Five new tests cover build
  identity, the two source-checkout fixtures, CLI version output, and CLI help invocation.
- Fork workflows restored unchanged from `cxline-v0.159.2.2`: branch pushes to `cxline/**` use the
  Ubuntu 24.04 quality gate, and pushed `cxline-v*` tags use the single GNU x86_64 package/release
  path. Existing action pins, permission boundaries, concurrency, tag/build/package identity
  validation, complete package inputs, and checksum artifacts are retained. Upstream workflows
  and shared setup/package scripts are unchanged.
- Maintenance records restored under `docs/cxline_docs/`: the index and guide distinguish the
  active 0.161.0 candidate from the accepted 0.159.2 installation. Four historical migration/release
  records are restored byte-for-byte from the previous branch. No 0.161.0 release handoff is created
  before publication. The installed `current` still resolves to `0.159.2+cxline.2`.
- Next: port remaining outer-screen snapshot deltas and related visual assertions, then complete
  non-test/strict/full-TUI checks before the push/hosted-CI and tag/release acceptance stages.

## Verification

- Locked Linux dependency resolution passed after downloading uncached upstream dependencies.
- The foundation checkpoint passed 32 tests; the internal UI checkpoint passed 82 tests after
  restoring 50 UI tests. The runtime-editor checkpoint passed 104 focused tests, with 5,628
  unrelated tests skipped: the prior 82, 17 restored or added editor-entry/configuration tests,
  and five existing slash-command tests. This behavior-preserving port reuses published
  regression coverage and adds checks for the current App and widget handoff.
- The live-base checkpoint ran 138 focused checks, including 19 upstream regression cases for
  usage windows, Fast/reasoning, status warnings, terminal-title sharing, Daybreak, and token
  reset behavior. Initially 137 passed; one new fixture omitted the App layer's explicit refresh
  after a token notification. After correcting only that fixture, a targeted rerun passed both
  it and an added App-level notification test (2/2). In total, 139 distinct scenarios have passed;
  the entire selection was not rerun at that checkpoint. The Git checkpoint below covers that
  selection again.
- The asynchronous-Git checkpoint first confirmed a minimal regression failed because an enabled
  Git segment scheduled zero probes instead of one. After integration, one concentrated run passed
  all 162 tests, with 5,599 unrelated tests skipped: 133 CxLine/slash-command checks and 29 upstream
  regressions. It adds 13 Git cases: seven reused adapter/runtime cases and six new cache,
  directory-return, disable/re-enable, turn lifecycle, and App-event checks. The four published
  widget-level Git tests are unchanged; their new module keeps the live-base tests separate.
  The reusable CxLine subset can be run from the repository root with Cargo and just on PATH:

  ```bash
  env -u TERM_PROGRAM -u TERM_PROGRAM_VERSION -u NO_COLOR -u TMUX -u TMUX_PANE \
    -u STY -u ZELLIJ -u ZELLIJ_SESSION_NAME TERM=xterm-256color COLORTERM=truecolor \
    CARGO_BUILD_JOBS=1 CARGO_NET_OFFLINE=true just test --locked -p codex-tui --lib \
    -E 'test(statusline) | test(cxline) | test(slash_command::tests::)' \
    --test-threads 1 --retries 0 --failure-output final --status-level fail
  ```

- All fourteen imported CxLine rendering snapshots passed with their original display content:
  four foundation, six UI, three live-base footer, and one Git footer snapshot. The Git snapshot
  changes only its source module path and assertion line to follow the separated Git test module.
  No unreviewed snapshot changes were accepted at those checkpoints.
- The composer checkpoint first confirmed the restored border test failed because the current
  renderer still left a blank border row. After the visual port, one concentrated offline run
  passed all 1,204 tests, with 4,561 unrelated tests skipped, using the filter
  `test(bottom_pane::) | test(statusline) | test(cxline) | test(slash_command::tests::)`.
  This includes all bottom-pane input/paste/Vim/history/question/effort/sparkle tests plus the
  CxLine regression subset. Four visual cases are restored or added; snapshots run with
  `INSTA_UPDATE=no`, so the passing run neither accepts updates nor leaves pending snapshot files.
- Reviewed 137 composer-related visual snapshots: 112 reuse the published display content against
  an unchanged official baseline, 21 combine that visual delta with current upstream changes,
  and four retain new 0.161.0 scenarios with only borders/prompt updated. The overlapping MCP
  popup snapshot keeps the new help text and its style boundary. Header metadata is retained;
  upstream input behavior and unrelated snapshots are not replaced.
- The build-identity checkpoint first confirmed the minimal version-fixture regression failed
  because a unit-test checkout still reported `0.161.0` instead of `0.0.0`. After restoring the
  version split, one concentrated offline run selected 102 checks across the TUI library, CLI
  library, and CLI binary. Initially 101 passed; the email-less ChatGPT status test could not bind
  its local wiremock server in the sandbox (`Operation not permitted`). Its existing configuration
  and credentials are temporary fixtures. An approved rerun of only that case outside the sandbox,
  using an isolated temporary Codex data directory, passed 1/1. All 102 distinct scenarios have therefore
  passed; the passing selection was not repeated.
- The identity selection covers five new regressions plus session/update-banner rendering,
  status cards, update-picker input and snapshots, and upstream version-comparison behavior.
  The CLI test build also compiles the normal, non-test TUI library. No snapshots were modified
  or accepted, and `INSTA_UPDATE=no` left no pending snapshot files. With Cargo and just on PATH,
  the concentrated selection is reproducible from the repository root:

  ```bash
  CARGO_BUILD_JOBS=1 CARGO_NET_OFFLINE=true INSTA_UPDATE=no \
    just test-tui-unit -p codex-cli --bin codex \
    -E 'test(version::tests::) | test(cli_version_identifies_the_cxline_build) | test(cli_help_keeps_the_codex_command_name) | (test(history_cell::tests::) & (test(session_) | test(update_available))) | test(status::) | test(update_prompt::tests::) | test(update_versions::tests::)' \
    --test-threads 1 --retries 0 --failure-output final --status-level fail
  ```

  The email-less ChatGPT case needs local loopback binding permission. For a sandbox-only failure,
  keep the same package/target selection and narrow the filter to
  `test(=status::tests::status_snapshot_shows_chatgpt_plan_without_email)`; no production code change
  or full-suite rerun is needed for that permission error.
- The workflow/documentation checkpoint used lightweight checks only; no Rust build or test
  suite was run. Both workflow files parse as YAML, all ten embedded shell blocks pass `bash -n`,
  and fourteen structure/policy assertions confirm the existing trigger, target, permissions,
  concurrency, pinned Actions, quality commands, package inputs, and identity guards.
- Fifteen isolated checks run the actual tag-validation and staging-identity snippets: nine tag
  cases and six mocked CLI-output cases cover the planned tag, invalid formats, upstream/revision
  mismatches, official/missing-suffix identities, and a non-publishing manual branch candidate.
  These checks use temporary directories and a mock `--version` executable; they do not run
  Cargo builds, stage real binaries, upload artifacts, or demonstrate hosted release success.
- Captured arguments from the workflow's package invocation are accepted by the unchanged
  0.161.0 package parser, including `0.161.0+cxline.1`, the GNU target, archive output, and prebuilt
  entrypoint/code-mode-host/bwrap paths. Four upstream package-helper modules pass 14 tests.
  The first direct invocation did not reach tests because it omitted the required
  `CODEX_REPO_ROOT`; supplying the same root environment already exported by `setup-ci` resolved
  it without changing source. From the repository root, that helper subset can be run with:

  ```bash
  env CODEX_REPO_ROOT="$PWD" PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=scripts \
    python3 -m unittest codex_package.test_cli codex_package.test_cargo \
      codex_package.test_layout codex_package.test_archive
  ```

- Byte comparisons confirm the two workflows match the published customization exactly, and
  all four restored historical records match the prior documentation branch. Relative document
  links resolve locally; new and modified documents/workflows pass whitespace and newline checks.
  The guide/index describe hosted CI, publication, checksum, and installation as pending rather
  than recording unperformed acceptance. No production dependencies or validation tools were added.
- Rust formatting and source/document whitespace and final-newline checks passed. Difference
  checks exclude `.snap` files from normal trailing-whitespace rules; those fixed-width rendering
  grids are checked separately with `core.whitespace=-blank-at-eol`, preserving meaningful padding.
- The runtime-editor test build and execution completed without warnings. `load_saved` now has
  runtime callers. Strict Clippy, non-test checking, full TUI regression, manual terminal acceptance,
  hosted CI, and release checks remain for later stages. The Git and composer checkpoints completed
  without compiler warnings and did not repeat their passing test runs.
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
  Preserve those paths. The old `should_show_fast_status` helper no longer exists; derive Fast
  from the effective service-tier id, its catalog name, and ChatGPT-account visibility, matching
  the current official model-with-reasoning label rather than checking the raw config value.
- `ChatWidget::handle_server_notification` updates token state; the App's current-thread routing
  then refreshes the status line for token notifications. Widget-only tests must simulate that
  second step, and the App-level regression protects the actual live path.
- Keep Git collection outside rendering, with the existing 5-second timeout, 64 KiB output cap,
  `GIT_OPTIONAL_LOCKS=0`, and both cwd/request guards. The shared branch-refresh entry must keep
  running the official terminal-title path after requesting a CxLine refresh.
- Reapply visual snapshot deltas against current upstream content rather than copying old
  full-screen snapshots wholesale. Keep transparent Astra blending separate from eligibility and
  scheduling, and keep Max/Ultra glyphs on the official effort renderer.
- Keep source-build test version and WSL shortcut fixtures deterministic without changing
  production version, platform detection, configuration, or daemon settings.
- Keep the build/display suffix separate from the plain upstream version: the current official
  server-version notice path treats build metadata as a different client identity. Do not replace
  compatibility or protocol inputs with `CODEX_BUILD_VERSION` when updating visible labels.
- Legacy UI limits remain unchanged: Options editing is unsupported, segment reordering only
  changes the preview order, and theme-name input keeps its 32-byte length guard.
- New upstream dependencies may require cache downloads before locked tests can run.
