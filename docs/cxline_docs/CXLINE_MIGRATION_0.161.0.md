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
- The live footer, including Git, is connected. Composer visuals are not migrated yet, and the
  installed version is unchanged.
- Next: restore transparent composer borders and prompt accents, then build identity, CI, and
  release documentation.

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
  Rust formatting and `git diff --check` passed; the imported Rust and migration files also
  passed whitespace and final-newline checks. No snapshot updates were accepted.
- The runtime-editor test build and execution completed without warnings. `load_saved` now has
  runtime callers. Strict Clippy, non-test checking, full TUI regression, composer visuals,
  manual terminal acceptance, hosted CI, and release checks remain for later stages. The Git
  checkpoint completed without compiler warnings and did not repeat its passing test run.
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
- Keep source-build test version and WSL shortcut fixtures deterministic without changing
  production version, platform detection, configuration, or daemon settings.
- Legacy UI limits remain unchanged: Options editing is unsupported, segment reordering only
  changes the preview order, and theme-name input keeps its 32-byte length guard.
- New upstream dependencies may require cache downloads before locked tests can run.
