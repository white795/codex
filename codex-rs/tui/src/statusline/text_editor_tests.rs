use super::name_input::NameInputDialog;
use super::separator_editor::SeparatorEditor;
use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

#[test]
fn name_input_accepts_unicode_ignores_controls_and_clears_on_reopen() {
    let mut dialog = NameInputDialog::default();
    dialog.open("Save Theme", "Enter theme name");
    for c in "深色🌙\n\t".chars() {
        dialog.input_char(c);
    }
    assert_eq!(dialog.get_input(), "深色🌙");
    dialog.backspace();
    assert_eq!(dialog.get_input(), "深色");
    dialog.close();
    assert!(!dialog.is_open);
    assert_eq!(dialog.get_input(), "");
    dialog.open("Save Theme", "Enter theme name");
    assert!(dialog.is_open);
    assert_eq!(dialog.get_input(), "");
}

#[test]
fn separator_selection_clamps_and_custom_edits_clear_the_selection() {
    let mut editor = SeparatorEditor::default();
    editor.open(" | ");
    editor.move_preset_selection(-1);
    assert_eq!(editor.get_separator(), " | ");
    editor.move_preset_selection(1);
    assert_eq!(editor.get_separator(), " │ ");
    editor.input_char('中');
    editor.input_char('\n');
    assert_eq!(editor.get_separator(), " │ 中");
    assert_eq!(editor.selected_preset, None);
    editor.backspace();
    assert_eq!(editor.get_separator(), " │ ");
    editor.move_preset_selection(-1);
    assert_eq!(editor.get_separator(), " • ");
    editor.move_preset_selection(1);
    assert_eq!(editor.get_separator(), " • ");
    editor.clear_input();
    assert_eq!(editor.get_separator(), "");
    editor.move_preset_selection(1);
    assert_eq!(editor.get_separator(), " | ");
    editor.close();
    assert!(!editor.is_open);
    assert_eq!(editor.get_separator(), "");
    editor.open("custom");
    assert_eq!(editor.get_separator(), "custom");
    assert_eq!(editor.selected_preset, None);
}

#[test]
fn closed_editors_leave_the_background_unchanged() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 64, /*height*/ 20,
    );
    let mut buffer = Buffer::filled(area, ratatui::buffer::Cell::new("x"));
    let before = buffer.clone();
    NameInputDialog::default().render(area, &mut buffer);
    SeparatorEditor::default().render(area, &mut buffer);
    assert_eq!(buffer, before);
}

#[test]
fn text_editors_render_inside_small_and_offset_areas() {
    let mut name = NameInputDialog::default();
    name.open("Save Theme", "Enter theme name");
    let mut separator = SeparatorEditor::default();
    separator.open(" │ ");
    for width in [0, 1, 2, 8, 24, 64] {
        for height in [0, 1, 2, 4, 8, 20] {
            let area = Rect::new(/*x*/ 7, /*y*/ 5, width, height);
            // A viewport-sized buffer detects writes outside the supplied rectangle.
            let mut buffer = Buffer::empty(area);
            name.render(area, &mut buffer);
            separator.render(area, &mut buffer);
        }
    }
}

#[test]
fn long_separator_preview_and_presets_do_not_draw_over_the_surrounding_content() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 90, /*height*/ 24,
    );
    let viewport = Rect::new(
        /*x*/ 7, /*y*/ 5, /*width*/ 20, /*height*/ 16,
    );
    let mut buffer = Buffer::filled(area, ratatui::buffer::Cell::new("x"));
    let mut editor = SeparatorEditor::default();
    editor.open(&"中文".repeat(50));
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
fn name_dialog_preserves_placeholder_and_unicode_input_appearance() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 64, /*height*/ 10,
    );
    let mut dialog = NameInputDialog::default();
    dialog.open("Save Theme", "Enter theme name");
    let mut empty = Buffer::empty(area);
    dialog.render(area, &mut empty);
    for c in "深色主题".chars() {
        dialog.input_char(c);
    }
    let mut filled = Buffer::empty(area);
    dialog.render(area, &mut filled);
    insta::assert_debug_snapshot!("cxline_name_dialog", (empty, filled));
}

#[test]
fn separator_editor_preserves_preset_appearance_and_clips_in_narrow_viewports() {
    let mut editor = SeparatorEditor::default();
    editor.open(" │ ");
    let mut buffers = Vec::new();
    for width in [60, 24] {
        let area = Rect::new(/*x*/ 0, /*y*/ 0, width, /*height*/ 18);
        let mut buffer = Buffer::empty(area);
        editor.render(area, &mut buffer);
        buffers.push(buffer);
    }
    insta::assert_debug_snapshot!("cxline_separator_editor", buffers);
}
