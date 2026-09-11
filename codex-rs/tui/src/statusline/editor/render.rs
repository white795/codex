// Derived from Cometix; see the parent statusline module for provenance.
//! Rendering for the CxLine configuration page.

use std::path::Path;

use codex_protocol::openai_models::ReasoningEffort;
use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Clear;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

use super::CxLineEditor;
use super::EditorField;
use super::EditorPanel;
use super::segment_name;
use crate::statusline::StatusLineContext;
use crate::statusline::renderer::StatusLineRenderer;
use crate::statusline::segment::Segment;
use crate::statusline::segment::SegmentId;
use crate::statusline::segments::ContextSegment;
use crate::statusline::segments::DirectorySegment;
use crate::statusline::segments::GitSegment;
use crate::statusline::segments::ModelSegment;
use crate::statusline::segments::UsageSegment;
use crate::statusline::themes::THEME_NAMES;

const HELP_ITEMS: [(&str, &str); 12] = [
    ("[Tab]", "Switch Panel"),
    ("[↑↓]", "Select"),
    ("[Shift+↑↓]", "Reorder"),
    ("[Enter]", "Toggle/Edit"),
    ("[1-9]", "Theme"),
    ("[P]", "Cycle Theme"),
    ("[R]", "Reset Theme"),
    ("[E]", "Edit Separator"),
    ("[W]", "Write Theme"),
    ("[Ctrl+S]", "Save Theme"),
    ("[S]", "Save Config"),
    ("[Esc]", "Quit"),
];

