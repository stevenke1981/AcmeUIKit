use acme_ui::{RovingOrientation, RovingTabIndex};
use gpui::Modifiers;

fn press(nav: &mut RovingTabIndex, key: &str) -> Option<usize> {
    nav.handle_key(key, &Modifiers::default())
}

#[test]
fn gpui_keys_respect_orientation() {
    for (orientation, next, previous, ignored) in [
        (RovingOrientation::Horizontal, "right", "left", "down"),
        (RovingOrientation::Vertical, "down", "up", "right"),
        (RovingOrientation::Both, "down", "left", "tab"),
        (RovingOrientation::Both, "right", "up", "escape"),
    ] {
        let mut nav = RovingTabIndex::new(5).orientation(orientation).focus(2);
        assert_eq!(press(&mut nav, next), Some(3));
        assert_eq!(press(&mut nav, previous), Some(2));
        assert_eq!(press(&mut nav, ignored), None);
        assert_eq!(nav.focused_index, 2);
    }
}

#[test]
fn legacy_and_gpui_names_are_case_insensitive() {
    for (key, expected) in [
        ("ArrowDown", 3),
        ("ARROWUP", 1),
        ("aRrOwRiGhT", 3),
        ("ArrowLeft", 1),
        ("DOWN", 3),
        ("UP", 1),
        ("RIGHT", 3),
        ("LEFT", 1),
        ("Home", 0),
        ("END", 4),
    ] {
        let mut nav = RovingTabIndex::new(5)
            .orientation(RovingOrientation::Both)
            .focus(2);
        assert_eq!(press(&mut nav, key), Some(expected), "{key}");
        assert_eq!(nav.focused_index, expected);
    }
}

#[test]
fn arrows_wrap_or_clamp_at_both_boundaries() {
    for wrap in [false, true] {
        for (forward, backward) in [("down", "up"), ("right", "left")] {
            let mut nav = RovingTabIndex::new(5)
                .orientation(RovingOrientation::Both)
                .wrap(wrap)
                .focus(4);
            assert_eq!(press(&mut nav, forward), Some(if wrap { 0 } else { 4 }));
            nav = nav.focus(0);
            assert_eq!(press(&mut nav, backward), Some(if wrap { 4 } else { 0 }));
        }
    }
}

#[test]
fn home_and_end_are_absolute_regardless_of_wrap() {
    for wrap in [false, true] {
        for orientation in [
            RovingOrientation::Horizontal,
            RovingOrientation::Vertical,
            RovingOrientation::Both,
        ] {
            for (key, expected) in [("home", 0), ("end", 4), ("Home", 0), ("End", 4)] {
                let mut nav = RovingTabIndex::new(5)
                    .orientation(orientation.clone())
                    .wrap(wrap)
                    .focus(2);
                assert_eq!(press(&mut nav, key), Some(expected));
                assert_eq!(nav.focused_index, expected);
            }
        }
    }
}

#[test]
fn unhandled_keys_preserve_state() {
    let mut nav = RovingTabIndex::new(5).focus(2);
    for key in [
        "tab",
        "escape",
        "enter",
        "a",
        "",
        "left",
        "right",
        "ArrowLeft",
        "ArrowRight",
    ] {
        assert_eq!(press(&mut nav, key), None);
        assert_eq!(nav.focused_index, 2);
    }
    let mut nav = nav.orientation(RovingOrientation::Horizontal);
    for key in ["up", "down", "ArrowUp", "ArrowDown"] {
        assert_eq!(press(&mut nav, key), None);
        assert_eq!(nav.focused_index, 2);
    }
}

#[test]
fn focus_clamps_without_integer_truncation() {
    for (count, index, expected) in [
        (5, 2, 2),
        (5, 5, 4),
        (5, usize::MAX, 4),
        (0, 0, 0),
        (0, usize::MAX, 0),
        (1, usize::MAX, 0),
        (usize::MAX, usize::MAX, usize::MAX - 1),
    ] {
        assert_eq!(
            RovingTabIndex::new(count).focus(index).focused_index,
            expected
        );
    }
    let mut nav = RovingTabIndex::new(usize::MAX).focus(usize::MAX);
    assert_eq!(press(&mut nav, "down"), Some(usize::MAX - 1));
    nav = nav.wrap(true);
    assert_eq!(press(&mut nav, "down"), Some(0));
    assert_eq!(press(&mut nav, "up"), Some(usize::MAX - 1));
}

#[test]
fn empty_collection_never_handles_keys_or_has_a_tab_stop() {
    for wrap in [false, true] {
        let mut nav = RovingTabIndex::new(0)
            .orientation(RovingOrientation::Both)
            .wrap(wrap);
        // Public fields can be changed by callers: even that state must be preserved.
        nav.focused_index = 7;
        for key in [
            "down",
            "up",
            "left",
            "right",
            "home",
            "end",
            "ArrowDown",
            "Home",
            "tab",
        ] {
            assert_eq!(press(&mut nav, key), None);
            assert_eq!(nav.focused_index, 7);
        }
        for index in [0, 1, 7, usize::MAX] {
            assert_eq!(nav.tab_index_str(index), "-1");
        }
    }
}

#[test]
fn single_item_remains_the_only_tab_stop() {
    for wrap in [false, true] {
        let mut nav = RovingTabIndex::new(1)
            .orientation(RovingOrientation::Both)
            .wrap(wrap);
        for key in ["down", "up", "left", "right", "home", "end"] {
            assert_eq!(press(&mut nav, key), Some(0));
            assert_eq!(nav.focused_index, 0);
        }
        assert_eq!(nav.tab_index_str(0), "0");
        assert_eq!(nav.tab_index_str(1), "-1");
        assert_eq!(nav.tab_index_str(usize::MAX), "-1");
    }
}
