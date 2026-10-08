# CxLine fork maintenance

This repository keeps the upstream-compatible Codex version in Cargo metadata and adds a
separate user-visible CxLine revision. It is intentionally maintained for WSL/Linux x86_64 only.

The current published release is
[0.159.2+cxline.1](https://github.com/white795/codex/releases/tag/cxline-v0.159.2.1), tagged at
`4582156b97a60b92af29de2628dedd8a93f1249b`. The next release candidate is
`0.159.2+cxline.2` on `cxline/0.159.2`; it restores the transparent bordered composer and adds the
effective Fast-mode state to the CxLine model segment. See the active
[0.159.2 migration record](CXLINE_MIGRATION_0.159.2.md). The
[0.155.1 handoff](CXLINE_RELEASE_0.155.1.md) and
[first-release handoff](CXLINE_RELEASE_0.154.0.md) retain earlier customization decisions and
limitations.

## Version convention

- Upstream compatibility version: `0.159.2`
- Published CxLine display version: `0.159.2+cxline.1`
- Next CxLine display version: `0.159.2+cxline.2`
- Next release tag: `cxline-v0.159.2.2`

The release workflow rejects a tag when its upstream portion differs from
`codex-rs/Cargo.toml`, or when the built binary does not report the matching CxLine revision.
When moving to a new upstream release, reset the CxLine revision to `1`. Increment the revision
when publishing another customization based on the same upstream version, and update the
`CODEX_BUILD_VERSION` suffix to match before creating its tag.

`CODEX_CLI_VERSION` remains the upstream Cargo version in production and is used for update and
server-compatibility decisions. `CODEX_BUILD_VERSION` adds the CxLine suffix for user-visible
build identity. TUI unit tests intentionally pin `CODEX_CLI_VERSION` to `0.0.0`, matching
upstream source snapshots even when the maintenance branch starts from a versioned release tag.

## Branch and CI strategy

### Follow-up release candidate: 0.159.2+cxline.2

The `cxline/0.159.2` branch reapplies the maintained customization onto `rust-v0.159.2`
(`ff6aec96948b70d94983af2641a6b67c94faeff5`). It keeps the full CxLine configuration UI,
themes, persistence, runtime footer, rate-limit and asynchronous Git segments, plus the selected
composer/input visuals. Missing or disabled CxLine configuration preserves the official status
line. No reasoning translation, CJK cursor patch, thread-deletion patch or new Cometix feature is
included.

The published `.1` candidate completed formatting, a locked CLI check, strict TUI Clippy, and the
deterministic TUI suite before tagging. The `.2` follow-up keeps the same upstream base and adds two
user-visible refinements: the composer again uses a transparent background with horizontal borders
and a `❯` prompt, and CxLine renders an effective Fast state as
`<model> · <reasoning> fast`. Its release-preparation commit must receive a new green branch CI
before it becomes the tagged release commit.

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
`workflow_dispatch` entry. Preserve `main` rather than copying fork workflows into it. Use a local
release-candidate build before tagging, or let the tag-triggered build be the first hosted release
build after all other checks pass.

## Release checklist

1. Confirm `cxline-ci` is green for the exact commit that will receive the tag.
2. Build the WSL/Linux release candidate with two Cargo jobs, matching the hosted workflow.
3. Verify the complete package reports `codex-cxline 0.159.2+cxline.2` and contains `codex`,
   `codex-code-mode-host`, `bwrap`, `rg`, the pinned zsh resource, and its package manifest.
4. With an isolated `CODEX_HOME`, check startup, normal prompting, `/cxline`, save-and-switch,
   restart persistence, Chinese/emoji input, paste, multiline input, and `/daemon` using the
   complete package.
5. Back up the active configuration, then test the same candidate with the existing `.codex`.
6. Tag the same tested commit and push the tag:

   ```bash
   git tag -a cxline-v0.159.2.2 -m "Release Codex CxLine 0.159.2+cxline.2"
   git push origin cxline-v0.159.2.2
   ```

7. Confirm the tag-triggered build and GitHub Release succeed. Download `SHA256SUMS` with the
   complete archive and verify it before installation.
8. Add `CXLINE_RELEASE_0.159.2.md` only after `.2` publication, recording both 0.159.2 revisions,
   the released commit, Actions run, artifact verification, installation result and remaining
   limitations.

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
- Generic composer snapshots force the non-WSL image-paste shortcut under `cfg(test)`, keeping the
  stored Ubuntu value `ctrl+v` deterministic. The dedicated shortcut-overlay tests still cover the
  WSL `ctrl+alt+v` path, while production WSL detection remains unchanged.
- The upstream `codex-core` currently emits an unused `ToolCallSource` import warning in this
  release tree. The fork's scoped Clippy command uses `--no-deps -D warnings`: CxLine/TUI warnings
  remain errors while the unrelated dependency warning does not fail the job.
- Do not update snapshots merely because `codex-rs/Cargo.toml` contains a release version. TUI
  tests use the source-build fixture `0.0.0`; production builds still use `0.159.2`.

## Updating from upstream

For each selected stable upstream tag:

1. Update local `main` from `upstream` and verify the unmodified upstream baseline.
2. Create `cxline/<new-version>` from the selected `rust-v<new-version>` tag.
3. Review the previous customization range, then reapply small commits in functional order:
   statusline core, configuration UI, runtime entry, live base data, rate limits, asynchronous Git,
   input visuals, build identity, fork CI, and maintenance documentation.
4. Resolve against current TUI APIs instead of copying old central files wholesale. Check whether
   upstream or Cometix already provides a better implementation, but merge neither source blindly.
5. Keep Cargo and protocol versions at the upstream value. Reset the CxLine revision to `1`, and
   verify production display identity separately from test-only source identity.
6. Run focused tests as each stage lands, then one hosted branch quality gate on the final
   candidate. Recheck official status-line behavior with CxLine disabled.
7. Build and test the complete package before tagging. Install into a new version directory and
   keep the previous directory for rollback.

The published `.1` customization review range is `rust-v0.159.2..cxline-v0.159.2.1`. Review the
same-base follow-up as `cxline-v0.159.2.1..HEAD`; after `.2` publication, replace `HEAD` with the
immutable `cxline-v0.159.2.2` tag in the release handoff.

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

Suggested interactive Bash aliases remain:

```bash
alias codex='codex-cxline'
alias cx="$HOME/.local/bin/codex-cxline"
alias codex-default="$HOME/.local/bin/codex"
```

Both builds use the existing `~/.codex` unless `CODEX_HOME` is explicitly overridden. Switching
binaries does not roll back configuration or session data.
