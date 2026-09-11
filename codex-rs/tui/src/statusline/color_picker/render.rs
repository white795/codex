// Derived from Cometix; see the parent statusline module for provenance.
//! Rendering for the CxLine color picker.

use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Clear;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Widget;

use super::ColorPicker;
use super::ColorPickerMode;
use super::RgbField;
use crate::statusline::style::AnsiColor;

impl ColorPicker {
    pub(super) fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if !self.is_open || area.is_empty() {
            return;
        }

        let popup_area = centered_rect(/*percent_x*/ 60, /*percent_y*/ 70, area);
        if popup_area.is_empty() {
            return;
        }
        Clear.render(popup_area, buf);

        let popup_block = Block::default().borders(Borders::ALL).title("Color Picker");
        let inner = popup_block.inner(popup_area);
        popup_block.render(popup_area, buf);

        let [mode_area, content_area, preview_area, help_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .areas(inner);

        let mode_text = match self.mode {
            ColorPickerMode::Basic16 => "[•] Basic (16)  [ ] Extended (256)  [ ] RGB",
            ColorPickerMode::Extended256 => "[ ] Basic (16)  [•] Extended (256)  [ ] RGB",
            ColorPickerMode::RgbInput => "[ ] Basic (16)  [ ] Extended (256)  [•] RGB",
        };
        Paragraph::new(mode_text)
            .block(Block::default().borders(Borders::ALL).title("Mode"))
            .render(mode_area, buf);

        match self.mode {
            ColorPickerMode::Basic16 => self.render_basic_colors(content_area, buf),
            ColorPickerMode::Extended256 => self.render_extended_colors(content_area, buf),
            ColorPickerMode::RgbInput => self.render_rgb_input(content_area, buf),
        }

        self.render_preview(preview_area, buf);

        Paragraph::new("[Enter] Select  [Esc] Cancel  [Tab] Cycle Mode")
            .block(Block::default().borders(Borders::ALL))
            .render(help_area, buf);
    }

    fn render_basic_colors(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Basic Colors (ANSI 16)");
        let inner = block.inner(area);
        block.render(area, buf);
        if inner.is_empty() {
            self.cached_basic_cols = 1;
            return;
        }

        let available_width = usize::from(inner.width);
        let available_height = usize::from(inner.height);
        let colors_per_row = (available_width / 6).max(1);
        self.cached_basic_cols = colors_per_row;

        for color_index in 0..16 {
            let row = color_index / colors_per_row;
            let col = color_index % colors_per_row;
            let display_row = row * 2;
            if display_row >= available_height {
                break;
            }

            let x = inner.x + (col * 6) as u16;
            let y = inner.y + display_row as u16;
            let text = if color_index == self.selected_basic {
                "[ ██ ]"
            } else {
                "  ██  "
            };
            buf.set_stringn(
                x,
                y,
                text,
                usize::from(inner.right().saturating_sub(x)),
                Style::default().fg(AnsiColor::c16(color_index as u8).to_ratatui_color()),
            );
        }

        let rows_needed = 16_usize.div_ceil(colors_per_row);
        let display_rows_needed = rows_needed * 2;
        if available_height > display_rows_needed && self.selected_basic < 16 {
            let text = format!(
                "Selected: {} ({})",
                self.selected_basic,
                color_name(self.selected_basic as u8)
            );
            buf.set_stringn(
                inner.x,
                inner.y + display_rows_needed as u16,
                text,
                available_width,
                Style::default().fg(Color::Gray),
            );
        }
    }

