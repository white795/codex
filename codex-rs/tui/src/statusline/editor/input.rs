// Derived from Cometix; see the parent statusline module for provenance.
//! Keyboard routing and child-dialog coordination for the CxLine editor.

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;

use super::CxLineEditor;
use super::EditorAction;
use super::EditorCommand;
use super::EditorPanel;
use super::MoveDirection;
use crate::statusline::color_picker::ColorTarget;
use crate::statusline::style::AnsiColor;
use crate::statusline::style::StyleMode;
use crate::statusline::themes::THEME_NAMES;

impl CxLineEditor {
    pub(in crate::statusline) fn handle_key_event(&mut self, key_event: KeyEvent) -> EditorCommand {
        if key_event.kind != KeyEventKind::Press && key_event.kind != KeyEventKind::Repeat {
            return EditorCommand::None;
        }

        if self.color_picker.is_open {
            self.handle_color_picker_key(key_event);
            return EditorCommand::None;
        }
        if self.icon_selector.is_open {
            self.handle_icon_selector_key(key_event);
            return EditorCommand::None;
        }
        if self.separator_editor.is_open {
            self.handle_separator_editor_key(key_event);
            return EditorCommand::None;
        }
        if self.name_input_dialog.is_open {
            return self.handle_name_input_key(key_event);
        }

        if key_event.modifiers.contains(KeyModifiers::CONTROL)
            && key_event.code == KeyCode::Char('s')
        {
            self.name_input_dialog
                .open("Save as New Theme", "Enter theme name:");
            return EditorCommand::None;
        }

        if key_event.modifiers.contains(KeyModifiers::SHIFT) {
            match key_event.code {
                KeyCode::Up => {
                    self.move_segment(MoveDirection::Previous);
                    return EditorCommand::None;
                }
                KeyCode::Down => {
                    self.move_segment(MoveDirection::Next);
                    return EditorCommand::None;
                }
                _ => {}
            }
        }

        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => EditorCommand::Exit,
            KeyCode::Up | KeyCode::Char('k') => {
                self.move_selection(MoveDirection::Previous);
                EditorCommand::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.move_selection(MoveDirection::Next);
                EditorCommand::None
            }
            KeyCode::Tab => {
                self.switch_panel();
                EditorCommand::None
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                let action = self.activate_current();
                self.open_editor_action(action);
                EditorCommand::None
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Right | KeyCode::Char('l') => {
                if self.selected_panel == EditorPanel::Settings {
                    let action = self.activate_current();
                    self.open_editor_action(action);
                }
                EditorCommand::None
            }
            KeyCode::Char('p') | KeyCode::Char('P') => self.next_theme_command(),
            KeyCode::Char('r') | KeyCode::Char('R') => {
                EditorCommand::ResetTheme(self.original_theme.clone())
            }
            KeyCode::Char('w') | KeyCode::Char('W') => EditorCommand::SaveTheme,
            KeyCode::Char('s') | KeyCode::Char('S') => EditorCommand::SaveConfig,
            KeyCode::Char('e') | KeyCode::Char('E') => {
                self.separator_editor.open(&self.draft.separator);
                EditorCommand::None
            }
            KeyCode::Char(number @ '1'..='9') => THEME_NAMES
                .get(number as usize - '1' as usize)
                .copied()
                .map_or(EditorCommand::None, EditorCommand::SelectTheme),
            _ => EditorCommand::None,
        }
    }

    fn open_editor_action(&mut self, action: EditorAction) {
        match action {
            EditorAction::None => {}
            EditorAction::OpenIconSelector { style } => self.icon_selector.open(style),
            EditorAction::OpenColorPicker { target, current } => {
                self.color_picker.open(target, current);
            }
        }
    }

    fn next_theme_command(&self) -> EditorCommand {
        let current = THEME_NAMES
            .iter()
            .position(|theme| *theme == self.draft.theme)
            .unwrap_or_default();
        EditorCommand::SelectTheme(THEME_NAMES[(current + 1) % THEME_NAMES.len()])
    }

