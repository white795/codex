# CxLine fork maintenance

This repository keeps the upstream-compatible Codex version in Cargo metadata and adds a
separate user-visible CxLine revision. It is intentionally maintained for WSL/Linux x86_64 only.

The last accepted and locally adopted release is
[0.159.2+cxline.2](https://github.com/white795/codex/releases/tag/cxline-v0.159.2.2), tagged at
`be99489eaeaaab6a0e3cef2bbca69a1b67a01e10`. It restores the transparent bordered composer and
adds the effective Fast-mode state to the CxLine model segment. See the completed
[0.159.2 release handoff](CXLINE_RELEASE_0.159.2.md) and archived
[migration record](CXLINE_MIGRATION_0.159.2.md). The
[0.155.1 handoff](CXLINE_RELEASE_0.155.1.md) and
[first-release handoff](CXLINE_RELEASE_0.154.0.md) retain earlier customization decisions and
limitations.

The active migration is `cxline/0.161.0`, based on `rust-v0.161.0`
(`979011409de0a60b52f179721948e65531d26144`) and targeting `0.161.0+cxline.1`.
Publication, downloaded-package verification, and local adoption are still pending. Read the
[active migration record](CXLINE_MIGRATION_0.161.0.md) for the next checkpoint; do not treat
restored workflows as evidence that hosted CI or release acceptance has completed.

## Version convention

- Active upstream compatibility version: `0.161.0`
- Planned CxLine display version: `0.161.0+cxline.1`
- Planned release tag: `cxline-v0.161.0.1`
- Last accepted release: `0.159.2+cxline.2` (`cxline-v0.159.2.2`)

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

### Active migration: 0.161.0+cxline.1

The branch starts from the new official release tag and reapplies the released
`rust-v0.159.2..cxline-v0.159.2.2` customization delta. Foundation, full editor UI, runtime handoff,
live footer including asynchronous Git, composer visuals, and build identity are connected.
The WSL/Linux workflows retain the last released fork policy; upstream workflows stay unchanged.
Remaining outer-screen snapshot adaptation, final Rust checks, hosted CI, release packaging,
and manual WSL acceptance remain separate gates in the active migration record.

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
   new tag. Only after the exact 0.161.0 candidate passes branch CI, its planned commands are:

   ```bash
   git tag -a cxline-v0.161.0.1 -m "Release Codex CxLine 0.161.0+cxline.1"
   git push origin cxline-v0.161.0.1
   ```

4. Confirm the tag-triggered workflow validates the tag, builds the WSL/Linux package, and publishes
   the GitHub Release successfully.
5. Download `SHA256SUMS` with the complete archive and verify the archive before extraction.
6. With an isolated `CODEX_HOME`, verify the package version and required `codex`,
   `codex-code-mode-host`, `bwrap`, `rg`, zsh, and manifest files. Then check startup, `/cxline`,
   save-and-switch, input behavior, Fast display, and `/daemon` as relevant to the change.
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
- The recipe changes only the test subprocess's color and terminal hints; it clears
  `NO_COLOR`, tmux, screen, and Zellij hints without changing ordinary `just test` or user settings.
- Generic composer snapshots force the non-WSL image-paste shortcut under `cfg(test)`, keeping the
  stored Ubuntu value `ctrl+v` deterministic. The dedicated shortcut-overlay tests still cover the
  WSL `ctrl+alt+v` path, while production WSL detection remains unchanged.
- The fork's scoped Clippy command uses `--no-deps -D warnings`: CxLine/TUI warnings remain
  errors. Do not carry old baseline warnings or compatibility workarounds into a new release
  without checking whether they still apply.
- Do not update snapshots merely because `codex-rs/Cargo.toml` contains a release version. TUI
  tests use the source-build fixture `0.0.0`; the active production compatibility version is `0.161.0`.
- When changing composer visuals, exercise normal, disabled, Bash, Luna Reserve, Max, and Ultra
  prompt paths. Upstream effort-tier rendering can override the normal prompt glyph.
- Derive Fast display from the same effective predicate as the official status line; do not infer
  it independently from raw configuration or the requested service tier. On 0.161.0, use the
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
6. Run focused tests as each stage lands, then one hosted branch quality gate on the final
   candidate. Recheck official status-line behavior with CxLine disabled.
7. After green branch CI, use the hosted tag workflow for the normal release. Build locally only
   when diagnosing packaging or when explicit pre-tag verification is desired. Verify the published
   complete archive before installing it into a new version directory.

The complete published 0.159.2 customization range is
`rust-v0.159.2..cxline-v0.159.2.2`. The `.2` follow-up alone is
`cxline-v0.159.2.1..cxline-v0.159.2.2`. See the release handoff for the commit sequence and lessons
to carry into the next upstream migration.

## Personal installation and rollback

Install complete archives under versioned directories:

```text
~/.local/opt/codex-cxline/0.159.2+cxline.1/
~/.local/opt/codex-cxline/0.159.2+cxline.2/
~/.local/opt/codex-cxline/current -> 0.159.2+cxline.2/
~/.local/bin/codex-cxline -> ~/.local/opt/codex-cxline/current/bin/codex
```

The archive has no enclosing directory; keep `bin/`, `codex-resources/`, `codex-path/`, and
`codex-package.json` together. Test the absolute `bin/codex` path with an isolated `CODEX_HOME`
before using the existing configuration. Repoint only `current` after acceptance, and retain the
previous version directory so rollback requires changing one symlink and starting a new process.

For the pending 0.161.0 release, use a new `0.161.0+cxline.1/` directory only after downloading
and verifying its published complete package. The existing `current` remains on the last accepted
0.159.2 build until manual acceptance; restoring the maintenance branch does not upgrade it.

Suggested interactive Bash aliases remain:

```bash
alias codex='codex-cxline'
alias cx="$HOME/.local/bin/codex-cxline"
alias codex-default="$HOME/.local/bin/codex"
```

Both builds use the existing `~/.codex` unless `CODEX_HOME` is explicitly overridden. Switching
binaries does not roll back configuration or session data.
