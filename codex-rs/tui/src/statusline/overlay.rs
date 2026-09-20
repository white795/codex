// Derived from Cometix; see this module's mod.rs for provenance.
//! Full-screen runtime wrapper for the CxLine configuration editor.

use std::io::Result;
use std::path::Path;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use super::config::CxLineConfig;
use super::editor::CxLineEditor;
use super::editor::EditorOutcome;
use super::storage::CxLineStore;
use crate::tui;
use crate::tui::TuiEvent;

pub(crate) struct CxLineOverlay {
    editor: CxLineEditor,
    store: CxLineStore,
    is_done: bool,
}

impl CxLineOverlay {
    pub(crate) fn new(codex_home: &Path, config: CxLineConfig) -> Self {
        let store = CxLineStore::new(codex_home);
        let load_error = CxLineConfig::load_saved(codex_home).err();
        let mut editor = CxLineEditor::new(config);
        if let Some(error) = load_error {
            editor.status_message = Some(format!("Failed to load configuration: {error}"));
        }

        Self {
            editor,
            store,
            is_done: false,
        }
    }

    pub(crate) fn handle_event(&mut self, tui: &mut tui::Tui, event: TuiEvent) -> Result<()> {
        match event {
            TuiEvent::Key(key_event) => {
                self.handle_key_event(key_event);
                tui.frame_requester().schedule_frame();
                Ok(())
            }
            TuiEvent::Draw | TuiEvent::Resume | TuiEvent::Resize(_) | TuiEvent::FocusGained => {
                tui.draw(u16::MAX, |frame| {
                    self.render(frame.area(), frame.buffer);
                })?;
                Ok(())
            }
            TuiEvent::Paste(_) | TuiEvent::FocusLost => Ok(()),
        }
    }

    pub(super) fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        let command = self.editor.handle_key_event(key_event);
        if self.editor.execute_command(&self.store, command) == EditorOutcome::Exit {
            self.is_done = true;
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        self.editor.render(area, buf);
    }

    pub(crate) fn is_done(&self) -> bool {
        self.is_done
    }

    pub(crate) fn config_for_exit(&self) -> CxLineConfig {
        self.editor.config_for_exit()
    }

    #[cfg(test)]
    pub(super) fn draft(&self) -> &CxLineConfig {
        &self.editor.draft
    }

    #[cfg(test)]
    pub(super) fn status_message(&self) -> Option<&str> {
        self.editor.status_message.as_deref()
    }
}