    fn handle_color_picker_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.color_picker.close(),
            KeyCode::Enter => {
                if let Some(color) = self.color_picker.get_selected_color() {
                    self.apply_color(self.color_picker.target_field, color);
                }
                self.color_picker.close();
            }
            KeyCode::Tab => self.color_picker.cycle_mode(),
            KeyCode::Up | KeyCode::Char('k') => self.color_picker.move_vertical(-1),
            KeyCode::Down | KeyCode::Char('j') => self.color_picker.move_vertical(1),
            KeyCode::Left | KeyCode::Char('h') => self.color_picker.move_horizontal(-1),
            KeyCode::Right | KeyCode::Char('l') => self.color_picker.move_horizontal(1),
            KeyCode::Backspace => self.color_picker.backspace(),
            KeyCode::Char(c) => self.color_picker.input_char(c),
            _ => {}
        }
    }

    fn handle_icon_selector_key(&mut self, key_event: KeyEvent) {
        if self.icon_selector.editing_custom {
            match key_event.code {
                KeyCode::Esc => self.icon_selector.editing_custom = false,
                KeyCode::Enter if self.icon_selector.finish_custom_input() => {
                    if let Some(icon) = self.icon_selector.get_selected_icon() {
                        self.apply_icon(icon);
                    }
                    self.icon_selector.close();
                }
                KeyCode::Enter => {}
                KeyCode::Backspace => self.icon_selector.backspace(),
                KeyCode::Char(c) => self.icon_selector.input_char(c),
                _ => {}
            }
            return;
        }

        match key_event.code {
            KeyCode::Esc => self.icon_selector.close(),
            KeyCode::Enter => {
                if let Some(icon) = self.icon_selector.get_selected_icon() {
                    self.apply_icon(icon);
                }
                self.icon_selector.close();
            }
            KeyCode::Tab => self.icon_selector.toggle_style(),
            KeyCode::Up | KeyCode::Char('k') => self.icon_selector.move_selection(-1),
            KeyCode::Down | KeyCode::Char('j') => self.icon_selector.move_selection(1),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.icon_selector.start_custom_input();
            }
            _ => {}
        }
    }

    fn handle_separator_editor_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.separator_editor.close(),
            KeyCode::Enter => {
                self.draft.separator = self.separator_editor.get_separator();
                self.status_message = Some("Separator updated".to_string());
                self.separator_editor.close();
            }
            KeyCode::Tab => self.separator_editor.clear_input(),
            KeyCode::Up | KeyCode::Char('k') => {
                self.separator_editor.move_preset_selection(-1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.separator_editor.move_preset_selection(1);
            }
            KeyCode::Backspace => self.separator_editor.backspace(),
            KeyCode::Char(c) => self.separator_editor.input_char(c),
            _ => {}
        }
    }

    fn handle_name_input_key(&mut self, key_event: KeyEvent) -> EditorCommand {
        match key_event.code {
            KeyCode::Esc => self.name_input_dialog.close(),
            KeyCode::Enter => {
                let name = self.name_input_dialog.get_input().to_string();
                self.name_input_dialog.close();
                if !name.is_empty() {
                    return EditorCommand::SaveNewTheme(name);
                }
            }
            KeyCode::Backspace => self.name_input_dialog.backspace(),
            KeyCode::Char(c) => self.name_input_dialog.input_char(c),
            _ => {}
        }
        EditorCommand::None
    }

    fn apply_color(&mut self, target: ColorTarget, color: AnsiColor) {
        let segment_id = self.selected_segment_id();
        let segment = self.draft.get_segment_config_mut(segment_id);
        let message = match target {
            ColorTarget::Icon => {
                segment.colors.icon = Some(color);
                "Icon color updated"
            }
            ColorTarget::Text => {
                segment.colors.text = Some(color);
                "Text color updated"
            }
            ColorTarget::Background => {
                segment.colors.background = Some(color);
                "Background color updated"
            }
        };
        self.status_message = Some(message.to_string());
    }

    fn apply_icon(&mut self, icon: String) {
        let style = self.draft.style;
        let segment_id = self.selected_segment_id();
        let segment = self.draft.get_segment_config_mut(segment_id);
        match style {
            StyleMode::Plain => segment.icon.plain = icon,
            StyleMode::NerdFont | StyleMode::Powerline => segment.icon.nerd_font = icon,
        }
        self.status_message = Some("Icon updated".to_string());
    }
}
