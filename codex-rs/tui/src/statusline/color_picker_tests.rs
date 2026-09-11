use super::ColorPicker;
use super::ColorPickerMode;
use super::ColorTarget;
use super::RgbField;
use crate::statusline::style::AnsiColor;
use pretty_assertions::assert_eq;

#[test]
fn opening_resets_the_editor_and_preserves_the_supplied_color() {
    let mut picker = ColorPicker::default();
    picker.cycle_mode();
    for _ in 0..5 {
        picker.move_horizontal(/*delta*/ 1);
    }
    picker.cycle_mode();
    picker.input_char('9');

    picker.open(ColorTarget::Background, Some(AnsiColor::rgb(12, 34, 56)));

    assert!(picker.is_open);
    assert_eq!(picker.mode, ColorPickerMode::Basic16);
    assert_eq!(picker.selected_basic, 0);
    assert_eq!(picker.selected_extended, 0);
    assert_eq!(picker.rgb_input, Default::default());
    assert_eq!(picker.target_field, ColorTarget::Background);
    assert_eq!(
        picker.get_selected_color(),
        Some(AnsiColor::rgb(12, 34, 56))
    );
}

#[test]
fn mode_selection_cycles_in_both_directions_for_rgb_fields() {
    let mut picker = ColorPicker::default();
    assert_eq!(picker.mode, ColorPickerMode::Basic16);
    picker.cycle_mode();
    assert_eq!(picker.mode, ColorPickerMode::Extended256);
    picker.cycle_mode();
    assert_eq!(picker.mode, ColorPickerMode::RgbInput);

    assert_eq!(picker.rgb_input.editing_field, RgbField::Red);
    picker.move_horizontal(/*delta*/ 1);
    assert_eq!(picker.rgb_input.editing_field, RgbField::Green);
    picker.move_horizontal(/*delta*/ 1);
    assert_eq!(picker.rgb_input.editing_field, RgbField::Blue);
    picker.move_horizontal(/*delta*/ 1);
    assert_eq!(picker.rgb_input.editing_field, RgbField::Hex);
    picker.move_horizontal(/*delta*/ 1);
    assert_eq!(picker.rgb_input.editing_field, RgbField::Red);
    picker.move_horizontal(/*delta*/ -1);
    assert_eq!(picker.rgb_input.editing_field, RgbField::Hex);

    picker.cycle_mode();
    assert_eq!(picker.mode, ColorPickerMode::Basic16);
}

#[test]
fn horizontal_palette_navigation_wraps_at_both_ends() {
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Icon, /*current*/ None);
    picker.move_horizontal(/*delta*/ -1);
    assert_eq!(picker.selected_basic, 15);
    assert_eq!(picker.get_selected_color(), Some(AnsiColor::c16(15)));
    picker.move_horizontal(/*delta*/ 1);
    assert_eq!(picker.selected_basic, 0);
    assert_eq!(picker.get_selected_color(), Some(AnsiColor::c16(0)));

    picker.cycle_mode();
    picker.move_horizontal(/*delta*/ -1);
    assert_eq!(picker.selected_extended, 255);
    assert_eq!(picker.get_selected_color(), Some(AnsiColor::c256(255)));
    picker.move_horizontal(/*delta*/ 1);
    assert_eq!(picker.selected_extended, 0);
    assert_eq!(picker.get_selected_color(), Some(AnsiColor::c256(0)));
}

#[test]
fn vertical_palette_navigation_uses_the_last_rendered_column_count() {
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Text, /*current*/ None);
    picker.cached_basic_cols = 6;
    for _ in 0..5 {
        picker.move_horizontal(/*delta*/ 1);
    }
    picker.move_vertical(/*delta*/ 1);
    assert_eq!(picker.selected_basic, 11);
    picker.move_vertical(/*delta*/ 1);
    assert_eq!(picker.selected_basic, 15);
    picker.move_vertical(/*delta*/ -1);
    assert_eq!(picker.selected_basic, 9);

    picker.cycle_mode();
    picker.cached_extended_cols = 10;
    for _ in 0..5 {
        picker.move_horizontal(/*delta*/ 1);
    }
    picker.move_vertical(/*delta*/ 1);
    assert_eq!(picker.selected_extended, 15);
    picker.move_vertical(/*delta*/ -1);
    assert_eq!(picker.selected_extended, 5);
    picker.move_vertical(/*delta*/ -1);
    assert_eq!(picker.selected_extended, 5);
}

#[test]
fn rgb_and_hex_inputs_accept_only_their_legacy_character_sets_and_limits() {
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Text, /*current*/ None);
    picker.cycle_mode();
    picker.cycle_mode();

    for c in "255x9".chars() {
        picker.input_char(c);
    }
    assert_eq!(picker.rgb_input.r, "255");
    assert_eq!(picker.get_selected_color(), None);
    picker.move_horizontal(/*delta*/ 1);
    for c in "000".chars() {
        picker.input_char(c);
    }
    picker.move_horizontal(/*delta*/ 1);
    for c in "128".chars() {
        picker.input_char(c);
    }
    assert_eq!(
        picker.get_selected_color(),
        Some(AnsiColor::rgb(255, 0, 128))
    );

    picker.move_horizontal(/*delta*/ 1);
    for c in "1a2B3cz".chars() {
        picker.input_char(c);
    }
    assert_eq!(picker.rgb_input.hex, "1A2B3C");
    assert_eq!(
        picker.get_selected_color(),
        Some(AnsiColor::rgb(0x1A, 0x2B, 0x3C))
    );
    picker.backspace();
    assert_eq!(picker.rgb_input.hex, "1A2B3");
    assert_eq!(
        picker.get_selected_color(),
        Some(AnsiColor::rgb(255, 0, 128))
    );
}

#[test]
fn palette_modes_ignore_text_input_and_rgb_mode_ignores_vertical_navigation() {
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Icon, Some(AnsiColor::c16(4)));
    picker.input_char('9');
    picker.backspace();
    assert_eq!(picker.rgb_input, Default::default());
    assert_eq!(picker.get_selected_color(), Some(AnsiColor::c16(4)));

    picker.cycle_mode();
    picker.cycle_mode();
    picker.move_vertical(/*delta*/ 1);
    picker.move_vertical(/*delta*/ -1);
    assert_eq!(picker.rgb_input.editing_field, RgbField::Red);
}

#[test]
fn closing_only_hides_the_picker_until_the_next_explicit_open() {
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Icon, /*current*/ None);
    for _ in 0..3 {
        picker.move_horizontal(/*delta*/ 1);
    }
    let selected = picker.get_selected_color();
    picker.close();
    assert!(!picker.is_open);
    assert_eq!(picker.get_selected_color(), selected);
    assert_eq!(picker.target_field, ColorTarget::Icon);
}