    fn render_extended_colors(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Extended Colors (256)");
        let inner = block.inner(area);
        block.render(area, buf);
        if inner.is_empty() {
            self.cached_extended_cols = 1;
            return;
        }

        let available_width = usize::from(inner.width);
        let available_height = usize::from(inner.height);
        let colors_per_row = (available_width / 7).max(1);
        self.cached_extended_cols = colors_per_row;

        let logical_rows_available = if available_height > 3 {
            (available_height - 2) / 2
        } else {
            1
        };
        let colors_per_page = colors_per_row * logical_rows_available;
        let start_index = self.selected_extended / colors_per_page * colors_per_page;
        let end_index = (start_index + colors_per_page).min(256);

        for (offset, color_index) in (start_index..end_index).enumerate() {
            let row = offset / colors_per_row;
            let col = offset % colors_per_row;
            let display_row = row * 2;
            if display_row >= available_height.saturating_sub(2) {
                break;
            }

            let x = inner.x + (col * 7) as u16;
            let y = inner.y + display_row as u16;
            let text = if color_index == self.selected_extended {
                "[ ██ ]"
            } else {
                "  ██  "
            };
            buf.set_stringn(
                x,
                y,
                text,
                usize::from(inner.right().saturating_sub(x)),
                Style::default().fg(AnsiColor::c256(color_index as u8).to_ratatui_color()),
            );
        }

        if available_height > 2 {
            let text = format!(
                "Selected: {} | Use ↑↓←→ to navigate",
                self.selected_extended
            );
            buf.set_stringn(
                inner.x,
                inner.bottom() - 1,
                text,
                available_width,
                Style::default().fg(Color::Gray),
            );
        }
    }

    fn render_rgb_input(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("RGB Input");
        let inner = block.inner(area);
        block.render(area, buf);
        if inner.is_empty() {
            return;
        }

        let rgb_text = format!(
            "R[{}]  G[{}]  B[{}]",
            format_field(
                RgbField::Red,
                &self.rgb_input.r,
                self.rgb_input.editing_field
            ),
            format_field(
                RgbField::Green,
                &self.rgb_input.g,
                self.rgb_input.editing_field
            ),
            format_field(
                RgbField::Blue,
                &self.rgb_input.b,
                self.rgb_input.editing_field
            ),
        );
        buf.set_stringn(
            inner.x,
            inner.y,
            rgb_text,
            usize::from(inner.width),
            Style::default(),
        );

        if inner.height > 2 {
            let hex_text = format!(
                "Hex: #{}",
                format_field(
                    RgbField::Hex,
                    &self.rgb_input.hex,
                    self.rgb_input.editing_field
                )
            );
            buf.set_stringn(
                inner.x,
                inner.y + 2,
                hex_text,
                usize::from(inner.width),
                Style::default(),
            );
        }
    }

    fn render_preview(&self, area: Rect, buf: &mut Buffer) {
        let preview_text = match self.current_color {
            Some(AnsiColor::Color16 { c16 }) => {
                format!("████ Color 16: {} ({})", c16, color_name(c16))
            }
            Some(AnsiColor::Color256 { c256 }) => format!("████ Color 256: {c256}"),
            Some(AnsiColor::Rgb { r, g, b }) => format!("████ RGB: ({r}, {g}, {b})"),
            None => "████ No color selected".to_string(),
        };
        let color = self
            .current_color
            .map(AnsiColor::to_ratatui_color)
            .unwrap_or(Color::Reset);
        Paragraph::new(preview_text)
            .style(Style::default().fg(color))
            .block(Block::default().borders(Borders::ALL).title("Preview"))
            .render(area, buf);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let [_, middle, _] = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .areas(area);
    let [_, popup, _] = Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .areas(middle);
    popup
}

fn format_field(field: RgbField, value: &str, current: RgbField) -> String {
    if field == current {
        format!("> {value} <")
    } else {
        value.to_string()
    }
}

fn color_name(ansi: u8) -> &'static str {
    match ansi {
        0 => "Black",
        1 => "Red",
        2 => "Green",
        3 => "Yellow",
        4 => "Blue",
        5 => "Magenta",
        6 => "Cyan",
        7 => "White",
        8 => "DarkGray",
        9 => "LightRed",
        10 => "LightGreen",
        11 => "LightYellow",
        12 => "LightBlue",
        13 => "LightMagenta",
        14 => "LightCyan",
        15 => "Gray",
        _ => "Unknown",
    }
}
