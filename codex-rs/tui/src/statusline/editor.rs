// Derived from Cometix; see this module's mod.rs for provenance.
//! Pure editing state for the CxLine configuration page.

mod input;
mod render;

use super::color_picker::ColorPicker;
use super::color_picker::ColorTarget;
use super::config::CxLineConfig;
use super::icon_selector::IconSelector;
use super::name_input::NameInputDialog;
use super::segment::SegmentId;
use super::separator_editor::SeparatorEditor;
use super::style::AnsiColor;
use super::style::StyleMode;

const SEGMENTS: [SegmentId; 5] = [
    SegmentId::Model,
    SegmentId::Directory,
    SegmentId::Git,
    SegmentId::Context,
    SegmentId::Usage,
];

const FIELDS: [EditorField; 7] = [
    EditorField::Enabled,
    EditorField::Icon,
    EditorField::IconColor,
    EditorField::TextColor,
    EditorField::BackgroundColor,
    EditorField::TextStyle,
    EditorField::Options,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EditorPanel {
    Segments,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EditorField {
    Enabled,
    Icon,
    IconColor,
    TextColor,
    BackgroundColor,
    TextStyle,
    Options,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MoveDirection {
    Previous,
    Next,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum EditorAction {
    None,
    OpenIconSelector {
        style: StyleMode,
    },
    OpenColorPicker {
        target: ColorTarget,
        current: Option<AnsiColor>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum EditorCommand {
    None,
    Exit,
    SelectTheme(&'static str),
    ResetTheme(String),
    SaveConfig,
    SaveTheme,
    SaveNewTheme(String),
}

#[derive(Debug, Clone)]
pub(super) struct CxLineEditor {
    pub(super) draft: CxLineConfig,
    original_config: CxLineConfig,
    original_theme: String,
    selected_theme_baseline: CxLineConfig,
    pub(super) segment_order: [SegmentId; 5],
    pub(super) selected_segment: usize,
    pub(super) selected_panel: EditorPanel,
    pub(super) selected_field: EditorField,
    pub(super) status_message: Option<String>,
    pub(super) color_picker: ColorPicker,
    pub(super) icon_selector: IconSelector,
    pub(super) separator_editor: SeparatorEditor,
    pub(super) name_input_dialog: NameInputDialog,
}

impl CxLineEditor {
    pub(super) fn new(config: CxLineConfig) -> Self {
        Self {
            original_theme: config.theme.clone(),
            original_config: config.clone(),
            selected_theme_baseline: config.clone(),
            draft: config,
            segment_order: SEGMENTS,
            selected_segment: 0,
            selected_panel: EditorPanel::Segments,
            selected_field: EditorField::Enabled,
            status_message: None,
            color_picker: ColorPicker::default(),
            icon_selector: IconSelector::default(),
            separator_editor: SeparatorEditor::default(),
            name_input_dialog: NameInputDialog::default(),
        }
    }

    /// Match the legacy overlay's exit behavior: same-theme edits remain drafts, while a
    /// selected theme is applied even if the page exits without saving. Edits made after the
    /// switch are excluded until an explicit save succeeds.
    pub(super) fn config_for_exit(&self) -> CxLineConfig {
        if self.draft.theme == self.original_theme {
            return self.original_config.clone();
        }

        let mut config = self.selected_theme_baseline.clone();
        config.enabled = self.original_config.enabled;
        config
    }

    pub(super) fn select_theme(&mut self, mut theme: CxLineConfig) {
        theme.enabled = self.draft.enabled;
        let theme_name = theme.theme.clone();
        self.status_message = Some(format!("Theme: {theme_name}"));
        self.selected_theme_baseline = theme.clone();
        self.draft = theme;
    }

    /// Update the in-memory baseline only after the storage layer reports a successful save.
    pub(super) fn mark_saved(&mut self) {
        self.original_config = self.draft.clone();
        self.original_theme = self.draft.theme.clone();
        self.selected_theme_baseline = self.draft.clone();
        self.status_message = Some("Configuration saved!".to_string());
    }

    pub(super) fn move_selection(&mut self, direction: MoveDirection) {
        match self.selected_panel {
            EditorPanel::Segments => {
                self.selected_segment = move_index(
                    self.selected_segment,
                    self.segment_order.len() - 1,
                    direction,
                );
            }
            EditorPanel::Settings => {
                let current = FIELDS
                    .iter()
                    .position(|field| *field == self.selected_field)
                    .unwrap_or_default();
                self.selected_field = FIELDS[move_index(current, FIELDS.len() - 1, direction)];
            }
        }
    }

    pub(super) fn switch_panel(&mut self) {
        self.selected_panel = match self.selected_panel {
            EditorPanel::Segments => EditorPanel::Settings,
            EditorPanel::Settings => EditorPanel::Segments,
        };
    }

    pub(super) fn move_segment(&mut self, direction: MoveDirection) {
        if self.selected_panel != EditorPanel::Segments {
            return;
        }
        let next = move_index(
            self.selected_segment,
            self.segment_order.len() - 1,
            direction,
        );
        if next == self.selected_segment {
            return;
        }

        self.segment_order.swap(self.selected_segment, next);
        self.selected_segment = next;
        let direction = match direction {
            MoveDirection::Previous => "up",
            MoveDirection::Next => "down",
        };
        self.status_message = Some(format!("Segment moved {direction}"));
    }

    pub(super) fn activate_current(&mut self) -> EditorAction {
        let id = self.selected_segment_id();
        if self.selected_panel == EditorPanel::Segments {
            self.toggle_segment(id);
            return EditorAction::None;
        }

        match self.selected_field {
            EditorField::Enabled => {
                self.toggle_segment(id);
                EditorAction::None
            }
            EditorField::Icon => EditorAction::OpenIconSelector {
                style: self.draft.style,
            },
            EditorField::IconColor => EditorAction::OpenColorPicker {
                target: ColorTarget::Icon,
                current: self.draft.get_segment_config(id).colors.icon,
            },
            EditorField::TextColor => EditorAction::OpenColorPicker {
                target: ColorTarget::Text,
                current: self.draft.get_segment_config(id).colors.text,
            },
            EditorField::BackgroundColor => EditorAction::OpenColorPicker {
                target: ColorTarget::Background,
                current: self.draft.get_segment_config(id).colors.background,
            },
            EditorField::TextStyle => {
                let name = segment_name(id);
                let segment = self.draft.get_segment_config_mut(id);
                segment.styles.text_bold = !segment.styles.text_bold;
                let state = if segment.styles.text_bold {
                    "enabled"
                } else {
                    "disabled"
                };
                self.status_message = Some(format!("{name} bold {state}"));
                EditorAction::None
            }
            EditorField::Options => {
                self.status_message = Some("Options editing not yet supported".to_string());
                EditorAction::None
            }
        }
    }

    pub(super) fn selected_segment_id(&self) -> SegmentId {
        self.segment_order
            .get(self.selected_segment)
            .copied()
            .unwrap_or(SegmentId::Model)
    }

    fn toggle_segment(&mut self, id: SegmentId) {
        let name = segment_name(id);
        let segment = self.draft.get_segment_config_mut(id);
        segment.enabled = !segment.enabled;
        let state = if segment.enabled {
            "enabled"
        } else {
            "disabled"
        };
        self.status_message = Some(format!("{name} {state}"));
    }
}

pub(super) fn segment_name(id: SegmentId) -> &'static str {
    match id {
        SegmentId::Model => "Model",
        SegmentId::Directory => "Directory",
        SegmentId::Git => "Git",
        SegmentId::Context => "Context Window",
        SegmentId::Usage => "Usage",
    }
}

fn move_index(current: usize, last: usize, direction: MoveDirection) -> usize {
    match direction {
        MoveDirection::Previous => current.saturating_sub(1),
        MoveDirection::Next => current.saturating_add(1).min(last),
    }
}

#[cfg(test)]
#[path = "editor_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "editor_input_tests.rs"]
mod input_tests;
