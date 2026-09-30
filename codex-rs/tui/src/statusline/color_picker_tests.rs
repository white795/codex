use super::ColorPicker;
use super::ColorPickerMode;
use super::ColorTarget;
use super::RgbField;
use crate::statusline::style::AnsiColor;
use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::buffer::Cell;
use ratatui::layout::Rect;

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
#[test]
fn closed_picker_does_not_modify_the_background() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 40,
    );
    let mut buffer = Buffer::filled(area, Cell::new("x"));
    let before = buffer.clone();
    ColorPicker::default().render(area, &mut buffer);
    assert_eq!(buffer, before);
}

#[test]
fn picker_renders_safely_in_small_offset_and_intersected_buffers() {
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Icon, /*current*/ None);
    for width in [0, 1, 2, 8, 24, 80] {
        for height in [0, 1, 2, 8, 24, 50] {
            let area = Rect::new(/*x*/ 7, /*y*/ 5, width, height);
            let mut buffer = Buffer::empty(area);
            picker.render(area, &mut buffer);
            picker.render(
                Rect::new(
                    /*x*/ 0, /*y*/ 0, /*width*/ 120, /*height*/ 80,
                ),
                &mut buffer,
            );
        }
    }
}

#[test]
fn narrow_picker_does_not_overwrite_content_outside_its_viewport() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 60,
    );
    let viewport = Rect::new(
        /*x*/ 7, /*y*/ 5, /*width*/ 20, /*height*/ 45,
    );
    let mut buffer = Buffer::filled(area, Cell::new("x"));
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Icon, /*current*/ None);
    picker.render(viewport, &mut buffer);
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            if !viewport.contains((x, y).into()) {
                assert_eq!(buffer[(x, y)].symbol(), "x", "at ({x}, {y})");
            }
        }
    }
}

#[test]
fn rendering_updates_palette_columns_and_uses_the_selected_foreground() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 40,
    );
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Icon, /*current*/ None);
    for _ in 0..4 {
        picker.move_horizontal(/*delta*/ 1);
    }
    let mut basic = Buffer::empty(area);
    picker.render(area, &mut basic);
    assert!(picker.cached_basic_cols > 0);
    assert!(has_colored_block(&basic, AnsiColor::c16(4)));

    picker.cycle_mode();
    for _ in 0..200 {
        picker.move_horizontal(/*delta*/ 1);
    }
    let mut extended = Buffer::empty(area);
    picker.render(area, &mut extended);
    assert!(picker.cached_extended_cols > 0);
    assert!(has_colored_block(&extended, AnsiColor::c256(200)));
}

#[test]
fn color_picker_modes_preserve_the_legacy_layout() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 40,
    );
    let mut picker = ColorPicker::default();
    picker.open(ColorTarget::Text, /*current*/ None);
    for _ in 0..10 {
        picker.move_horizontal(/*delta*/ 1);
    }
    let basic = render_text(&mut picker, area);

    picker.cycle_mode();
    for _ in 0..200 {
        picker.move_horizontal(/*delta*/ 1);
    }
    let extended = render_text(&mut picker, area);

    picker.cycle_mode();
    for c in "255".chars() {
        picker.input_char(c);
    }
    picker.move_horizontal(/*delta*/ 1);
    for c in "128".chars() {
        picker.input_char(c);
    }
    picker.move_horizontal(/*delta*/ 1);
    picker.input_char('0');
    let rgb = render_text(&mut picker, area);
    assert_eq!(
        picker.get_selected_color(),
        Some(AnsiColor::rgb(255, 128, 0))
    );

    insta::assert_snapshot!(
        "cxline_color_picker_modes",
        format!("BASIC 16\n{basic}\n\nEXTENDED 256\n{extended}\n\nRGB\n{rgb}")
    );
}

fn has_colored_block(buffer: &Buffer, color: AnsiColor) -> bool {
    let expected = color.to_ratatui_color();
    buffer
        .content()
        .iter()
        .any(|cell| cell.symbol() == "█" && cell.fg == expected)
}

fn render_text(picker: &mut ColorPicker, area: Rect) -> String {
    let mut buffer = Buffer::empty(area);
    picker.render(area, &mut buffer);
    let Some((left, top, right, bottom)) = non_blank_bounds(&buffer) else {
        return String::new();
    };
    (top..=bottom)
        .map(|y| {
            let mut line = (left..=right)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>();
            let trimmed = line.trim_end().len();
            line.truncate(trimmed);
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn non_blank_bounds(buffer: &Buffer) -> Option<(u16, u16, u16, u16)> {
    let area = *buffer.area();
    let mut bounds: Option<(u16, u16, u16, u16)> = None;
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            if buffer[(x, y)].symbol() != " " {
                bounds = Some(match bounds {
                    Some((left, top, right, bottom)) => {
                        (left.min(x), top.min(y), right.max(x), bottom.max(y))
                    }
                    None => (x, y, x, y),
                });
            }
        }
    }
    bounds
}
