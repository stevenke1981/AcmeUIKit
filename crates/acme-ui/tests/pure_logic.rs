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

use acme_ui::{Scale, ScaleType, validators};

#[test]
fn linear_scales_map_and_invert_negative_domains_and_reversed_ranges() {
    for (start, end) in [(20., 220.), (220., 20.)] {
        let scale = Scale::linear().domain(-30., -10.).range(start, end);
        assert_eq!(scale.map(-30.), start);
        assert_eq!(scale.map(-10.), end);
        assert_eq!(scale.map(-20.), 120.);
        for value in [-40., -30., -23.5, -10., 0.] {
            assert!((scale.invert(scale.map(value)) - value).abs() < 1e-10);
        }
    }
    assert_eq!(Scale::default().map(50.), 0.5);
    assert_eq!(Scale::linear().band_width(), 0.);
}

#[test]
fn degenerate_scales_use_unit_denominators() {
    let scale = Scale::linear().domain(5., 5.).range(10., 30.);
    assert_eq!(scale.map(5.), 10.);
    assert_eq!(scale.map(6.), 30.);
    assert_eq!(scale.invert(100.), 5.);
    assert_eq!(scale.ticks(3), vec![5., 5., 5.]);
    let flat_range = Scale::linear().domain(-10., 10.).range(4., 4.);
    assert_eq!(flat_range.map(0.), 4.);
    assert_eq!(flat_range.invert(4.), -10.);
    assert_eq!(flat_range.invert(5.), 10.);
}

#[test]
fn bands_space_categories_and_preserve_range_direction() {
    let scale = Scale::band(4).range(10., 90.);
    assert_eq!(scale.band_width(), 20.);
    assert_eq!(scale.map(0.), 10.);
    assert_eq!(scale.map(3.), 70.);
    assert_eq!(scale.invert(50.), 2.);
    assert_eq!(Scale::band(4).range(90., 10.).band_width(), -20.);
    let empty = Scale::band(0).range(10., 90.);
    assert_eq!(empty.band_width(), 0.);
    assert_eq!(empty.map(0.), 10.);
    assert_eq!(empty.invert(50.), 0.);
}

#[test]
fn ticks_include_endpoints_and_clamp_count_to_two() {
    let scale = Scale::linear().domain(-8., 4.);
    assert_eq!(scale.ticks(5), vec![-8., -5., -2., 1., 4.]);
    for count in [0, 1, 2] {
        assert_eq!(scale.ticks(count), vec![-8., 4.]);
    }
    assert_eq!(Scale::linear().domain(4., -8.).ticks(3), vec![4., -2., -8.]);
}

#[test]
fn scale_type_variants_remain_distinct_when_cloned() {
    let types = [ScaleType::Linear, ScaleType::Band];
    let cloned = types.clone();
    assert_eq!(cloned, types);
    assert_ne!(types[0], types[1]);
}

#[test]
fn required_validation_trims_whitespace_without_rejecting_zero() {
    let rule = validators::required();
    for value in ["", " \t\n", "\u{3000}"] {
        assert_eq!(rule(value).as_deref(), Some("This field is required"));
    }
    for value in ["0", " 儲存 "] {
        assert!(rule(value).is_none());
    }
}

#[test]
fn length_validation_counts_characters_and_includes_boundaries() {
    let min = validators::min_length(3);
    let max = validators::max_length(3);
    assert_eq!(min("ab").as_deref(), Some("Must be at least 3 characters"));
    assert!(min("abc").is_none());
    assert!(max("abc").is_none());
    assert_eq!(max("abcd").as_deref(), Some("Must be at most 3 characters"));
    // Multi-byte characters count once, matching the "characters" wording of the messages.
    assert!(validators::min_length(1)("貓").is_none());
    assert!(validators::max_length(1)("貓").is_none());
    assert!(validators::min_length(3)("貓貓貓").is_none());
    assert!(validators::max_length(3)("貓貓貓").is_none());
    assert!(validators::min_length(4)("貓貓貓").is_some());
    assert!(validators::max_length(2)("貓貓貓").is_some());
    assert!(validators::min_length(0)("").is_none());
    assert!(validators::max_length(0)("").is_none());
    assert!(validators::max_length(0)("a").is_some());
}

#[test]
fn pattern_validation_honors_anchors_and_reports_custom_messages() {
    let digits = validators::pattern(r"^\d{3}$", "Three digits required");
    assert!(digits("123").is_none());
    for value in ["", "12", "1234", "x123", "123\n"] {
        assert_eq!(digits(value).as_deref(), Some("Three digits required"));
    }
    assert!(validators::pattern("cat", "missing")("a cat naps").is_none());
}

#[test]
fn email_validation_rejects_missing_parts_and_whitespace() {
    let email = validators::email();
    for value in ["a@b.co", "first+tag@example.com"] {
        assert!(email(value).is_none(), "{value}");
    }
    for value in ["", "a", "@b.co", "a@b", "a@@b.co", " a@b.co", "a@b.co\n"] {
        assert_eq!(
            email(value).as_deref(),
            Some("Must be a valid email address")
        );
    }
}
