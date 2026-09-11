use super::CxLineEditor;
use super::EditorAction;
use super::EditorField;
use super::EditorPanel;
use super::MoveDirection;
use crate::statusline::color_picker::ColorTarget;
use crate::statusline::segment::SegmentId;
use crate::statusline::style::StyleMode;
use crate::statusline::themes::ThemePresets;
use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::buffer::Cell;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Modifier;

#[test]
fn exiting_discards_unsaved_edits_when_the_theme_did_not_change() {
    let mut original = ThemePresets::get_cometix();
    original.enabled = false;
    original.separator = " original ".to_string();
    let mut editor = CxLineEditor::new(original.clone());

    editor.draft.separator = " unsaved ".to_string();
    editor.draft.segments.model.enabled = false;

    assert_eq!(editor.config_for_exit(), original);
}

#[test]
fn exiting_after_a_theme_switch_applies_only_the_selected_theme_baseline() {
    let mut original = ThemePresets::get_cometix();
    original.enabled = false;
    let mut editor = CxLineEditor::new(original);
    let selected_theme = ThemePresets::get_nord();

    editor.select_theme(selected_theme.clone());
    editor.draft.separator = " unsaved after switch ".to_string();
    editor.draft.segments.git.enabled = false;

    let mut expected = selected_theme;
    expected.enabled = false;
    assert_eq!(editor.config_for_exit(), expected);
}

#[test]
fn marking_a_save_commits_the_current_draft_as_the_new_exit_baseline() {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    editor.draft.separator = " saved ".to_string();
    editor.draft.segments.context.enabled = false;
    let saved = editor.draft.clone();

    editor.mark_saved();
    editor.draft.separator = " later unsaved ".to_string();

    assert_eq!(editor.config_for_exit(), saved);
    assert_eq!(
        editor.status_message.as_deref(),
        Some("Configuration saved!")
    );
}

#[test]
fn selection_navigation_clamps_within_each_panel() {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());

    editor.move_selection(MoveDirection::Previous);
    assert_eq!(editor.selected_segment, 0);
    for _ in 0..10 {
        editor.move_selection(MoveDirection::Next);
    }
    assert_eq!(editor.selected_segment, 4);

    editor.switch_panel();
    assert_eq!(editor.selected_panel, EditorPanel::Settings);
    editor.move_selection(MoveDirection::Previous);
    assert_eq!(editor.selected_field, EditorField::Enabled);
    for _ in 0..10 {
        editor.move_selection(MoveDirection::Next);
    }
    assert_eq!(editor.selected_field, EditorField::Options);
    editor.switch_panel();
    assert_eq!(editor.selected_panel, EditorPanel::Segments);
}

#[test]
fn segment_reordering_changes_only_the_preview_order() {
    let original = ThemePresets::get_cometix();
    let mut editor = CxLineEditor::new(original.clone());

    editor.move_segment(MoveDirection::Previous);
    editor.move_segment(MoveDirection::Next);

    assert_eq!(
        editor.segment_order,
        [
            SegmentId::Directory,
            SegmentId::Model,
            SegmentId::Git,
            SegmentId::Context,
            SegmentId::Usage,
        ]
    );
    assert_eq!(editor.selected_segment, 1);
    assert_eq!(editor.config_for_exit(), original);
    assert_eq!(editor.status_message.as_deref(), Some("Segment moved down"));
}

#[test]
fn activating_fields_updates_the_draft_or_returns_the_required_dialog() {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    assert_eq!(editor.activate_current(), EditorAction::None);
    assert!(!editor.draft.segments.model.enabled);

    editor.switch_panel();
    editor.selected_field = EditorField::Icon;
    assert_eq!(
        editor.activate_current(),
        EditorAction::OpenIconSelector {
            style: StyleMode::NerdFont,
        }
    );

    let color_cases = [
        (EditorField::IconColor, ColorTarget::Icon),
        (EditorField::TextColor, ColorTarget::Text),
        (EditorField::BackgroundColor, ColorTarget::Background),
    ];
    for (field, target) in color_cases {
        editor.selected_field = field;
        let expected_color = match target {
            ColorTarget::Icon => editor.draft.segments.model.colors.icon,
            ColorTarget::Text => editor.draft.segments.model.colors.text,
            ColorTarget::Background => editor.draft.segments.model.colors.background,
        };
        assert_eq!(
            editor.activate_current(),
            EditorAction::OpenColorPicker {
                target,
                current: expected_color,
            }
        );
    }

    editor.selected_field = EditorField::TextStyle;
    let was_bold = editor.draft.segments.model.styles.text_bold;
    assert_eq!(editor.activate_current(), EditorAction::None);
    assert_eq!(editor.draft.segments.model.styles.text_bold, !was_bold);

    editor.selected_field = EditorField::Options;
    assert_eq!(editor.activate_current(), EditorAction::None);
    assert_eq!(
        editor.status_message.as_deref(),
        Some("Options editing not yet supported")
    );
}

