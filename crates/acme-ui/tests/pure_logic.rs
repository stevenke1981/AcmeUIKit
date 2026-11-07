//! Public API regression tests that never create a GPUI application or window.
//! The focused target compiles GPUI without initializing a platform backend.

use acme_ui::{Density, Size, Theme, ThemeMode, Tone, aria_label, disabled_opacity, hsl};
use gpui::px;

#[test]
fn defaults_match_the_initial_light_theme() {
    let theme = Theme::light();
    assert_eq!(ThemeMode::default(), theme.mode);
    assert_eq!(Density::default(), theme.density);
    assert_eq!(Size::default(), Size::Medium);
    assert_eq!(Tone::default(), Tone::Neutral);
    assert_eq!(Theme::dark().mode, ThemeMode::Dark);
}

#[test]
fn hsl_converts_degrees_and_percentages_to_opaque_unit_channels() {
    for (degrees, saturation, lightness) in [(0., 0., 0.), (180., 50., 25.), (360., 100., 100.)] {
        let color = hsl(degrees, saturation, lightness);
        assert!((color.h - degrees / 360.).abs() < f32::EPSILON);
        assert!((color.s - saturation / 100.).abs() < f32::EPSILON);
        assert!((color.l - lightness / 100.).abs() < f32::EPSILON);
        assert_eq!(color.a, 1.);
    }
}

#[test]
fn theme_color_channels_are_finite_and_normalized() {
    for theme in [Theme::light(), Theme::dark()] {
        let c = theme.colors;
        let colors = [
            c.app_background,
            c.panel_background,
            c.elevated_surface,
            c.overlay_surface,
            c.foreground_secondary,
            c.foreground_tertiary,
            c.border_subtle,
            c.border_default,
            c.border_strong,
            c.primary_pressed,
            c.primary_soft,
            c.success_soft,
            c.warning_soft,
            c.danger_soft,
            c.selection,
            c.focus_ring,
            c.background,
            c.surface,
            c.foreground,
            c.muted,
            c.muted_foreground,
            c.border,
            c.primary,
            c.primary_hover,
            c.primary_foreground,
            c.secondary,
            c.secondary_hover,
            c.danger,
            c.danger_hover,
            c.success,
            c.warning,
            c.ring,
        ];
        for (index, color) in colors.into_iter().enumerate() {
            for channel in [color.h, color.s, color.l, color.a] {
                assert!(
                    channel.is_finite() && (0.0..=1.0).contains(&channel),
                    "{:?} color {index} has invalid channel {channel}",
                    theme.mode
                );
            }
            assert_eq!(color.a, 1., "{:?} color {index} is not opaque", theme.mode);
        }
    }
}

#[test]
fn semantic_foregrounds_and_surfaces_follow_theme_mode() {
    let light = Theme::light().colors;
    let dark = Theme::dark().colors;
    assert!(light.background.l > dark.background.l);
    assert!(light.foreground.l < light.background.l);
    assert!(dark.foreground.l > dark.background.l);
    assert!(light.primary_foreground.l > light.primary.l);
    assert!(dark.primary_foreground.l < dark.primary.l);
}

#[test]
fn primitive_heights_match_theme_control_tokens_in_both_modes() {
    for theme in [Theme::light(), Theme::dark()] {
        for (size, height) in [
            (Size::ExtraSmall, theme.controls.xs),
            (Size::Small, theme.controls.small),
            (Size::Medium, theme.controls.medium),
            (Size::Large, theme.controls.large),
        ] {
            assert_eq!(size.height(), height, "{:?} {size:?}", theme.mode);
        }
    }
}

#[test]
fn size_scale_grows_and_leaves_room_for_text() {
    let sizes = [Size::ExtraSmall, Size::Small, Size::Medium, Size::Large];
    for size in sizes {
        assert!(size.text_size() > px(0.));
        assert!(size.height() > size.text_size(), "{size:?} clips its text");
        assert!(size.horizontal_padding() > px(0.));
    }
    for pair in sizes.windows(2) {
        assert!(pair[0].height() < pair[1].height());
        assert!(pair[0].text_size() < pair[1].text_size());
        assert!(pair[0].horizontal_padding() < pair[1].horizontal_padding());
    }
}

#[test]
fn typography_has_positive_sizes_and_room_for_each_line() {
    for theme in [Theme::light(), Theme::dark()] {
        let t = theme.typography;
        for style in [
            t.display,
            t.title_large,
            t.title,
            t.body,
            t.body_compact,
            t.label,
            t.caption,
        ] {
            assert!(style.size > px(0.));
            assert!(style.line_height >= style.size);
        }
        assert!(t.display.size > t.title_large.size);
        assert!(t.title_large.size > t.title.size);
        assert!(t.title.size > t.body.size);
        assert!(t.body.size > t.caption.size);
        assert_eq!(theme.font_sizes.body, t.body.size);
        assert_eq!(theme.font_sizes.caption, t.caption.size);
    }
}

#[test]
fn spacing_radius_and_motion_scales_are_ordered() {
    for theme in [Theme::light(), Theme::dark()] {
        let s = theme.spacing;
        assert!(s.widget > px(0.));
        assert!(s.widget < s.panel && s.panel < s.group && s.group < s.section);
        let r = theme.radius_scale;
        for pair in [r.none, r.xs, r.sm, r.md, r.lg, r.xl, r.pill].windows(2) {
            assert!(pair[0] < pair[1]);
        }
        let m = theme.motion;
        assert_eq!(m.instant, 0);
        assert!(m.instant < m.fast && m.fast < m.normal && m.normal < m.slow);
    }
}

#[test]
fn label_helper_preserves_unicode_and_optional_description() {
    assert_eq!(aria_label("儲存", None).as_ref(), "儲存");
    assert_eq!(
        aria_label("儲存", Some("目前文件")).as_ref(),
        "儲存. 目前文件"
    );
    // Pin existing empty-string behavior without claiming these strings reach a screen reader.
    assert_eq!(aria_label("", None).as_ref(), "");
    assert_eq!(aria_label("Save", Some("")).as_ref(), "Save. ");
}

#[test]
fn disabled_opacity_dims_without_hiding_the_element() {
    let opacity = disabled_opacity();
    assert!(opacity.is_finite() && opacity > 0. && opacity < 1.);
}
