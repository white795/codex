use super::IconSelector;
use crate::statusline::style::StyleMode;
use pretty_assertions::assert_eq;
use ratatui::buffer::Buffer;
use ratatui::buffer::Cell;
use ratatui::layout::Rect;

#[test]
fn opening_chooses_emoji_or_nerd_font_for_the_current_style() {
    for (style, expected) in [
        (StyleMode::Plain, "🤖"),
        (StyleMode::NerdFont, "\u{e26d}"),
        (StyleMode::Powerline, "\u{e26d}"),
    ] {
        let mut selector = IconSelector::default();
        selector.open(style);
        assert!(selector.is_open);
        assert_eq!(selector.get_selected_icon(), Some(expected.to_string()));
    }
}

#[test]
fn switching_styles_and_reopening_remember_each_lists_selection() {
    let mut selector = IconSelector::default();
    selector.open(StyleMode::Plain);
    selector.move_selection(2);
    let plain = selector.get_selected_icon();
    selector.toggle_style();
    selector.move_selection(1);
    let nerd = selector.get_selected_icon();
    selector.toggle_style();
    assert_eq!(selector.get_selected_icon(), plain);
    selector.toggle_style();
    assert_eq!(selector.get_selected_icon(), nerd);
    selector.close();
    assert!(!selector.is_open);
    selector.open(StyleMode::Plain);
    assert_eq!(selector.get_selected_icon(), plain);
}

#[test]
fn list_navigation_stays_at_the_first_and_last_icons() {
    for style in [StyleMode::Plain, StyleMode::NerdFont] {
        let mut selector = IconSelector::default();
        selector.open(style);
        let first = selector.get_selected_icon();
        selector.move_selection(-1);
        assert_eq!(selector.get_selected_icon(), first);
        selector.move_selection(100);
        let last = selector.get_selected_icon();
        selector.move_selection(1);
        assert_eq!(selector.get_selected_icon(), last);
        assert_ne!(last, first);
    }
}

#[test]
fn custom_unicode_input_is_applied_only_when_finished_and_nonempty() {
    let mut selector = IconSelector::default();
    selector.open(StyleMode::Plain);
    let original = selector.get_selected_icon();
    selector.input_char('x');
    selector.backspace();
    assert_eq!(selector.custom_input, "");
    selector.start_custom_input();
    assert!(!selector.finish_custom_input());
    assert_eq!(selector.get_selected_icon(), original);
    selector.start_custom_input();
    for c in "中文🌙".chars() {
        selector.input_char(c);
    }
    selector.backspace();
    selector.move_selection(1);
    assert_eq!(selector.get_selected_icon(), original);
    assert!(selector.finish_custom_input());
    assert!(!selector.editing_custom);
    assert_eq!(selector.get_selected_icon(), Some("中文".into()));
    selector.start_custom_input();
    selector.input_char('新');
    selector.close();
    selector.open(StyleMode::Plain);
    assert!(!selector.editing_custom);
    assert_eq!(selector.custom_input, "");
    assert_eq!(selector.get_selected_icon(), original);
}

#[test]
fn closed_selector_does_not_modify_the_background() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 80, /*height*/ 32,
    );
    let mut buffer = Buffer::filled(area, Cell::new("x"));
    let before = buffer.clone();
    IconSelector::default().render(area, &mut buffer);
    assert_eq!(buffer, before);
}

#[test]
fn selector_renders_safely_in_small_and_offset_buffers() {
    let mut selector = IconSelector::default();
    selector.open(StyleMode::Plain);
    for width in [0, 1, 2, 8, 24, 80] {
        for height in [0, 1, 2, 8, 24, 40] {
            let area = Rect::new(/*x*/ 7, /*y*/ 5, width, height);
            let mut buffer = Buffer::empty(area);
            selector.render(area, &mut buffer);
            selector.render(
                Rect::new(
                    /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 80,
                ),
                &mut buffer,
            );
        }
    }
}

#[test]
fn narrow_icon_list_does_not_overwrite_content_outside_its_viewport() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 90, /*height*/ 70,
    );
    let viewport = Rect::new(
        /*x*/ 7, /*y*/ 5, /*width*/ 12, /*height*/ 60,
    );
    let mut buffer = Buffer::filled(area, Cell::new("x"));
    let mut selector = IconSelector::default();
    selector.open(StyleMode::Plain);
    selector.render(viewport, &mut buffer);
    for y in area.top()..area.bottom() {
        for x in area.left()..area.right() {
            if !viewport.contains((x, y).into()) {
                assert_eq!(buffer[(x, y)].symbol(), "x", "at ({x}, {y})");
            }
        }
    }
}

#[test]
fn snapshot_emoji_and_nerd_font_lists_keep_the_selected_row_visible() {
    let area = Rect::new(
        /*x*/ 0, /*y*/ 0, /*width*/ 80, /*height*/ 32,
    );
    let mut buffers = Vec::new();
    for style in [StyleMode::Plain, StyleMode::NerdFont] {
        let mut selector = IconSelector::default();
        selector.open(style);
        selector.move_selection(10);
        let mut buffer = Buffer::empty(area);
        selector.render(area, &mut buffer);
        buffers.push(buffer);
    }
    insta::assert_debug_snapshot!("cxline_icon_lists", buffers);
}

#[test]
fn snapshot_custom_input_and_narrow_list_preserve_the_editor_appearance() {
    let mut selector = IconSelector::default();
    selector.open(StyleMode::Plain);
    selector.start_custom_input();
    for c in "中文🌙".chars() {
        selector.input_char(c);
    }
    let mut buffers = Vec::new();
    for width in [80, 32] {
        let area = Rect::new(/*x*/ 0, /*y*/ 0, width, /*height*/ 32);
        let mut buffer = Buffer::empty(area);
        selector.render(area, &mut buffer);
        buffers.push(buffer);
    }
    insta::assert_debug_snapshot!("cxline_icon_custom_input", buffers);
}
