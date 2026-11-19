# CxLine fork maintenance

This repository keeps the upstream-compatible Codex version in Cargo metadata and adds a
separate user-visible CxLine revision. It is intentionally maintained for WSL/Linux x86_64 only.

The latest published and installed release is
[0.165.0+cxline.2](https://github.com/white795/codex/releases/tag/cxline-v0.165.0.2), tagged at
`a399fc867e90484150538dbe7364a99ef1aa93f8`. Its branch CI and release workflow passed, and the
complete archive was verified and installed on 2026-11-10. Its SHA256 is
`7c759a99b922d36b320533b6898a298a46ef442ec58378a2bbe183419b50909b`.
The [0.165.0 record](CXLINE_MIGRATION_0.165.0.md) retains the development checkpoints written
before that publication; current release state is recorded here.
The archived [0.161.0 migration](CXLINE_MIGRATION_0.161.0.md) retains the previous migration's
development checkpoints, including gates that were pending when written.
See the earlier [0.159.2 release handoff](CXLINE_RELEASE_0.159.2.md) and
[migration record](CXLINE_MIGRATION_0.159.2.md) for the composer/Fast refinements. The
[0.155.1 handoff](CXLINE_RELEASE_0.155.1.md) and
[first-release handoff](CXLINE_RELEASE_0.154.0.md) retain earlier customization decisions and
limitations.

The active migration uses `cxline/0.170.0`, based on `rust-v0.170.0`
(`29eb79e9866b3262a777fdf8195c1b9e63684c91`), and targets `0.170.0+cxline.1`.
Reapply `rust-v0.165.0..cxline-v0.165.0.2`, retaining the Shell UI follow-up, and add the
configuration-page master switch. Local verification, maintainer commit/push, hosted CI,
publication, and installation remain separate gates.

## Version convention

- Active upstream compatibility version: `0.170.0`
- Planned CxLine display version: `0.170.0+cxline.1`
- Planned release tag: `cxline-v0.170.0.1`
- Last published and installed release: `0.165.0+cxline.2` (`cxline-v0.165.0.2`)

The release workflow rejects a tag when its upstream portion differs from
`codex-rs/Cargo.toml`, or when the built binary does not report the matching CxLine revision.
When moving to a new upstream release, reset the CxLine revision to `1`. Increment the revision
when publishing another customization based on the same upstream version, and update the
`CODEX_BUILD_VERSION` suffix to match before creating its tag.

`CODEX_CLI_VERSION` remains the upstream Cargo version in production and is used for update and
server-compatibility decisions. `CODEX_BUILD_VERSION` adds the CxLine suffix for user-visible
build identity. `CODEX_DISPLAY_VERSION` is the UI layout label. TUI unit tests intentionally pin
the CLI and display fixtures to `0.0.0`, matching upstream source snapshots even when the
maintenance branch starts from a versioned release tag. Do not pass the CxLine build suffix into
server-compatibility or protocol inputs; the current upstream notice path distinguishes build metadata.

## Branch and CI strategy

### Active migration: 0.170.0+cxline.1

Preserve the full CxLine editor, themes, live data, asynchronous Git, Fast display, transparent
composer rules, and light-red Shell UI. Use the upstream 0.170 status-content and footer-layout
interfaces so wrapping, height allocation, and unfocused panes retain their current behavior.
Keep the upstream Shell recognition and command-submission implementation.
The 0.170 passive-hint aggregator runs before the older composer hint branches. Keep the dedicated
Shell hint out of that aggregation so it stays left-aligned and does not compete with Plan indicators.

The configuration-page header exposes the master switch. Press `T` to toggle, `S` to save,
then `Esc` to return to the conversation. Saved changes apply without restarting the client.
Disabling CxLine restores the official status line configured with `/statusline`; themes and
segment settings are retained. Exiting without saving retains the previous enabled setting.
The master switch controls status content; the selected composer and Shell visuals remain active.

Use upstream's pinned Rust 1.95.0. Align all 174 workspace package versions in the release
lockfile to 0.170.0 while preserving the 1,310 sourced external dependency records.
The complete package now includes `bin/codex-exec-server`; build and strip it alongside Codex
and the code-mode host, and pass it explicitly to the package builder. The builder also requires
`CODEX_REPO_ROOT`. The existing Cargo-only release workflow does not assemble the separate native
Linux voice runtime.

Local verification on 2026-11-19 completed a strict full TUI run: 6,254 passed and four skipped.
The only later Rust change adjusts the disabled-header foreground to the shared secondary style;
formatting, strict scoped Clippy, and the final CLI static recheck passed after that cosmetic change,
without repeating the full TUI compilation. Maintainer commit/push, hosted CI, publication, and
installation remain pending.
An initial shared saved-chat title snapshot mismatch did not recur in the strict full run;
the original thread-naming implementation and snapshot expectation are retained.

The GNU release-stage fixture verified all six package files, `execServer` metadata, archive
contents, checksums, and tag/version rejection without invoking Cargo. The real release build and
installation remain hosted and manual gates. The additional upstream Python package suite passed
26 of 27 test functions; the macOS archive-preservation test's four subcases failed because no
system `zstd` was available and the repository's fallback launcher reported
`expected platform linux-x86_64` but only provided Windows entries. The GNU `.tar.gz` staging
path passed; the macOS tool limitation is outside this fork's supported package target.

### Published release: 0.165.0+cxline.2

Shell mode uses the existing light-red accent for both composer borders, `!`, and the left-aligned
`! for shell mode` hint, while keeping the background transparent. It temporarily hides contextual
status and reclaims its row; exiting restores normal borders and the latest retained status.
Transient feedback, quit confirmation, and interactive transcript footers retain priority.
Shell recognition, key handling, and command submission keep their existing implementation.

The final Shell styling passed 37 focused tests, strict scoped Clippy, and formatting checks.
Revision preparation passed three TUI version tests and two CLI version/help tests.
The exact tag commit passed [branch CI](https://github.com/white795/codex/actions/runs/42570139259).
The [release workflow](https://github.com/white795/codex/actions/runs/42581295172) succeeded after
rerunning the original failed run, whose V8 download had returned HTTP 504. No tag was moved.
The published complete archive was verified and installed in its own version directory.

### Published release: 0.165.0+cxline.1

The initial port reapplies `rust-v0.161.0..cxline-v0.161.0.1` onto the official 0.165.0 tag,
preserving the full editor, live footer, asynchronous Git, composer visuals, and Fast display.
Its successful [branch CI](https://github.com/white795/codex/actions/runs/40985966965) and
[release workflow](https://github.com/white795/codex/actions/runs/40993418880) cover the tagged commit.
The workflows use upstream's pinned Rust 1.99.1; complete packages follow the upstream layout
without the retired patched zsh. Historical local verification remains in the migration record.

### Completed release: 0.159.2+cxline.2

The `cxline/0.159.2` branch reapplies the maintained customization onto `rust-v0.159.2`
(`ff6aec96948b70d94983af2641a6b67c94faeff5`). It keeps the full CxLine configuration UI,
themes, persistence, runtime footer, rate-limit and asynchronous Git segments, plus the selected
composer/input visuals. Missing or disabled CxLine configuration preserves the official status
line. No reasoning translation, CJK cursor patch, thread-deletion patch or new Cometix feature is
included.

The `.1` candidate completed the initial migration. The accepted `.2` follow-up keeps the same
upstream base and adds two user-visible refinements: the composer again uses a transparent
background with horizontal borders and a normal `❯` prompt, and CxLine renders an effective Fast
state as `<model> · <reasoning> fast`. The official Max `›` and Ultra `»` prompt accents remain
unchanged. The exact `.2` tag commit passed
[branch CI](https://github.com/white795/codex/actions/runs/37741875025), and its
[release workflow](https://github.com/white795/codex/actions/runs/37746114670) published the
complete package successfully.

### Workflow policy

- Keep `main` aligned with `openai/codex`.
- Maintain customization branches as `cxline/<upstream-version>`; do not merge them into `main`.
- `.github/workflows/cxline-ci.yml` runs on pushes to `cxline/**` and checks formatting, the CLI
  build, scoped TUI Clippy, and the deterministic TUI/CxLine unit suite on Ubuntu 24.04.
- `.github/workflows/cxline-release.yml` builds only `x86_64-unknown-linux-gnu` and publishes only
  for a pushed `cxline-v*` tag.
- Keep upstream workflows unchanged as reference. They target OpenAI infrastructure and are not
  this fork's release gate.

The release workflow is absent from the default `main`, so GitHub may not expose its manual
`workflow_dispatch` entry. Preserve `main` rather than copying fork workflows into it. The normal
personal workflow is green branch CI followed by a tag push; the tag-triggered workflow performs
the release build, package verification, and publication. A local release-profile build is optional
diagnostic work and should run only when explicitly needed.

## Release checklist

1. Select the upstream version and CxLine revision, update `CODEX_BUILD_VERSION`, and confirm the
   tag, binary identity, Cargo version, and documentation all agree.
2. Confirm `cxline-ci` is green for the exact commit that will receive the tag.
3. Create an annotated `cxline-v<upstream-version>.<revision>` tag on that commit and push only the
   new tag. After the final 0.170.0 migration commit passes branch CI:

   ```bash
   git tag -a cxline-v0.170.0.1 -m "Release Codex CxLine 0.170.0+cxline.1"
   git push origin cxline-v0.170.0.1
   ```

4. Confirm the tag-triggered workflow validates the tag, builds the WSL/Linux package, and publishes
   the GitHub Release successfully.
5. Download `SHA256SUMS` with the complete archive and verify the archive before extraction.
6. With an isolated `CODEX_HOME`, verify the package version and required `codex`,
   `codex-code-mode-host`, `codex-exec-server`, `bwrap`, `rg`, and manifest files. Do not require
   the retired patched zsh. Then check startup, `/cxline`, save-and-switch, master-switch
   disable/re-enable, input behavior, Fast display, and `/daemon` as relevant to the change.
7. Back up the active configuration, install into a new immutable version directory, test with the
   existing `.codex`, and repoint `current` only after acceptance. Keep the previous directory.
8. Add or update the release handoff only after publication. Record the immutable commit and tag,
   Actions runs, checksum, installation result, manual acceptance, and known limitations.

Git writes, tags and pushes remain manual operations performed by the repository maintainer.
Never move or recreate a published release tag. A documentation-only archival commit after
publication does not require a new tag.

## Artifacts

Each tagged release publishes:

- `codex-cxline-x86_64-unknown-linux-gnu`: convenient standalone binary;
- `codex-cxline-x86_64-unknown-linux-gnu.tar.gz`: canonical package containing Codex and the
  bundled tools required by package-aware features such as `/daemon` → `Use this CLI build`;
- `SHA256SUMS`: checksums for both downloads.

The artifacts are unsigned. Download them only from this repository's GitHub Release and verify
the checksum. Prefer the complete archive; the standalone binary does not provide the package
layout or bundled tools.

## Testing notes

- Use `just test-tui-unit` for the deterministic TUI suite and the branch workflow as the
  canonical Ubuntu snapshot gate.
- Nextest resolves workspace metadata before selecting tests. An offline nextest run can need
  additional locked packages that a scoped `cargo test` did not use; populate the missing caches
  without regenerating the lockfile or changing dependency versions.
- The recipe changes only the test subprocess's color and terminal hints; it clears
  `NO_COLOR`, tmux, screen, and Zellij hints without changing ordinary `just test` or user settings.
- Generic composer snapshots force the non-WSL image-paste shortcut under `cfg(test)`, keeping the
  stored Ubuntu value `ctrl+v` deterministic. The dedicated shortcut-overlay tests still cover the
  WSL `ctrl+alt+v` path, while production WSL detection remains unchanged.
- The fork's scoped Clippy command uses `--no-deps -D warnings`: CxLine/TUI warnings remain
  errors. Do not carry old baseline warnings or compatibility workarounds into a new release
  without checking whether they still apply.
- Do not update snapshots merely because `codex-rs/Cargo.toml` contains a release version. TUI
  tests use the source-build fixture `0.0.0`; the active production compatibility version is `0.170.0`.
- When changing composer visuals, exercise normal, disabled, Bash, Luna Reserve, Max, and Ultra
  prompt paths. Upstream effort-tier rendering can override the normal prompt glyph.
- For Shell UI changes, assert cell colors and transparent backgrounds as well as text snapshots.
  Cover status-row removal, multiline/narrow input, footer priority, and restoration on mode exit;
  a border-color correction needs focused style checks even when snapshot text is unchanged.
- Update test extraction with the visual change: the first row is now a border, not the prompt,
  and `\n\n›` no longer delimits the composer. Check the draft or full composer frame explicitly;
  do not replace transcript/history echoes, inactive pane previews, or menu selection markers.
  Active composer and inactive pane prompts can use different glyphs in the same frame.
  RGB comparison fixtures
  should use the existing `terminal_palette::rgb_color` helper rather than bypassing the lint.
- Fixed-grid snapshots retain trailing spaces as part of their terminal-width assertions. Preserve
  that padding; run source whitespace checks separately from snapshot review.
- Multiline Vim samples must retain every restored input line plus both borders. Include clipboard
  and terminal key-sequence snapshots when reviewing composer changes; App/ChatWidget/startup
  filters do not cover them. After snapshot-only corrections, rerun affected cases with updates
  disabled and report the initial full run and focused recheck separately.
- Derive Fast display from the same effective predicate as the official status line; do not infer
  it independently from raw configuration or the requested service tier. On 0.165.0, use the
  effective tier id, its catalog's Fast name, and ChatGPT-account visibility, including a
  catalog-default Fast tier whose request id is not `priority`.

## Updating from upstream

For each selected stable upstream tag:

1. Select a stable upstream release, update local `main` from `upstream`, and verify the unmodified
   tag before beginning customization work.
2. Create `cxline/<new-version>` from the selected `rust-v<new-version>` tag. Do not merge that
   upstream release into the previous customization branch.
3. Review the previous customization range, then reapply small commits in functional order:
   statusline core, configuration UI, runtime entry, live base data, rate limits, asynchronous Git,
   input visuals, build identity, fork CI, and maintenance documentation.
4. Resolve against current TUI APIs instead of copying old central files wholesale. Treat App,
   ChatWidget, composer, footer, daemon, version, and event wiring as high-churn seams. Check whether
   upstream or Cometix already provides a better implementation, but merge neither source blindly.
5. Keep Cargo and protocol versions at the upstream value. Reset the CxLine revision to `1`, and
   verify production display identity separately from test-only source identity.
   Audit all workspace members reported by Cargo metadata, including implicit path members,
   before aligning release-tag lockfile versions; preserve every sourced external dependency.
6. Run focused tests as each stage lands, then one hosted branch quality gate on the final
   candidate. Recheck official status-line behavior with CxLine disabled.
7. After green branch CI, use the hosted tag workflow for the normal release. Build locally only
   when diagnosing packaging or when explicit pre-tag verification is desired. Verify the published
   complete archive before installing it into a new version directory.

The current published customization range is `rust-v0.165.0..cxline-v0.165.0.2`, including the
Shell UI follow-up. The earlier
complete published 0.159.2 customization range is
`rust-v0.159.2..cxline-v0.159.2.2`. The `.2` follow-up alone is
`cxline-v0.159.2.1..cxline-v0.159.2.2`. See the release handoff for the commit sequence and lessons
to carry into the next upstream migration.

## Personal installation and rollback

Install complete archives under versioned directories:

```text
~/.local/opt/codex-cxline/0.161.0+cxline.1/
~/.local/opt/codex-cxline/0.165.0+cxline.1/
~/.local/opt/codex-cxline/0.165.0+cxline.2/
~/.local/opt/codex-cxline/0.170.0+cxline.1/  # create after release/package verification
~/.local/opt/codex-cxline/current -> 0.165.0+cxline.2/
~/.local/bin/codex-cxline -> ~/.local/opt/codex-cxline/current/bin/codex
```

The archive has no enclosing directory; keep `bin/`, `codex-resources/`, `codex-path/`, and
`codex-package.json` together. Test the absolute `bin/codex` path with an isolated `CODEX_HOME`
before using the existing configuration. Repoint only `current` after acceptance, and retain the
previous version directory so rollback requires changing one symlink and starting a new process.

`current` was switched to `0.165.0+cxline.2` on 2026-11-10, retaining older versions.
For the pending 0.170 release, use a new `0.170.0+cxline.1/` directory after downloading and
verifying the published complete package. Keep 0.165.0+cxline.2 available for rollback, test the
new build, then repoint `current` and start a new process. Source migration leaves the installed
build and real client configuration untouched.

Suggested interactive Bash aliases remain:

```bash
alias codex='codex-cxline'
alias cx="$HOME/.local/bin/codex-cxline"
alias codex-default="$HOME/.local/bin/codex"
```

Both builds use the existing `~/.codex` unless `CODEX_HOME` is explicitly overridden. Switching
binaries does not roll back configuration or session data.
