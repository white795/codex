# CxLine fork maintenance

This repository keeps the upstream-compatible Codex version in Cargo metadata and adds a
separate user-visible CxLine revision. It is intentionally maintained for WSL/Linux x86_64
only.

## Version convention

- Upstream compatibility version: `0.154.0`
- CxLine display version: `0.154.0+cxline.1`
- Release tag: `cxline-v0.154.0.1`

The release workflow rejects a tag when its upstream portion differs from
`codex-rs/Cargo.toml`, or when the built binary does not report the matching CxLine revision.
When moving to a new upstream release, reset the CxLine revision to `1`. Increment the revision
when publishing another customization based on the same upstream version.

## Branch and CI strategy

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
   git tag -a cxline-v0.154.0.1 -m "Release Codex CxLine 0.154.0+cxline.1"
   git push origin cxline-v0.154.0.1
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
3. Reapply the small CxLine commits in functional order: statusline core, configuration UI,
   runtime integration, input visuals, then build identity.
4. Resolve against the new TUI APIs instead of copying old upstream files wholesale.
5. Update `CODEX_BUILD_VERSION` to `<new-version>+cxline.1` while keeping Cargo and protocol
   versions at the upstream value.
6. Run the branch CI, repeat the WSL manual checklist, run the release workflow manually, and
   only then create the release tag.

Do not copy Cometix's npm scope, translation feature, workflow deletion, or multi-platform
release matrix unless those become explicit requirements later.
