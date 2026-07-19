//! Example 1: 設定中心 (Settings Center)
//!
//! Demonstrates: Sidebar + SettingsPage/SettingsGroup/SettingsRow + Dialog + Button + Switch

use acme_ui::{
    ActiveTheme, Button, Dialog, SettingsGroup, SettingsPage, SettingsRow, Sidebar, StyledExt,
    Switch,
};
use gpui::{
    Context, InteractiveElement as _, IntoElement, ParentElement as _, Render, SharedString,
    StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

const CATEGORIES: &[&str] = &["Profile", "Notifications", "Appearance", "Security"];

pub struct SettingsCenter {
    selected: usize,
    dialog_open: bool,
    dialog_title: SharedString,
    dialog_message: SharedString,
    // Toggles
    email_notif: bool,
    push_notif: bool,
    dark_mode: bool,
    compact_mode: bool,
}

impl SettingsCenter {
    pub fn new() -> Self {
        Self {
            selected: 0,
            dialog_open: false,
            dialog_title: SharedString::default(),
            dialog_message: SharedString::default(),
            email_notif: true,
            push_notif: false,
            dark_mode: false,
            compact_mode: false,
        }
    }

    fn render_profile(&self, cx: &gpui::App) -> gpui::AnyElement {
        let c = cx.theme().colors;
        SettingsPage::new("profile-page")
            .groups(vec![
                SettingsGroup::new("personal-info", "Personal Information")
                    .description("Manage your personal details and contact information.")
                    .rows(vec![
                        SettingsRow::new("row-username", "Username")
                            .description("Your unique display name")
                            .control(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(c.muted)
                                    .text_color(c.foreground)
                                    .child("johndoe"),
                            ),
                        SettingsRow::new("row-email", "Email")
                            .description("Primary email address")
                            .control(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(c.muted)
                                    .text_color(c.foreground)
                                    .child("john@example.com"),
                            ),
                        SettingsRow::new("row-role", "Role")
                            .description("Your access level")
                            .control(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(c.primary)
                                    .text_color(c.primary_foreground)
                                    .text_size(px(11.))
                                    .child("Administrator"),
                            ),
                    ]),
                SettingsGroup::new("preferences", "Preferences")
                    .description("Application behavior preferences.")
                    .rows(vec![
                        SettingsRow::new("row-language", "Language")
                            .description("Interface language")
                            .control(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(c.muted)
                                    .text_color(c.foreground)
                                    .child("English (US)"),
                            ),
                        SettingsRow::new("row-timezone", "Timezone")
                            .description("Date and time display")
                            .control(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(c.muted)
                                    .text_color(c.foreground)
                                    .child("UTC+8 (Taipei)"),
                            ),
                    ]),
            ])
            .into_any_element()
    }

    fn render_notifications(&self, _cx: &gpui::App) -> gpui::AnyElement {
        SettingsPage::new("notif-page")
            .groups(vec![
                SettingsGroup::new("email-group", "Email Notifications")
                    .description("Control which updates you receive via email.")
                    .rows(vec![
                        SettingsRow::new("row-order-update", "Order Updates")
                            .description("Get notified when order status changes")
                            .control(Switch::new("sw-order", self.email_notif)),
                        SettingsRow::new("row-promo", "Promotions")
                            .description("Receive promotional offers and news")
                            .control(Switch::new("sw-promo", self.push_notif)),
                        SettingsRow::new("row-security-alert", "Security Alerts")
                            .description("Critical security notifications (always on)")
                            .control(Switch::new("sw-security", true).disabled(true)),
                    ]),
                SettingsGroup::new("push-group", "Push Notifications")
                    .description("Desktop and mobile push notification settings.")
                    .rows(vec![
                        SettingsRow::new("row-push-all", "All Notifications")
                            .description("Receive all push notifications")
                            .control(Switch::new("sw-push-all", self.push_notif)),
                        SettingsRow::new("row-push-mentions", "Mentions Only")
                            .description("Only when someone mentions you")
                            .control(Switch::new("sw-mentions", true)),
                    ]),
            ])
            .into_any_element()
    }

    fn render_appearance(&self, cx: &gpui::App) -> gpui::AnyElement {
        SettingsPage::new("appearance-page")
            .groups(vec![
                SettingsGroup::new("theme-group", "Theme")
                    .description("Customize the look and feel of the application.")
                    .rows(vec![
                        SettingsRow::new("row-dark-mode", "Dark Mode")
                            .description("Switch between light and dark themes")
                            .control(Switch::new("sw-dark", self.dark_mode)),
                        SettingsRow::new("row-compact", "Compact Mode")
                            .description("Reduce spacing and font sizes")
                            .control(Switch::new("sw-compact", self.compact_mode)),
                        SettingsRow::new("row-accent", "Accent Color")
                            .description("Primary brand color")
                            .control(
                                div().h_flex().gap_1().children(
                                    [0x3b82f6, 0xef4444, 0x10b981, 0xf59e0b, 0x8b5cf6]
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, hex)| {
                                            div()
                                                .id(gpui::ElementId::Name(
                                                    format!("accent-{i}").into(),
                                                ))
                                                .size(px(20.))
                                                .rounded_full()
                                                .cursor_pointer()
                                                .bg(gpui::rgba(hex))
                                        }),
                                ),
                            ),
                    ]),
                SettingsGroup::new("layout-group", "Layout")
                    .description("Interface density and arrangement preferences.")
                    .rows(vec![
                        SettingsRow::new("row-sidebar", "Sidebar Position")
                            .description("Left or right sidebar placement")
                            .control(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().colors.muted)
                                    .text_color(cx.theme().colors.foreground)
                                    .child("Left"),
                            ),
                    ]),
            ])
            .into_any_element()
    }

    fn render_security(&self, cx: &gpui::App) -> gpui::AnyElement {
        SettingsPage::new("security-page")
            .groups(vec![
                SettingsGroup::new("password-group", "Password")
                    .description("Update your account password.")
                    .rows(vec![
                        SettingsRow::new("row-current-pw", "Current Password")
                            .description("Enter your existing password")
                            .control(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().colors.muted)
                                    .text_color(cx.theme().colors.muted_foreground)
                                    .child("••••••••"),
                            ),
                        SettingsRow::new("row-new-pw", "New Password")
                            .description("Must be at least 8 characters")
                            .control(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(cx.theme().colors.muted)
                                    .text_color(cx.theme().colors.muted_foreground)
                                    .text_size(px(11.))
                                    .child("Minimum 8 characters"),
                            ),
                    ]),
                SettingsGroup::new("session-group", "Active Sessions")
                    .description("Manage your logged-in devices and sessions.")
                    .rows(vec![
                        SettingsRow::new("row-session-1", "Windows Desktop")
                            .description("Taipei, Taiwan · Last active 2 min ago")
                            .control(Button::new("btn-revoke-1").ghost().small().label("Revoke")),
                        SettingsRow::new("row-session-2", "iPhone 15 Pro")
                            .description("Taipei, Taiwan · Last active 1 hour ago")
                            .control(Button::new("btn-revoke-2").ghost().small().label("Revoke")),
                        SettingsRow::new("row-session-3", "Chrome (macOS)")
                            .description("San Francisco, US · Last active 3 days ago")
                            .control(Button::new("btn-revoke-3").ghost().small().label("Revoke")),
                    ]),
            ])
            .into_any_element()
    }
}

