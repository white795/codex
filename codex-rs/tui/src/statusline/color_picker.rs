// Derived from Cometix; see this module's mod.rs for provenance.
//! State and input handling for the CxLine color picker.
//!
//! Rendering is staged separately so each migration step remains independently reviewable.

use super::style::AnsiColor;

mod render;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ColorPickerMode {
    Basic16,
    Extended256,
    RgbInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RgbField {
    Red,
    Green,
    Blue,
    Hex,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RgbInput {
    pub(super) r: String,
    pub(super) g: String,
    pub(super) b: String,
    pub(super) hex: String,
    pub(super) editing_field: RgbField,
}

impl Default for RgbInput {
    fn default() -> Self {
        Self {
            r: String::new(),
            g: String::new(),
            b: String::new(),
            hex: String::new(),
            editing_field: RgbField::Red,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ColorTarget {
    Icon,
    Text,
    Background,
}

#[derive(Debug, Clone)]
pub(super) struct ColorPicker {
    pub(super) is_open: bool,
    pub(super) mode: ColorPickerMode,
    pub(super) selected_basic: usize,
    pub(super) selected_extended: usize,
    pub(super) rgb_input: RgbInput,
    current_color: Option<AnsiColor>,
    pub(super) target_field: ColorTarget,
    pub(super) cached_basic_cols: usize,
    pub(super) cached_extended_cols: usize,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            is_open: false,
            mode: ColorPickerMode::Basic16,
            selected_basic: 0,
            selected_extended: 0,
            rgb_input: RgbInput::default(),
            current_color: None,
            target_field: ColorTarget::Icon,
            cached_basic_cols: 8,
            cached_extended_cols: 8,
        }
    }
}

impl ColorPicker {
    pub(super) fn open(&mut self, target: ColorTarget, current: Option<AnsiColor>) {
        self.is_open = true;
        self.target_field = target;
        self.mode = ColorPickerMode::Basic16;
        self.selected_basic = 0;
        self.selected_extended = 0;
        self.rgb_input = RgbInput::default();
        self.current_color = current;
    }

    pub(super) fn close(&mut self) {
        self.is_open = false;
    }

    pub(super) fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            ColorPickerMode::Basic16 => ColorPickerMode::Extended256,
            ColorPickerMode::Extended256 => ColorPickerMode::RgbInput,
            ColorPickerMode::RgbInput => ColorPickerMode::Basic16,
        };
    }

    pub(super) fn move_horizontal(&mut self, delta: i32) {
        match self.mode {
            ColorPickerMode::Basic16 => {
                self.selected_basic = wrapped_horizontal(self.selected_basic, 15, delta);
                self.current_color = Some(AnsiColor::c16(self.selected_basic as u8));
            }
            ColorPickerMode::Extended256 => {
                self.selected_extended = wrapped_horizontal(self.selected_extended, 255, delta);
                self.current_color = Some(AnsiColor::c256(self.selected_extended as u8));
            }
            ColorPickerMode::RgbInput => {
                self.rgb_input.editing_field = match (self.rgb_input.editing_field, delta > 0) {
                    (RgbField::Red, true) => RgbField::Green,
                    (RgbField::Green, true) => RgbField::Blue,
                    (RgbField::Blue, true) => RgbField::Hex,
                    (RgbField::Hex, true) => RgbField::Red,
                    (RgbField::Red, false) => RgbField::Hex,
                    (RgbField::Green, false) => RgbField::Red,
                    (RgbField::Blue, false) => RgbField::Green,
                    (RgbField::Hex, false) => RgbField::Blue,
                };
            }
        }
    }

    pub(super) fn move_vertical(&mut self, delta: i32) {
        match self.mode {
            ColorPickerMode::Basic16 => {
                self.selected_basic =
                    vertical_selection(self.selected_basic, self.cached_basic_cols, 16, delta);
                self.current_color = Some(AnsiColor::c16(self.selected_basic as u8));
            }
            ColorPickerMode::Extended256 => {
                self.selected_extended = vertical_selection(
                    self.selected_extended,
                    self.cached_extended_cols,
                    256,
                    delta,
                );
                self.current_color = Some(AnsiColor::c256(self.selected_extended as u8));
            }
            ColorPickerMode::RgbInput => {}
        }
    }

    pub(super) fn input_char(&mut self, c: char) {
        if self.mode != ColorPickerMode::RgbInput {
            return;
        }
        match self.rgb_input.editing_field {
            RgbField::Red => push_decimal(&mut self.rgb_input.r, c),
            RgbField::Green => push_decimal(&mut self.rgb_input.g, c),
            RgbField::Blue => push_decimal(&mut self.rgb_input.b, c),
            RgbField::Hex => {
                if self.rgb_input.hex.len() < 6 && c.is_ascii_hexdigit() {
                    self.rgb_input.hex.push(c.to_ascii_uppercase());
                }
            }
        }
        self.update_rgb_color();
    }

    pub(super) fn backspace(&mut self) {
        if self.mode != ColorPickerMode::RgbInput {
            return;
        }
        match self.rgb_input.editing_field {
            RgbField::Red => self.rgb_input.r.pop(),
            RgbField::Green => self.rgb_input.g.pop(),
            RgbField::Blue => self.rgb_input.b.pop(),
            RgbField::Hex => self.rgb_input.hex.pop(),
        };
        self.update_rgb_color();
    }

    fn update_rgb_color(&mut self) {
        if self.rgb_input.hex.len() == 6
            && let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&self.rgb_input.hex[0..2], 16),
                u8::from_str_radix(&self.rgb_input.hex[2..4], 16),
                u8::from_str_radix(&self.rgb_input.hex[4..6], 16),
            )
        {
            self.current_color = Some(AnsiColor::rgb(r, g, b));
            return;
        }
        if let (Ok(r), Ok(g), Ok(b)) = (
            self.rgb_input.r.parse::<u8>(),
            self.rgb_input.g.parse::<u8>(),
            self.rgb_input.b.parse::<u8>(),
        ) {
            self.current_color = Some(AnsiColor::rgb(r, g, b));
        }
    }

    pub(super) fn get_selected_color(&self) -> Option<AnsiColor> {
        self.current_color
    }
}

fn wrapped_horizontal(current: usize, last: usize, delta: i32) -> usize {
    if delta > 0 {
        if current < last { current + 1 } else { 0 }
    } else if current > 0 {
        current - 1
    } else {
        last
    }
}

fn vertical_selection(current: usize, columns: usize, count: usize, delta: i32) -> usize {
    let columns = columns.max(1);
    let current_row = current / columns;
    let current_col = current % columns;
    let total_rows = count.div_ceil(columns);
    let new_row = if delta > 0 && current_row + 1 < total_rows {
        current_row + 1
    } else if delta <= 0 && current_row > 0 {
        current_row - 1
    } else {
        current_row
    };
    (new_row * columns + current_col).min(count - 1)
}

fn push_decimal(input: &mut String, c: char) {
    if input.len() < 3 && c.is_ascii_digit() {
        input.push(c);
    }
}

#[cfg(test)]
#[path = "color_picker_tests.rs"]
mod tests;
