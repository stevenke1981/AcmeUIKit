use crate::{ActiveTheme, StyledExt};
use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, Styled as _, Window, div, px,
};

/// Severity level of a log entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn color(&self, c: &crate::ThemeColors) -> gpui::Hsla {
        match self {
            LogLevel::Debug => c.muted_foreground,
            LogLevel::Info => gpui::hsla(210., 0.7, 0.5, 1.),
            LogLevel::Warn => gpui::hsla(38., 0.9, 0.5, 1.),
            LogLevel::Error => gpui::hsla(0., 0.7, 0.5, 1.),
        }
    }

    fn label(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// A single log entry with level, timestamp, and message.
pub struct LogEntry {
    pub level: LogLevel,
    pub message: SharedString,
    pub timestamp: SharedString,
}

/// A scrollable log output viewer.
///
/// # Example
///
/// ```ignore
/// LogViewer::new("log")
///     .entry(LogLevel::Info, "10:00:00", "Server started");
/// ```
#[derive(IntoElement)]
pub struct LogViewer {
    id: ElementId,
    entries: Vec<LogEntry>,
}

impl LogViewer {
    /// Create a new [`LogViewer`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            entries: Vec::new(),
        }
    }

    /// Add a log entry.
    pub fn entry(
        mut self,
        level: LogLevel,
        timestamp: impl Into<SharedString>,
        message: impl Into<SharedString>,
    ) -> Self {
        self.entries.push(LogEntry {
            level,
            message: message.into(),
            timestamp: timestamp.into(),
        });
        self
    }
}

impl RenderOnce for LogViewer {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let theme = cx.theme();

        div()
            .id(self.id)
            .v_flex()
            .bg(c.background)
            .border_1()
            .border_color(c.border)
            .rounded(px(4.))
            .overflow_hidden()
            .children(self.entries.into_iter().map(|entry| {
                let level_color = entry.level.color(&c);

                div()
                    .h_flex()
                    .gap_2()
                    .px(px(8.))
                    .py(px(3.))
                    .child(
                        // Level badge
                        div()
                            .h(px(16.))
                            .px(px(4.))
                            .flex()
                            .items_center()
                            .rounded(theme.radius_sm)
                            .text_size(px(9.))
                            .text_color(level_color)
                            .child(SharedString::from(entry.level.label())),
                    )
                    .child(
                        // Timestamp
                        div()
                            .text_size(px(10.))
                            .text_color(c.muted_foreground)
                            .child(entry.timestamp),
                    )
                    .child(
                        // Message
                        div()
                            .flex_1()
                            .text_size(theme.font_sizes.body)
                            .text_color(c.foreground)
                            .child(entry.message),
                    )
            }))
    }
}