impl Render for SettingsCenter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;

        let category_list = div()
            .v_flex()
            .gap_1()
            .children(CATEGORIES.iter().enumerate().map(|(i, label)| {
                let selected = i == self.selected;
                div()
                    .id(gpui::ElementId::Name(format!("cat-{i}").into()))
                    .h(px(36.))
                    .px_3()
                    .flex()
                    .items_center()
                    .rounded(cx.theme().radius)
                    .bg(if selected {
                        c.muted
                    } else {
                        gpui::transparent_black()
                    })
                    .text_color(if selected {
                        c.foreground
                    } else {
                        c.muted_foreground
                    })
                    .text_size(px(13.))
                    .cursor_pointer()
                    .child(*label)
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.selected = i;
                        cx.notify();
                    }))
            }));

        let page = match self.selected {
            0 => self.render_profile(cx),
            1 => self.render_notifications(cx),
            2 => self.render_appearance(cx),
            3 => self.render_security(cx),
            _ => unreachable!(),
        };

        div()
            .flex()
            .h_full()
            .child(
                Sidebar::new("settings-sidebar")
                    .width(px(220.))
                    .title("Settings")
                    .child(category_list),
            )
            .child(
                div().flex_1().v_flex().child(page).child(
                    div()
                        .h_flex()
                        .justify_end()
                        .gap_2()
                        .p_4()
                        .border_t_1()
                        .border_color(c.border)
                        .child(Button::new("btn-discard").ghost().label("Discard"))
                        .child(Button::new("btn-save").primary().label("Save Changes")),
                ),
            )
            .child(
                Dialog::new()
                    .title(self.dialog_title.clone())
                    .open(self.dialog_open)
                    .child(div().p_4().child(self.dialog_message.clone()))
                    .on_close(cx.listener(|this, _, _window, cx| {
                        this.dialog_open = false;
                        cx.notify();
                    })),
            )
    }
}
