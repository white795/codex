// Derived from Cometix; see this module's mod.rs for provenance.
// 分隔符编辑器组件

use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

#[derive(Debug, Clone)]
pub struct SeparatorPreset {
    pub name: &'static str,
    pub value: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Default)]
pub struct SeparatorEditor {
    pub is_open: bool,
    pub input: String,
    pub selected_preset: Option<usize>,
}

impl SeparatorEditor {
    pub fn presets() -> Vec<SeparatorPreset> {
        vec![
            SeparatorPreset {
                name: "Pipe",
                value: " | ",
                description: "Classic pipe",
            },
            SeparatorPreset {
                name: "Thin",
                value: " │ ",
                description: "Thin vertical line",
            },
            SeparatorPreset {
                name: "Arrow",
                value: "\u{e0b0}",
                description: "Powerline arrow",
            },
            SeparatorPreset {
                name: "Space",
                value: "  ",
                description: "Double space",
            },
            SeparatorPreset {
                name: "Dot",
                value: " • ",
                description: "Middle dot",
            },
        ]
    }

    pub fn open(&mut self, current_separator: &str) {
        self.is_open = true;
        self.input = current_separator.to_string();
        self.selected_preset = None;

        let presets = Self::presets();
        for (i, preset) in presets.iter().enumerate() {
            if preset.value == current_separator {
                self.selected_preset = Some(i);
                break;
            }
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.input.clear();
        self.selected_preset = None;
    }

    pub fn input_char(&mut self, c: char) {
        if !c.is_control() {
            self.input.push(c);
            self.selected_preset = None;
        }
    }

    pub fn backspace(&mut self) {
        self.input.pop();
        self.selected_preset = None;
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.selected_preset = None;
    }

    pub fn move_preset_selection(&mut self, delta: i32) {
        let presets = Self::presets();
        let new_selection = if let Some(current) = self.selected_preset {
            let new_idx = (current as i32 + delta).clamp(0, presets.len() as i32 - 1) as usize;
            Some(new_idx)
        } else if delta > 0 {
            Some(0)
        } else {
            Some(presets.len() - 1)
        };

        self.selected_preset = new_selection;
        if let Some(idx) = new_selection {
            self.input = presets[idx].value.to_string();
        }
    }

    pub fn get_separator(&self) -> String {
        self.input.clone()
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "Preserve the existing CxLine editor's yellow input styling during migration."
    )]
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if !self.is_open || area.is_empty() {
            return;
        }

        let popup_height = area.height.min(16);
        let popup_width = area.width.min(55);
        let popup_area = Rect {
            x: area.x + (area.width - popup_width) / 2,
            y: area.y + (area.height - popup_height) / 2,
            width: popup_width,
            height: popup_height,
        };

        Clear.render(popup_area, buf);

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .title("Separator Editor");
        let inner = popup_block.inner(popup_area);
        popup_block.render(popup_area, buf);

        let [input_area, presets_area, help_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .areas(inner);

        // Current input
        Paragraph::new(format!("> {} <", self.input))
            .yellow()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Current Separator"),
            )
            .render(input_area, buf);

        // Presets
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Presets (↑↓ to select)");
        let presets_inner = block.inner(presets_area);
        block.render(presets_area, buf);

        let presets = Self::presets();
        for (i, preset) in presets.iter().enumerate() {
            let y = presets_inner.y + i as u16;
            if y >= presets_inner.y + presets_inner.height {
                break;
            }

            let marker = if Some(i) == self.selected_preset {
                "[•]"
            } else {
                "[ ]"
            };
            let text = format!("{marker} {} - {}", preset.name, preset.description);
            buf.set_stringn(
                presets_inner.x,
                y,
                &text,
                usize::from(presets_inner.width),
                Style::default(),
            );
        }

        // Help
        Paragraph::new("[Enter] Confirm  [Esc] Cancel  [Tab] Clear")
            .block(Block::default().borders(Borders::ALL))
            .render(help_area, buf);
    }
}