impl CxLineEditor {
    pub(super) fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }
        Clear.render(area, buf);

        let theme_height = self.theme_selector_height(area.width);
        let [
            title_area,
            preview_area,
            theme_area,
            content_area,
            help_area,
        ] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(theme_height),
            Constraint::Min(10),
            Constraint::Length(4),
        ])
        .areas(area);

        self.render_title(title_area, buf);
        self.render_preview(preview_area, buf);
        self.render_theme_selector(theme_area, buf);

        let [segments_area, settings_area] =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(content_area);
        self.render_segments(segments_area, buf);
        self.render_settings(settings_area, buf);
        self.render_help(help_area, buf);

        self.color_picker.render(area, buf);
        self.icon_selector.render(area, buf);
        self.separator_editor.render(area, buf);
        self.name_input_dialog.render(area, buf);
    }

    fn theme_selector_height(&self, width: u16) -> u16 {
        let content_width = usize::from(width.saturating_sub(4));
        (self.theme_lines(content_width).len() as u16 + 2).min(5)
    }

    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("CxLine Configuration")
            .cyan()
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
            .render(area, buf);
    }

    fn render_preview(&self, area: Rect, buf: &mut Buffer) {
        let context = StatusLineContext::new("gpt-5.2-codex", Path::new("/home/user/Cxline"))
            .with_reasoning_effort(Some(ReasoningEffort::Medium))
            .with_context(
                /*used_tokens*/ Some(50_000),
                /*window_size*/ Some(128_000),
            )
            .with_rate_limit(
                /*hourly_percent*/ Some(25.0),
                /*weekly_percent*/ Some(15.0),
                /*weekly_resets_at*/ Some("1-28-14".to_string()),
            )
            .with_git_preview("main", "✓", /*ahead*/ 0, /*behind*/ 0);

        let mut renderer = StatusLineRenderer::new(&self.draft);
        for segment_id in self.segment_order {
            if !self.draft.get_segment_config(segment_id).enabled {
                continue;
            }
            let data = match segment_id {
                SegmentId::Model => ModelSegment.collect(&context),
                SegmentId::Directory => DirectorySegment.collect(&context),
                SegmentId::Git => GitSegment.collect(&context),
                SegmentId::Context => ContextSegment.collect(&context),
                SegmentId::Usage => UsageSegment.collect(&context),
            };
            if let Some(data) = data {
                renderer.add_segment(segment_id, data);
            }
        }

        Paragraph::new(renderer.render_line())
            .block(Block::default().borders(Borders::ALL).title("Preview"))
            .render(area, buf);
    }

    fn render_theme_selector(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Theme");
        let content_width = usize::from(block.inner(area).width);
        Paragraph::new(self.theme_lines(content_width))
            .block(block)
            .render(area, buf);
    }

    fn theme_lines(&self, content_width: usize) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let mut current_spans = Vec::new();
        let mut current_width = 0;

        for theme in THEME_NAMES {
            let selected = self.draft.theme == *theme;
            let marker = if selected { "[✓]" } else { "[ ]" };
            let text = format!("{marker} {theme}");
            let separator_width = usize::from(!current_spans.is_empty()) * 2;
            let item_width = text.chars().count();

            if current_width + separator_width + item_width > content_width
                && !current_spans.is_empty()
            {
                lines.push(Line::from(current_spans));
                current_spans = Vec::new();
                current_width = 0;
            }
            if !current_spans.is_empty() {
                current_spans.push("  ".into());
                current_width += 2;
            }
            current_spans.push(if selected {
                Span::from(text).green().bold()
            } else {
                text.into()
            });
            current_width += item_width;
        }

        if !current_spans.is_empty() {
            lines.push(Line::from(current_spans));
        }
        lines
    }

    fn render_segments(&self, area: Rect, buf: &mut Buffer) {
        let items = self.segment_order.map(|id| {
            let selected =
                id == self.selected_segment_id() && self.selected_panel == EditorPanel::Segments;
            let enabled = self.draft.get_segment_config(id).enabled;
            let marker = if enabled { "●" } else { "○" };
            let name = segment_name(id);
            let line = if selected {
                Line::from(vec!["▶ ".cyan(), format!("{marker} {name}").into()])
            } else {
                Line::from(format!("  {marker} {name}"))
            };
            ListItem::new(line)
        });
        let border_style = if self.selected_panel == EditorPanel::Segments {
            Style::default().cyan()
        } else {
            Style::default()
        };
        List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Segments")
                    .border_style(border_style),
            )
            .render(area, buf);
    }

    fn render_settings(&self, area: Rect, buf: &mut Buffer) {
        let id = self.selected_segment_id();
        let segment = self.draft.get_segment_config(id);
        let segment_name = segment_name(id);
        let icon_style = optional_color_style(segment.colors.icon_color());
        let text_style = optional_color_style(segment.colors.text_color());
        let background = match segment.colors.background_color() {
            Some(color) => Span::from("██").fg(color),
            None => "--".dark_gray(),
        };

        let field_line = |field: EditorField, spans: Vec<Span<'static>>| {
            let mut line = vec![if self.selected_panel == EditorPanel::Settings
                && self.selected_field == field
            {
                "▶ ".cyan()
            } else {
                "  ".into()
            }];
            line.extend(spans);
            Line::from(line)
        };

        let lines = vec![
            Line::from(format!("{segment_name} Segment").bold()),
            Line::default(),
            field_line(
                EditorField::Enabled,
                vec![format!("├─ Enabled: {}", if segment.enabled { "✓" } else { "✗" }).into()],
            ),
            field_line(
                EditorField::Icon,
                vec![
                    "├─ Icon: ".into(),
                    Span::styled(segment.icon.get(self.draft.style).to_string(), icon_style),
                ],
            ),
            field_line(
                EditorField::IconColor,
                vec!["├─ Icon Color: ".into(), Span::styled("██", icon_style)],
            ),
            field_line(
                EditorField::TextColor,
                vec!["├─ Text Color: ".into(), Span::styled("██", text_style)],
            ),
            field_line(
                EditorField::BackgroundColor,
                vec!["├─ Background: ".into(), background],
            ),
            field_line(
                EditorField::TextStyle,
                vec![
                    format!(
                        "├─ Bold: {}",
                        if segment.styles.text_bold {
                            "[✓]"
                        } else {
                            "[ ]"
                        }
                    )
                    .into(),
                ],
            ),
            field_line(
                EditorField::Options,
                vec![format!("└─ Options: {} items", segment.options.len()).into()],
            ),
        ];
        let border_style = if self.selected_panel == EditorPanel::Settings {
            Style::default().cyan()
        } else {
            Style::default()
        };
        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Settings")
                    .border_style(border_style),
            )
            .render(area, buf);
    }

    fn render_help(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Help");
        let content_width = usize::from(block.inner(area).width);
        let mut lines = Vec::new();
        let mut current_spans = Vec::new();
        let mut current_width = 0;

        for (key, description) in HELP_ITEMS {
            let item_width = key.chars().count() + description.chars().count() + 1;
            let separator_width = usize::from(!current_spans.is_empty()) * 2;
            if current_width + separator_width + item_width > content_width
                && !current_spans.is_empty()
            {
                lines.push(Line::from(current_spans));
                current_spans = Vec::new();
                current_width = 0;
            }
            if !current_spans.is_empty() {
                current_spans.push("  ".into());
                current_width += 2;
            }
            current_spans.push(key.cyan().bold());
            current_spans.push(format!(" {description}").gray());
            current_width += item_width;
        }

        if !current_spans.is_empty() {
            lines.push(Line::from(current_spans));
        }
        if let Some(message) = &self.status_message {
            lines.push(Line::from(message.clone().green()));
        }
        Paragraph::new(lines).block(block).render(area, buf);
    }
}

fn optional_color_style(color: Option<Color>) -> Style {
    color.map_or_else(Style::default, |color| Style::default().fg(color))
}
