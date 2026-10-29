use super::CxLineEditor;
use super::EditorCommand;
use super::EditorField;
use super::EditorPanel;
use crate::statusline::segment::SegmentId;
use crate::statusline::style::AnsiColor;
use crate::statusline::themes::ThemePresets;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::buffer::Cell;
use ratatui::layout::Rect;

#[test]
fn main_keys_navigate_edit_and_return_lifecycle_commands() {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    let release = KeyEvent::new_with_kind(KeyCode::Down, KeyModifiers::NONE, KeyEventKind::Release);
    assert_eq!(editor.handle_key_event(release), EditorCommand::None);
    assert_eq!(editor.selected_segment, 0);

    let was_enabled = editor.draft.segments.model.enabled;
    assert_eq!(press(&mut editor, KeyCode::Right), EditorCommand::None);
    assert_eq!(editor.draft.segments.model.enabled, was_enabled);

    assert_eq!(press(&mut editor, KeyCode::Down), EditorCommand::None);
    assert_eq!(editor.selected_segment, 1);
    assert_eq!(
        modified_press(&mut editor, KeyCode::Up, KeyModifiers::SHIFT),
        EditorCommand::None
    );
    assert_eq!(editor.selected_segment, 0);
    assert_eq!(editor.selected_segment_id(), SegmentId::Directory);
    assert_eq!(press(&mut editor, KeyCode::Tab), EditorCommand::None);
    assert_eq!(editor.selected_panel, EditorPanel::Settings);
    let was_enabled = editor.draft.segments.directory.enabled;
    assert_eq!(press(&mut editor, KeyCode::Enter), EditorCommand::None);
    assert_eq!(editor.draft.segments.directory.enabled, !was_enabled);

    assert_eq!(
        press(&mut editor, KeyCode::Char('p')),
        EditorCommand::SelectTheme("minimal")
    );
    assert_eq!(
        press(&mut editor, KeyCode::Char('9')),
        EditorCommand::SelectTheme("powerline-tokyo-night")
    );
    assert_eq!(
        press(&mut editor, KeyCode::Char('r')),
        EditorCommand::ResetTheme("cometix".to_string())
    );
    assert_eq!(
        press(&mut editor, KeyCode::Char('w')),
        EditorCommand::SaveTheme
    );
    assert_eq!(
        press(&mut editor, KeyCode::Char('s')),
        EditorCommand::SaveConfig
    );
    assert_eq!(press(&mut editor, KeyCode::Esc), EditorCommand::Exit);
}

#[test]
fn color_picker_has_priority_and_commits_only_on_enter() {
    let mut editor = settings_editor(EditorField::IconColor);
    let original = editor.draft.segments.model.colors.icon;

    assert_eq!(press(&mut editor, KeyCode::Enter), EditorCommand::None);
    assert!(editor.color_picker.is_open);
    assert_eq!(press(&mut editor, KeyCode::Right), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Enter), EditorCommand::None);
    assert!(!editor.color_picker.is_open);
    assert_eq!(
        editor.draft.segments.model.colors.icon,
        Some(AnsiColor::c16(1))
    );
    assert_eq!(editor.status_message.as_deref(), Some("Icon color updated"));

    assert_eq!(press(&mut editor, KeyCode::Enter), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Right), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Esc), EditorCommand::None);
    assert!(!editor.color_picker.is_open);
    assert_ne!(editor.draft.segments.model.colors.icon, original);
    assert_eq!(
        editor.draft.segments.model.colors.icon,
        Some(AnsiColor::c16(1))
    );
}

#[test]
fn icon_selector_supports_custom_input_and_canceling_only_the_custom_editor() {
    let mut editor = settings_editor(EditorField::Icon);
    assert_eq!(press(&mut editor, KeyCode::Enter), EditorCommand::None);
    assert!(editor.icon_selector.is_open);

    assert_eq!(press(&mut editor, KeyCode::Char('c')), EditorCommand::None);
    assert!(editor.icon_selector.editing_custom);
    assert_eq!(press(&mut editor, KeyCode::Esc), EditorCommand::None);
    assert!(editor.icon_selector.is_open);
    assert!(!editor.icon_selector.editing_custom);

    assert_eq!(press(&mut editor, KeyCode::Char('c')), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Char('界')), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Enter), EditorCommand::None);
    assert!(!editor.icon_selector.is_open);
    assert_eq!(editor.draft.segments.model.icon.nerd_font, "界");
    assert_eq!(editor.status_message.as_deref(), Some("Icon updated"));
}

#[test]
fn separator_editor_applies_enter_and_discards_escape() {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    let original = editor.draft.separator.clone();

    assert_eq!(press(&mut editor, KeyCode::Char('e')), EditorCommand::None);
    assert!(editor.separator_editor.is_open);
    assert_eq!(press(&mut editor, KeyCode::Tab), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Char('界')), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Enter), EditorCommand::None);
    assert_eq!(editor.draft.separator, "界");
    assert_eq!(editor.status_message.as_deref(), Some("Separator updated"));

    assert_eq!(press(&mut editor, KeyCode::Char('e')), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Backspace), EditorCommand::None);
    assert_eq!(press(&mut editor, KeyCode::Esc), EditorCommand::None);
    assert_eq!(editor.draft.separator, "界");
    assert_ne!(editor.draft.separator, original);
}

#[test]
fn ctrl_s_collects_a_new_theme_name_without_writing_storage() {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    assert_eq!(
        modified_press(&mut editor, KeyCode::Char('s'), KeyModifiers::CONTROL),
        EditorCommand::None
    );
    assert!(editor.name_input_dialog.is_open);

    for character in "新主题".chars() {
        assert_eq!(
            press(&mut editor, KeyCode::Char(character)),
            EditorCommand::None
        );
    }
    assert_eq!(
        press(&mut editor, KeyCode::Enter),
        EditorCommand::SaveNewTheme("新主题".to_string())
    );
    assert!(!editor.name_input_dialog.is_open);
    assert_eq!(editor.draft.theme, "cometix");
}

#[test]
fn editor_renders_an_open_child_dialog_above_the_main_page() {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    assert_eq!(press(&mut editor, KeyCode::Char('e')), EditorCommand::None);
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 80, /*height*/ 28,
    );
    let mut buffer = Buffer::empty(area);

    editor.render(area, &mut buffer);

    let rendered = buffer
        .content()
        .iter()
        .map(Cell::symbol)
        .collect::<String>();
    assert!(rendered.contains("CxLine Configuration"));
    assert!(rendered.contains("Separator Editor"));
    assert!(rendered.contains("Current Separator"));
}

fn settings_editor(field: EditorField) -> CxLineEditor {
    let mut editor = CxLineEditor::new(ThemePresets::get_cometix());
    editor.selected_panel = EditorPanel::Settings;
    editor.selected_field = field;
    editor
}

fn press(editor: &mut CxLineEditor, code: KeyCode) -> EditorCommand {
    modified_press(editor, code, KeyModifiers::NONE)
}

fn modified_press(
    editor: &mut CxLineEditor,
    code: KeyCode,
    modifiers: KeyModifiers,
) -> EditorCommand {
    editor.handle_key_event(KeyEvent::new(code, modifiers))
}
