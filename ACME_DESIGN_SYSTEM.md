# AcmeUIKit Visual System V2

AcmeUIKit is a quiet, precise desktop design system for GPUI applications.

## Tokens

`Theme` exposes Light/Dark semantic colors, typography, spacing, radius, control
sizes, density, motion, and elevation hints. Components should consume these
tokens instead of defining visual values locally.

## Density

- `Compact`: toolbars, IDE and dense data views.
- `Comfortable`: default application screens.
- `Spacious`: welcome and onboarding screens.

## Component states

Interactive components expose stable IDs and support enabled/disabled, selected,
focus-visible, loading, and invalid presentation where applicable.

## Accessibility and QA checklist

- Keyboard focus is visible and does not rely on color alone.
- Light and Dark themes remain readable.
- Long Traditional Chinese labels are allowed to expand.
- Validate Gallery at 1280×800, 1024×700, and 800×600.
- Run `cargo fmt`, `cargo check`, `cargo clippy -D warnings`, and `cargo test`.

Full screenshot regression, high-DPI capture, and manual Windows smoke testing
remain release activities.
