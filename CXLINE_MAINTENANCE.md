# CxLine fork maintenance

This repository keeps the upstream-compatible Codex version in Cargo metadata and adds a
separate user-visible CxLine revision. It is intentionally maintained for WSL/Linux x86_64
only.

The first release (`0.154.0+cxline.1`) remains in daily use while this branch prepares
`0.155.1+cxline.1`. See the [0.154.0 handoff](CXLINE_RELEASE_0.154.0.md)
for the released commit, verification evidence, retained limitations, and next-upgrade entry points.

## Version convention

- Upstream compatibility version: `0.155.1`
- CxLine display version: `0.155.1+cxline.1`
- Planned release tag: `cxline-v0.155.1.1`

The release workflow rejects a tag when its upstream portion differs from
`codex-rs/Cargo.toml`, or when the built binary does not report the matching CxLine revision.
When moving to a new upstream release, reset the CxLine revision to `1`. Increment the revision
when publishing another customization based on the same upstream version.

## Branch and CI strategy

### Current upgrade: 0.155.1

The `cxline/0.155.1` branch reapplies the previous custom release-tree delta onto
`rust-v0.155.1` (`be2951ea34f0d295ed0becf97079f92fa5f6950e`). The standalone CxLine
configuration/editor/runtime module is retained; no new Cometix features are added.
Upstream voice handling is preserved, with its tests adapted to the existing custom borders
and prompt. Server compatibility checks still use the upstream version, not the display suffix.

Local TUI unit regression: 4,677 passed, 2 skipped. The locked CLI build check, strict scoped
Clippy/fix, repository formatting, and Bazel lock refresh also passed; the Bazel lock was unchanged.
This is not a release or installation sign-off: hosted CI, release-package verification, and
WSL manual checks remain required before switching the installed `current` link.

### Workflow policy

- Keep `main` close to `openai/codex`.
- Maintain customization branches as `cxline/<upstream-version>`.
- `.github/workflows/cxline-ci.yml` runs on pushes to `cxline/**` and can also be started
  manually. It checks formatting, the CLI build, TUI Clippy, and the deterministic TUI/CxLine
  unit suite on Ubuntu 24.04 with two build jobs.
- `.github/workflows/cxline-release.yml` builds only `x86_64-unknown-linux-gnu`.

The upstream `blocking-ci`, `postmerge-ci`, and `v8-canary` workflows are intentionally left
unchanged to reduce conflicts during upstream merges. They target OpenAI's full infrastructure
and are not the quality gate for this fork. For this single-maintainer workflow, push the CxLine
branch directly and use `cxline-ci`. If pull requests or direct `main` pushes are introduced,
disable those upstream workflows in the fork's GitHub Actions settings before relying on their
status.

## Release checklist

1. Confirm `cxline-ci` is green for the exact commit to release.
2. Manually check `codex --version`, `/cxline`, save-and-switch, restart persistence, normal
   prompting, Chinese/emoji input, paste, and multiline input in WSL/Windows Terminal.
3. Run `cxline-release` manually once. This creates a downloadable workflow artifact without a
   GitHub Release.

   GitHub exposes `workflow_dispatch` in the web UI only after the workflow file exists on the
   default branch. For the first release, either place the two CxLine workflow files on `main`, or
   skip this dry run and let the first release tag exercise the same build job before it publishes.
4. Tag the same tested commit and push the tag:

   ```bash
   git tag -a cxline-v0.155.1.1 -m "Release Codex CxLine 0.155.1+cxline.1"
   git push origin cxline-v0.155.1.1
   ```

5. Download `SHA256SUMS` and verify the selected artifact before installing it.

Git writes, tags, and pushes remain manual operations performed by the repository maintainer.

## Artifacts

Each tagged release publishes:

- `codex-cxline-x86_64-unknown-linux-gnu`: convenient standalone binary;
- `codex-cxline-x86_64-unknown-linux-gnu.tar.gz`: canonical package containing Codex, the
  code-mode host, `bwrap`, `rg`, and the pinned zsh resource;
- `SHA256SUMS`: checksums for both downloads.

The artifacts are unsigned. Use the checksum and download only from this repository's GitHub
Release page.

## Updating from upstream

For each selected stable upstream tag:

1. Update local `main` from `upstream` and verify the unmodified upstream baseline.
2. Create `cxline/<new-version>` from that tag.
3. Review the previous customization range, then reapply the small CxLine commits in functional
   order: statusline core, configuration UI, runtime integration, input visuals, build identity,
   then fork CI and maintenance documentation. Check whether upstream already solves each change.
4. Resolve against the new TUI APIs instead of copying old upstream files wholesale.
5. Keep Cargo and protocol versions at the upstream value. `CODEX_BUILD_VERSION` derives its
   base from `CARGO_PKG_VERSION`; verify that its suffix is `+cxline.1` for a new upstream base.
   Reconcile internal workspace versions in `Cargo.lock` without upgrading unrelated dependencies.
6. Run the branch CI and repeat the WSL manual checklist. Run a manual release candidate when
   available; otherwise use the tag-triggered build as described in the release checklist.
7. Install the complete package into a new version directory, verify it, then switch `current`.

Do not copy Cometix's npm scope, translation feature, workflow deletion, or multi-platform
release matrix unless those become explicit requirements later.

## Personal installation and rollback

Download the complete `.tar.gz` package and `SHA256SUMS` from the selected GitHub Release.
In the download directory, run `sha256sum --check --ignore-missing SHA256SUMS` and confirm
that the archive is reported as `OK` before extracting it. The archive has no enclosing directory:
keep `bin/`, `codex-resources/`, `codex-path/`, and `codex-package.json` together.

The first installation uses this layout:

```text
~/.local/opt/codex-cxline/0.154.0+cxline.1/
~/.local/opt/codex-cxline/current -> 0.154.0+cxline.1/
~/.local/bin/codex-cxline -> ~/.local/opt/codex-cxline/current/bin/codex
```

With `~/.local/bin` on `PATH`, the interactive Bash aliases are:

```bash
alias codex='codex-cxline'
alias cx="$HOME/.local/bin/codex-cxline"
alias codex-default="$HOME/.local/bin/codex"
```

The last path is the existing npm-installed official launcher on this machine; verify it before
reusing these aliases elsewhere. Aliases affect interactive Bash, not necessarily scripts or IDE
launchers. Both builds use the existing `~/.codex` unless `CODEX_HOME` is explicitly overridden.

For each upgrade, extract into a new version directory and test its absolute `bin/codex` path
with an isolated `CODEX_HOME` first. Back up the active configuration before testing it with the
new version. After validation, repoint only the existing `current` symlink to the tested directory.
Keep the previous directory so rollback can repoint `current` to it, then start a new process.
Switching binaries does not roll back configuration or session data. An installer script is a
possible future convenience, not part of the first release.