#[test]
fn editor_renders_safely_in_small_offset_and_intersected_buffers() {
    let editor = CxLineEditor::new(ThemePresets::get_cometix());
    for width in [0, 1, 2, 8, 24, 80] {
        for height in [0, 1, 2, 8, 24, 40] {
            let area = Rect::new(/*x*/ 7, /*y*/ 5, width, height);
            let mut buffer = Buffer::empty(area);
            editor.render(area, &mut buffer);
            editor.render(
                Rect::new(
                    /*x*/ 0, /*y*/ 0, /*width*/ 120, /*height*/ 80,
                ),
                &mut buffer,
            );
        }
    }
}

#[test]
fn editor_does_not_overwrite_content_outside_its_viewport() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 60,
    );
    let viewport = Rect::new(
        /*x*/ 7, /*y*/ 5, /*width*/ 42, /*height*/ 30,
    );
    let mut buffer = Buffer::filled(area, Cell::new("x"));
    let editor = CxLineEditor::new(ThemePresets::get_cometix());

    editor.render(viewport, &mut buffer);

    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            if !viewport.contains((x, y).into()) {
                assert_eq!(buffer[(x, y)].symbol(), "x", "at ({x}, {y})");
            }
        }
    }
}

#[test]
fn editor_uses_legacy_focus_and_selected_theme_styles() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 32,
    );
    let mut buffer = Buffer::empty(area);
    let editor = CxLineEditor::new(ThemePresets::get_cometix());

    editor.render(area, &mut buffer);

    assert!(has_styled_symbol(
        &buffer,
        "▶",
        Color::Cyan,
        Modifier::empty()
    ));
    assert!(has_styled_symbol(
        &buffer,
        "✓",
        Color::Green,
        Modifier::BOLD
    ));
}

#[test]
fn editor_layout_preserves_the_legacy_page_at_wide_and_narrow_sizes() {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    let wide = render_text(
        &editor,
        Rect::new(
            /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 32,
        ),
    );

    editor.switch_panel();
    editor.selected_segment = 2;
    editor.selected_field = EditorField::TextColor;
    let narrow = render_text(
        &editor,
        Rect::new(
            /*x*/ 0, /*y*/ 0, /*width*/ 48, /*height*/ 24,
        ),
    );

    insta::assert_snapshot!(
        "cxline_editor_layout",
        format!("WIDE\n{wide}\n\nNARROW\n{narrow}")
    );
}

fn has_styled_symbol(buffer: &Buffer, symbol: &str, foreground: Color, modifier: Modifier) -> bool {
    buffer.content().iter().any(|cell| {
        cell.symbol() == symbol && cell.fg == foreground && cell.modifier.contains(modifier)
    })
}

fn render_text(editor: &CxLineEditor, area: Rect) -> String {
    let mut buffer = Buffer::empty(area);
    editor.render(area, &mut buffer);
    let Some((left, top, right, bottom)) = non_blank_bounds(&buffer) else {
        return String::new();
    };
    (top..=bottom)
        .map(|y| {
            let mut line = (left..=right)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>();
            let trimmed = line.trim_end().len();
            line.truncate(trimmed);
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn non_blank_bounds(buffer: &Buffer) -> Option<(u16, u16, u16, u16)> {
    let area = *buffer.area();
    let mut bounds: Option<(u16, u16, u16, u16)> = None;
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            if buffer[(x, y)].symbol() != " " {
                bounds = Some(match bounds {
                    Some((left, top, right, bottom)) => {
                        (left.min(x), top.min(y), right.max(x), bottom.max(y))
                    }
                    None => (x, y, x, y),
                });
            }
        }
    }
    bounds
}
