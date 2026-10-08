# CxLine 0.155.1 release handoff

Archived: 2026-09-20. Status: migration and publication complete; local adoption not verified.

## Delivered

- Upstream baseline: `rust-v0.155.1`, `be2951ea34f0d295ed0becf97079f92fa5f6950e`.
- Released commit: `93869126df1e8177e533e27d2a9d42488ef72135` on `cxline/0.155.1`.
- [Release cxline-v0.155.1.1](https://github.com/white795/codex/releases/tag/cxline-v0.155.1.1),
  published 2026-09-20; displayed version `0.155.1+cxline.1`.
- Complete existing CxLine configuration UI, themes, persistence, runtime footer and input
  visuals retained. No additional Cometix features or platforms were added.
- GNU Linux x86_64 standalone binary, complete `.tar.gz` package, and `SHA256SUMS` published.

## Decisions

Reapplied the previous customization delta onto the new official release tree, rather than
copying old upstream files. The standalone CxLine directory's 49 files are unchanged from the
previous custom branch. Adaptations preserve upstream voice behavior, add `spoken: false` to
two existing test fixtures, retain the upstream version for server compatibility checks, and
update visual assertions for the custom prompt and borders.

Cargo.lock retains all 1,333 external package records from the new official baseline; only
152 internal package versions were reconciled to `0.155.1`. The Bazel lock remained unchanged.
The [first-release decisions and UI limitations](CXLINE_RELEASE_0.154.0.md) still apply;
excluding Cometix's thread-deletion patch does not remove official upstream thread management.

## Verification

- [Branch CI](https://github.com/white795/codex/actions/runs/35495218889) and
  [release build/publication](https://github.com/white795/codex/actions/runs/35497535857)
  succeeded for the released commit. Remote branch, peeled release tag and local HEAD matched
  that commit at the pre-archive check.
- Local TUI regression: 4,677 passed, 2 skipped. Locked CLI check, strict scoped Clippy/fix,
  repository formatting and Bazel lock refresh passed. Tests were not repeated after fix/fmt.
- Hosted release passed tag/version validation, release builds, artifact staging and upload.
  Published `SHA256SUMS` entries match GitHub's asset digests. This metadata comparison is not
  a checksum or runtime verification of a locally downloaded package.

## Known limitations and installation handoff

- Last checked on 2026-09-20: the installed `current` and launcher still resolve to
  `0.154.0+cxline.1`. No claim is made that the new package has passed local WSL manual testing
  or been used with the existing `.codex`. Installation remains a separate follow-up.
- Install the complete archive into a new version directory, verify its checksum, test with
  an isolated Codex home, then back up the active configuration before testing it. Switch
  `current` only after acceptance; retain the old directory for rollback.
- Artifacts are unsigned. No full-workspace, cross-platform or heavy CLI unit-test completion
  is claimed. Existing UI limitations remain unchanged.
- This documentation-only handoff does not change the released binary or tag and needs no
  new release. The maintainer performs the final documentation commit and push.

## Next maintenance session

Read [CXLINE_MAINTENANCE.md](CXLINE_MAINTENANCE.md) and inspect the next selected official tag.
The current released customization delta is `rust-v0.155.1..cxline-v0.155.1.1`; separately carry
forward any later maintenance-document updates. Recheck toolchain, lock versions, TUI APIs,
voice layout and visual tests before deciding which compatibility fixes remain necessary.

Detailed plans and summaries live in the original workspace's sibling `docs/` directory,
outside this Git checkout. This handoff preserves the essential release evidence in the repo.
