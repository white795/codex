// Derived from Cometix; see the parent statusline module for provenance.
//! Executes editor commands against an explicitly supplied CxLine store.

use std::io;

use super::CxLineEditor;
use super::EditorCommand;
use super::EditorOutcome;
use crate::statusline::storage::CxLineStore;

impl CxLineEditor {
    pub(super) fn execute_command(
        &mut self,
        store: &CxLineStore,
        command: EditorCommand,
    ) -> EditorOutcome {
        match command {
            EditorCommand::None => EditorOutcome::Continue,
            EditorCommand::Exit => EditorOutcome::Exit,
            EditorCommand::SelectTheme(name) => {
                if let Err(error) = self.apply_stored_theme(store, name) {
                    self.status_message = Some(format!("Failed to load theme: {error}"));
                }
                EditorOutcome::Continue
            }
            EditorCommand::ResetTheme(name) => {
                match self.apply_stored_theme(store, &name) {
                    Ok(()) => self.status_message = Some(format!("Reset to: {name}")),
                    Err(error) => {
                        self.status_message = Some(format!("Failed to reset theme: {error}"));
                    }
                }
                EditorOutcome::Continue
            }
            EditorCommand::SaveConfig => {
                match store.save_config(&self.draft) {
                    Ok(()) => self.mark_saved(),
                    Err(error) => {
                        self.status_message = Some(format!("Failed to save: {error}"));
                    }
                }
                EditorOutcome::Continue
            }
            EditorCommand::SaveTheme => {
                let current_theme = self.draft.theme.clone();
                match store.save_theme(&self.draft) {
                    Ok(()) => {
                        self.status_message =
                            Some(format!("Wrote config to theme: {current_theme}"));
                    }
                    Err(error) => {
                        self.status_message = Some(format!("Failed to write theme: {error}"));
                    }
                }
                EditorOutcome::Continue
            }
            EditorCommand::SaveNewTheme(name) => {
                match store.save_new_theme(&name, &self.draft) {
                    Ok(()) => {
                        self.draft.theme = name.clone();
                        self.selected_theme_baseline = self.draft.clone();
                        self.status_message = Some(format!("Saved as new theme: {name}"));
                    }
                    Err(error) => {
                        self.status_message = Some(format!("Failed to save theme: {error}"));
                    }
                }
                EditorOutcome::Continue
            }
        }
    }

    fn apply_stored_theme(&mut self, store: &CxLineStore, name: &str) -> io::Result<()> {
        let mut theme = store.load_theme(name)?;
        // The selected filename remains authoritative if a hand-edited theme contains a stale
        // `theme` value, matching the legacy `apply_theme(name)` behavior.
        theme.theme = name.to_string();
        self.select_theme(theme);
        Ok(())
    }
}
