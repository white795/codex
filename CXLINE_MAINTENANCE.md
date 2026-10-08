# CxLine fork maintenance

This repository keeps the upstream-compatible Codex version in Cargo metadata and adds a
separate user-visible CxLine revision. It is intentionally maintained for WSL/Linux x86_64 only.

The current release candidate is `0.159.2+cxline.1` on `cxline/0.159.2`. Its code quality gate
[passed](https://github.com/white795/codex/actions/runs/37718380023) on 2026-10-08 for
`c671644f35e5c7e7c6c48a5ab1b92b48c4f5865d`; release packaging, manual WSL acceptance and
publication are still pending. See the active
[0.159.2 migration record](CXLINE_MIGRATION_0.159.2.md). The latest published version remains
[0.155.1+cxline.1](CXLINE_RELEASE_0.155.1.md); the
[first-release handoff](CXLINE_RELEASE_0.154.0.md) retains the original customization decisions
and limitations.

## Version convention

- Upstream compatibility version: `0.159.2`
- Planned CxLine display version: `0.159.2+cxline.1`
- Planned release tag: `cxline-v0.159.2.1`

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

### Release candidate: 0.159.2

The `cxline/0.159.2` branch reapplies the maintained customization onto `rust-v0.159.2`
(`ff6aec96948b70d94983af2641a6b67c94faeff5`). It keeps the full CxLine configuration UI,
themes, persistence, runtime footer, rate-limit and asynchronous Git segments, plus the selected
composer/input visuals. Missing or disabled CxLine configuration preserves the official status
line. No reasoning translation, CJK cursor patch, thread-deletion patch or new Cometix feature is
included.

The code candidate CI completed formatting, a locked CLI check, strict TUI Clippy, and the
deterministic TUI suite: 5,647 passed and 4 skipped. The first restored CI run exposed four
release-version-dependent upstream snapshots; the lasting fix keeps tests on the source-build
identity without changing production version behavior. A documentation commit pushed after that
candidate must receive its own green branch CI before it becomes the tagged release commit.

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
3. Verify the complete package reports `codex-cxline 0.159.2+cxline.1` and contains `codex`,
   `codex-code-mode-host`, `bwrap`, `rg`, the pinned zsh resource, and its package manifest.
4. With an isolated `CODEX_HOME`, check startup, normal prompting, `/cxline`, save-and-switch,
   restart persistence, Chinese/emoji input, paste, multiline input, and `/daemon` using the
   complete package.
5. Back up the active configuration, then test the same candidate with the existing `.codex`.
6. Tag the same tested commit and push the tag:

   ```bash
   git tag -a cxline-v0.159.2.1 -m "Release Codex CxLine 0.159.2+cxline.1"
   git push origin cxline-v0.159.2.1
   ```

7. Confirm the tag-triggered build and GitHub Release succeed. Download `SHA256SUMS` with the
   complete archive and verify it before installation.
8. Add `CXLINE_RELEASE_0.159.2.md` only after publication, recording the released commit, Actions
   run, artifact verification, installation result and remaining limitations.

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
- A full run under WSL detects the actual platform and renders image paste as `ctrl+alt+v`; the
  stored Ubuntu snapshots render `ctrl+v`. Reject resulting `*.snap.new` files unless the product
  behavior is intentionally changing.
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

The current customization review range is
`rust-v0.159.2..c671644f35e5c7e7c6c48a5ab1b92b48c4f5865d`. At publication, replace the end with
the immutable `cxline-v0.159.2.1` tag in the release handoff.

## Personal installation and rollback

Install complete archives under versioned directories:

```text
~/.local/opt/codex-cxline/0.159.2+cxline.1/
~/.local/opt/codex-cxline/current -> 0.159.2+cxline.1/
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
