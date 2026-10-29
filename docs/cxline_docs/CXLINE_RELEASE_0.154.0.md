# CxLine 0.154.0 first-release handoff

Archived: 2026-09-15. Status: released and adopted for personal WSL use.

## Delivered

- Upstream baseline: `rust-v0.154.0`, `6b9826e3aa83b1a5947db50f4332cb9c65f1b340`.
- CxLine reference: Haleclipse/Cometix `0.144.3`,
  `c5dce3cbd3914c6a3402fb00b2ee9d4df988b0a6`.
- Released commit: `bb55c29d47e586bdd91a9a98a62e1419bf12d074` on `cxline/0.154.0`.
- [Release cxline-v0.154.0.1](https://github.com/white795/codex/releases/tag/cxline-v0.154.0.1),
  published 2026-09-14; displayed version `0.154.0+cxline.1`.
- Full CxLine configuration UI, themes and persistence, runtime status data, selected Cometix
  input/list visuals, separate build identity, and WSL/Linux x86_64 CI and release packaging.

## Decisions

Cargo/protocol versions remain upstream-compatible; only display identity carries `+cxline.N`.
CxLine takes over the footer only with a valid saved and enabled `cxline/config.toml` under the
active Codex home. Opening `/cxline` without that file uses defaults; saving enables it.

The fork excludes reasoning translation, thread deletion, CJK custom cursor rendering, and the
other unselected Cometix changes. Official workflows remain as upstream reference. The personal
release supports only GNU Linux x86_64 in WSL, without npm publishing or signing. Use official
Codex as the upgrade baseline and Cometix as a selective reference, not as an automatic merge source.

## Verification

- [Branch CI](https://github.com/white795/codex/actions/runs/34829619518) and
  [release build/publication](https://github.com/white795/codex/actions/runs/34832017554)
  succeeded for the released commit.
- Final input-style local TUI regression: 4,403 passed, 2 skipped. Package-builder unit tests:
  20 passed. CLI check/build, relevant strict Clippy and formatting checks passed during delivery.
- The installed complete package reports `codex-cxline 0.154.0+cxline.1`; package metadata and
  the `current` / command symlinks were checked. The maintainer confirmed using the release with
  the existing `.codex`, resuming this session and continuing normal conversation.
- The earlier isolated check confirmed `/cxline` and save-to-activate statusline behavior.
  Individual manual checks for every theme, resize, emoji, paste, multiline and Vim interaction
  were not separately recorded; general acceptance does not imply exhaustive coverage.
- The CLI unit-test executable previously hit substantial memory pressure during linking and
  produced no result. Lower-memory CLI checks/builds and hosted CI were used instead; no full
  workspace or cross-platform test completion is claimed.

## Known limitations

- Inherited UI behavior: segment ordering is not persisted, Options is unimplemented, custom
  themes do not enter the built-in selection list, and the renderer ignores separator settings.
- Theme-name input retains the old 32-byte entry limit; tiny terminals are checked for safe
  clipping, not complete usability. CJK cursor rendering remains an optional future task.
- Manual `workflow_dispatch` was not exercised; the first release used the tag-triggered path.
- No automatic installer/updater was added. Existing upstream update behavior is not a CxLine
  upgrade mechanism; use this fork's releases and the version-directory installation procedure.

## Next maintenance session

Read [CXLINE_MAINTENANCE.md](CXLINE_MAINTENANCE.md), choose an upstream stable tag, and inspect
its changes before selecting a migration strategy. Review the first customization range with:

```bash
git log --reverse --oneline 6b9826e3aa83b1a5947db50f4332cb9c65f1b340..bb55c29d47e586bdd91a9a98a62e1419bf12d074
```

Start with `codex-rs/tui/src/statusline/`, `chatwidget/cxline.rs`, the App/overlay and footer
integration, input visuals, `codex-rs/build-info/`, and the two `cxline-*.yml` workflows.
Recheck toolchain and baseline test fixes against the new upstream instead of carrying them blindly.

Detailed first-session plans and evidence live in the original workspace's sibling `docs/`
directory and are not included in a fresh clone. This handoff and the maintenance guide preserve
the essential context in Git; temporary test homes, build caches and credentials are not archives.
