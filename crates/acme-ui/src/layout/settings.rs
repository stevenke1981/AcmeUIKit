use gpui::{
    AnyElement, App, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, StyledExt};

/// A single settings row with label, description, and a control widget.
#[derive(IntoElement)]
pub struct SettingsRow {
    id: gpui::ElementId,
    label: SharedString,
    description: Option<SharedString>,
    control: Option<AnyElement>,
}

impl SettingsRow {
    /// Creates a new settings row with the given `id` and `label`.
    pub fn new(id: impl Into<gpui::ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            control: None,
        }
    }

    /// Sets the optional description text.
    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Attaches a control widget rendered on the right side.
    pub fn control(mut self, control: impl IntoElement) -> Self {
        self.control = Some(control.into_any_element());
        self
    }
}

impl RenderOnce for SettingsRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .h_flex()
            .w_full()
            .h(px(44.))
            .px_3()
            .gap_3()
            .child(
                div()
                    .v_flex()
                    .flex_1()
                    .gap_1()
                    .child(
                        div()
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(self.label),
                    )
                    .when_some(self.description, |this, desc| {
                        this.child(
                            div()
                                .text_size(theme.font_sizes.caption)
                                .text_color(c.muted_foreground)
                                .child(desc),
                        )
                    }),
            )
            .when_some(self.control, |this, control| {
                this.child(div().h_flex().child(control))
            })
    }
}

/// A group of settings rows with a title.
#[derive(IntoElement)]
pub struct SettingsGroup {
    id: gpui::ElementId,
    title: SharedString,
    description: Option<SharedString>,
    rows: Vec<SettingsRow>,
}

impl SettingsGroup {
    /// Creates a new settings group with the given `id` and `title`.
    pub fn new(id: impl Into<gpui::ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            rows: Vec::new(),
        }
    }

    /// Sets the optional group description.
    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Adds a settings row.
    pub fn row(mut self, row: SettingsRow) -> Self {
        self.rows.push(row);
        self
    }

    /// Adds multiple settings rows.
    pub fn rows(mut self, rows: Vec<SettingsRow>) -> Self {
        self.rows.extend(rows);
        self
    }
}

impl RenderOnce for SettingsGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .child(
                div()
                    .v_flex()
                    .px_3()
                    .gap_1()
                    .child(
                        div()
                            .text_size(theme.font_sizes.heading)
                            .text_color(c.foreground)
                            .child(self.title),
                    )
                    .when_some(self.description, |this, desc| {
                        this.child(
                            div()
                                .text_size(theme.font_sizes.caption)
                                .text_color(c.muted_foreground)
                                .child(desc),
                        )
                    }),
            )
            .child(
                div()
                    .v_flex()
                    .mt_2()
                    .border_1()
                    .border_color(c.border)
                    .rounded(theme.radius)
                    .bg(c.surface)
                    .children(self.rows),
            )
    }
}

/// Top-level settings page container.
///
/// Renders a scrollable list of [`SettingsGroup`] sections.
///
/// # Example
///
/// ```ignore
/// SettingsPage::new("app-settings")
///     .groups(vec![
///         SettingsGroup::new("general", "General")
///             .description("Basic application preferences")
///             .row(SettingsRow::new("lang", "Language")
///                 .description("UI display language")
///                 .control(Switch::new("lang-switch", true))),
///     ])
/// ```
#[derive(IntoElement)]
pub struct SettingsPage {
    id: gpui::ElementId,
    groups: Vec<SettingsGroup>,
}

impl SettingsPage {
    /// Creates a new settings page with the given `id`.
    pub fn new(id: impl Into<gpui::ElementId>) -> Self {
        Self {
            id: id.into(),
            groups: Vec::new(),
        }
    }

    /// Sets the settings groups.
    pub fn groups(mut self, groups: Vec<SettingsGroup>) -> Self {
        self.groups = groups;
        self
    }
}

impl RenderOnce for SettingsPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .id(self.id)
            .v_flex()
            .w_full()
            .gap(theme.spacing.section)
            .p(theme.spacing.panel)
            .children(self.groups)
    }
}
