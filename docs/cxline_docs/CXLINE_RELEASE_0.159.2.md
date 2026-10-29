# CxLine 0.159.2 release handoff

Archived: 2026-10-08. Status: migration, publication, and local WSL adoption complete.

## Delivered

- Upstream baseline: `rust-v0.159.2`, `ff6aec96948b70d94983af2641a6b67c94faeff5`.
- Maintenance branch: `cxline/0.159.2`.
- [Release cxline-v0.159.2.1](https://github.com/white795/codex/releases/tag/cxline-v0.159.2.1)
  published the initial migration from commit `4582156b97a60b92af29de2628dedd8a93f1249b`;
  its displayed version is `0.159.2+cxline.1`.
- [Release cxline-v0.159.2.2](https://github.com/white795/codex/releases/tag/cxline-v0.159.2.2)
  published the accepted follow-up from commit `be99489eaeaaab6a0e3cef2bbca69a1b67a01e10`;
  its displayed version is `0.159.2+cxline.2`.
- The complete CxLine configuration UI, themes, persistence, runtime footer, official rate-limit
  data, asynchronous Git state, and selected composer visuals were retained. The `.2` follow-up
  restored the transparent bordered composer and added the official effective Fast state to the
  CxLine model segment.
- Both releases provide the WSL/Linux x86_64 standalone binary, canonical `.tar.gz` package, and
  `SHA256SUMS`. The `.2` complete package is installed and active through the versioned `current`
  symlink; `.1` remains available for rollback.

## Decisions

The migration started from the new official release tag and reapplied the maintained customization
in dependency order. It did not merge the new upstream history into the old customization branch,
nor copy old central TUI files wholesale. `main` remains aligned with `openai/codex`; customization
continues only on `cxline/<upstream-version>` branches.

CxLine remains opt-in through its saved `enabled` configuration. Missing, invalid, or disabled
configuration preserves the current official status line. The fork still excludes reasoning
translation, the Cometix CJK cursor patch, thread deletion, workflow deletion, and non-WSL release
targets.

Cargo and protocol identity remain the upstream `0.159.2`. Only the user-visible build identity is
`0.159.2+cxline.N`. Tests retain the upstream source-build `0.0.0` fixture so a release tag does not
rewrite unrelated snapshots.

Fast display reuses Codex's effective service-tier predicate instead of inferring state from raw
configuration. The composer uses `❯` normally while retaining the official Max `›` and Ultra `»`
prompt accents, colors, and ignition animation. These special glyphs are intentional, not a missed
part of the CxLine visual migration.

## Verification

- The `.1` tag commit passed
  [branch CI](https://github.com/white795/codex/actions/runs/37720679566) and the
  [release workflow](https://github.com/white795/codex/actions/runs/37724757633).
- The `.2` tag commit passed
  [branch CI](https://github.com/white795/codex/actions/runs/37741875025) and the
  [release workflow](https://github.com/white795/codex/actions/runs/37746114670). Formatting,
  locked CLI checking, scoped strict TUI Clippy, deterministic TUI tests, tag/version validation,
  release building, package staging, and publication completed successfully.
- Focused composer/Fast tests, the TUI regression suite, formatting, and linting were exercised
  before the `.2` release-preparation commit. One timing-sensitive analytics test timed out in a
  concurrent local run and passed in isolation; hosted branch CI is the canonical final result.
- The downloaded `.2` archive matched its published SHA-256:
  `818f7b8c030c6ff2907e6a076b312afc4d57a3a8ea723876f127e8fa27586d00`.
  Its manifest and binary report `0.159.2+cxline.2`, and the package contains `codex`,
  `codex-code-mode-host`, `bwrap`, `rg`, and the pinned zsh resource.
- The active link resolves to `~/.local/opt/codex-cxline/0.159.2+cxline.2`; both interactive
  `codex` and `cx` resolve to that build. The previous `.1` directory and original downloaded
  archive were retained.
- Manual WSL acceptance confirmed the transparent bordered composer, CxLine footer, spaced
  reasoning label, and visible `fast` state. The observed Max `›` prompt was traced to the official
  effort-tier behavior and accepted unchanged.

## Reusable migration lessons

1. Freeze a stable official tag first, create the new customization branch from that tag, and port
   the previous released delta into the new tree. Do not make the old customization branch the
   merge base for a new upstream release.
2. Keep commits independently reviewable: configuration/rendering foundation, storage and small
   editors, complete configuration UI, runtime entry, live base data, rate limits, asynchronous
   Git, composer visuals, build identity, fork automation, and documentation.
3. Standalone CxLine modules are usually reusable; App, ChatWidget, composer, footer, daemon, and
   event wiring are the high-churn integration seams. Adapt those seams to current APIs rather than
   transplanting old central files.
4. Reuse official derived state and helpers for reasoning effort, Fast mode, rate-limit windows,
   current working directory, and workspace commands. This avoids a second interpretation of
   upstream behavior.
5. Test identity and platform inputs explicitly. Release Cargo versions must not leak into source
   snapshots, and generic snapshots must not inherit WSL-only `ctrl+alt+v` behavior accidentally.
6. Inspect every upstream layer that can override a visual. In 0.159.2 the effort-tier renderer
   replaces the normal prompt for Max and Ultra; testing only the fallback `❯` path was insufficient
   to explain the live screenshot.
7. Treat screenshots and real-terminal acceptance as a separate gate for visual migrations.
   Snapshot updates can be internally consistent while still missing the intended appearance.
8. The normal personal release path is: green branch CI on the exact commit, annotated tag push,
   hosted release workflow, checksum verification, isolated package check, then versioned install.
   A local release-profile build is optional diagnostic work and should not run by default.
9. Install complete archives into immutable version directories and atomically repoint `current`.
   Keep at least one accepted previous directory so rollback does not require rebuilding.

## Known limitations

- Releases remain unsigned and support only WSL/Linux `x86_64-unknown-linux-gnu`.
- The upstream daemon can update independently and may request different feature settings. Use
  `--no-daemon` for isolation or `/daemon` → `Use this CLI build` when the package daemon should
  match the installed CxLine binary.
- The official update banner compares the upstream compatibility version; it is not a CxLine fork
  updater. CxLine upgrades remain manual.
- Earlier UI limitations still apply: segment ordering is not persisted, Options is unimplemented,
  custom themes do not enter the built-in theme list, and separator settings are not rendered.
- Manual acceptance was representative, not an exhaustive pass over every theme, terminal width,
  input mode, and daemon transition. No cross-platform or full-workspace claim is made.

## Next maintenance session

Read [CXLINE_MAINTENANCE.md](CXLINE_MAINTENANCE.md) and this handoff, then select a stable official
tag. The released customization range is
`rust-v0.159.2..cxline-v0.159.2.2`; separately carry forward any later documentation-only commits.
Reset the CxLine revision to `1` for the next upstream version, inspect official status-line,
composer, effort-tier, daemon, version, and rate-limit changes before porting, and let the hosted
tag workflow build the release unless a local release build is explicitly needed.
