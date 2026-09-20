use super::super::tests::new_test_composer;
use crate::bottom_pane::footer::FooterMode;
use crate::key_hint;
use crate::render::renderable::Renderable;
use crossterm::event::KeyCode;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

#[test]
fn shortcut_overlay_shows_the_paste_hint_for_the_explicit_platform() {
    for (is_wsl, key) in [
        (false, key_hint::ctrl(KeyCode::Char('v'))),
        (true, key_hint::ctrl_alt(KeyCode::Char('v'))),
    ] {
        let (mut composer, _events) = new_test_composer();
        composer.footer.is_wsl = is_wsl;
        composer.footer.mode = FooterMode::ShortcutOverlay;
        let area = Rect::new(
            /*x*/ 0, /*y*/ 0, /*width*/ 100, /*height*/ 20,
        );
        let mut buffer = Buffer::empty(area);
        composer.render(area, &mut buffer);
        let screen = buffer
            .content
            .iter()
            .map(ratatui::buffer::Cell::symbol)
            .collect::<String>();
        let expected_hint = format!("{} to paste images", key.display_label());
        assert!(
            screen.contains(&expected_hint),
            "expected {expected_hint:?} in {screen:?}"
        );
    }
}
