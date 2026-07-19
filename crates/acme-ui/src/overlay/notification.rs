use std::time::Duration;

use gpui::{
    App, AppContext as _, AsyncApp, Context, ElementId, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled as _,
    WeakEntity, Window, div, prelude::FluentBuilder as _, px,
};

use crate::{ActiveTheme, StyledExt, ThemeColors};

/// Severity level for a notification toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationLevel {
    /// Informational message.
    Info,
    /// Successful operation.
    Success,
    /// Warning that needs attention.
    Warning,
    /// Error / failure.
    Error,
}

impl NotificationLevel {
    fn border_color(self, c: ThemeColors) -> gpui::Hsla {
        match self {
            Self::Info => c.primary,
            Self::Success => c.success,
            Self::Warning => c.warning,
            Self::Error => c.danger,
        }
    }
}

/// A single notification entry.
#[derive(Clone)]
pub struct NotificationItem {
    /// Unique identifier.
    pub id: usize,
    /// Severity level.
    pub level: NotificationLevel,
    /// Heading text.
    pub title: SharedString,
    /// Optional body text.
    pub message: Option<SharedString>,
}

/// Entity-based container that manages a stack of notification toasts.
///
/// Notifications are rendered as an absolutely-positioned stack in the
/// top-right area. Each toast auto-dismisses after 4 seconds. The caller
/// controls the lifecycle via `push` and `dismiss`.
///
/// # Example
///
/// ```ignore
/// let toasts = Notifications::new(cx);
///
/// // From an event handler:
/// toasts.update(cx, |this, cx| {
///     this.push(NotificationLevel::Info, "Saved", None, cx);
/// });
///
/// // Embed in a render tree:
/// div().child(toasts.clone())
/// ```
pub struct Notifications {
    items: Vec<NotificationItem>,
    next_id: usize,
}

impl Notifications {
    /// Creates a new notification container entity.
    pub fn new(cx: &mut App) -> gpui::Entity<Self> {
        cx.new(|_| Self {
            items: Vec::new(),
            next_id: 0,
        })
    }

    /// Pushes a new toast and schedules auto-dismiss after 4 seconds.
    pub fn push(
        &mut self,
        level: NotificationLevel,
        title: impl Into<SharedString>,
        message: Option<SharedString>,
        cx: &mut Context<Self>,
    ) {
        let id = self.next_id;
        self.next_id += 1;
        self.items.push(NotificationItem {
            id,
            level,
            title: title.into(),
            message,
        });
        cx.notify();

        // Auto-dismiss after 4 seconds.
        cx.spawn(move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            // Clone AsyncApp so the async block can own it (lifetime safety).
            let mut cx = cx.clone();
            async move {
                cx.background_executor().timer(Duration::from_secs(4)).await;
                if let Some(this) = this.upgrade() {
                    this.update(&mut cx, |this, cx| {
                        this.items.retain(|item| item.id != id);
                        cx.notify();
                    });
                }
            }
        })
        .detach();
    }

    /// Removes a notification by its id.
    pub fn dismiss(&mut self, id: usize, cx: &mut Context<Self>) {
        self.items.retain(|item| item.id != id);
        cx.notify();
    }
}

impl Render for Notifications {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        let radius = cx.theme().radius;

        div()
            .absolute()
            .top(px(12.))
            .right(px(12.))
            .v_flex()
            .gap_2()
            .children(self.items.iter().map(|item| {
                let item_id = item.id;
                let level = item.level;
                let title = item.title.clone();
                let message = item.message.clone();

                let close_id = ElementId::Name(format!("notification-{item_id}-close").into());

                div()
                    .w(px(320.))
                    .flex()
                    .rounded(radius)
                    .border_1()
                    .border_color(c.border)
                    .bg(c.surface)
                    .overflow_hidden()
                    .child(
                        // Colored left accent bar
                        div().w(px(4.)).flex_none().bg(level.border_color(c)),
                    )
                    .child(
                        // Content area
                        div()
                            .flex_1()
                            .v_flex()
                            .gap_1()
                            .p_2()
                            .child(
                                div()
                                    .text_color(c.foreground)
                                    .text_size(cx.theme().font_sizes.body)
                                    .child(title),
                            )
                            .when_some(message, |this, msg| {
                                this.child(
                                    div()
                                        .text_color(c.muted_foreground)
                                        .text_size(cx.theme().font_sizes.caption)
                                        .child(msg),
                                )
                            }),
                    )
                    .child(
                        // Close button
                        div()
                            .id(close_id)
                            .p_2()
                            .flex_none()
                            .cursor_pointer()
                            .text_color(c.muted_foreground)
                            .hover(|style| style.text_color(c.foreground))
                            .child("✕")
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.dismiss(item_id, cx);
                            })),
                    )
                    .into_any_element()
            }))
    }
}
