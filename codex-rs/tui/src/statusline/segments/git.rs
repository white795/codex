// Derived from Cometix; see this module's mod.rs for provenance.
// Git Segment - displays git branch and status from async preview data

use crate::statusline::StatusLineContext;
use crate::statusline::segment::Segment;
use crate::statusline::segment::SegmentData;

pub struct GitSegment;

impl Segment for GitSegment {
    fn collect(&self, ctx: &StatusLineContext) -> Option<SegmentData> {
        // @cometix: only render from async preview data — never run blocking
        // git commands on the render thread.
        let preview = ctx.git_preview.as_ref()?;
        if preview.branch.is_empty() && preview.status.is_empty() {
            return None;
        }
        let primary = preview.branch.clone();
        let mut parts = Vec::new();
        parts.push(preview.status.clone());
        if preview.ahead > 0 {
            parts.push(format!("↑{}", preview.ahead));
        }
        if preview.behind > 0 {
            parts.push(format!("↓{}", preview.behind));
        }
        Some(
            SegmentData::new(primary)
                .with_secondary(parts.join(" "))
                .with_metadata("branch", &preview.branch)
                .with_metadata("status", &preview.status)
                .with_metadata("ahead", preview.ahead.to_string())
                .with_metadata("behind", preview.behind.to_string()),
        )
    }
}
