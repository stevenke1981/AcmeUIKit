use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _, px,
};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

use crate::{ActiveTheme, StyledExt};

// ── Validation ────────────────────────────────────────────────────────────────

/// Validation rule that verifies a field value and returns an error message on
/// failure.
pub type ValidationRule = Rc<dyn Fn(&str) -> Option<SharedString>>;

/// Pre‑built validation rules.
pub mod validators {
    use super::*;

    /// The value must not be empty.
    pub fn required() -> ValidationRule {
        Rc::new(|value: &str| {
            if value.trim().is_empty() {
                Some("This field is required".into())
            } else {
                None
            }
        })
    }

    /// The value must be at least `min` characters long.
    pub fn min_length(min: usize) -> ValidationRule {
        Rc::new(move |value: &str| {
            if value.len() < min {
                Some(format!("Must be at least {min} characters").into())
            } else {
                None
            }
        })
    }

    /// The value must be at most `max` characters long.
    pub fn max_length(max: usize) -> ValidationRule {
        Rc::new(move |value: &str| {
            if value.len() > max {
                Some(format!("Must be at most {max} characters").into())
            } else {
                None
            }
        })
    }

    /// The value must match the given regex pattern.
    pub fn pattern(regex: &str, message: impl Into<SharedString>) -> ValidationRule {
        let re = regex::Regex::new(regex);
        let msg: SharedString = message.into();
        Rc::new(move |value: &str| {
            if let Ok(ref re) = re {
                if !re.is_match(value) {
                    return Some(msg.clone());
                }
            }
            None
        })
    }

    /// The value must be a valid email (simple check).
    pub fn email() -> ValidationRule {
        pattern(
            r"^[^\s@]+@[^\s@]+\.[^\s@]+$",
            "Must be a valid email address",
        )
    }
}

// ── Field ─────────────────────────────────────────────────────────────────────

/// A form field with label, validation state, and error display.
///
/// The caller provides the current value string and a list of validation rules.
/// Validation runs eagerly on every render via `validate`.
#[derive(IntoElement)]
pub struct Field {
    id: ElementId,
    label: SharedString,
    value: SharedString,
    helper: Option<SharedString>,
    rules: Vec<ValidationRule>,
    on_submit: Option<ClickHandler>,
}

impl Field {
    /// Creates a new field with the given [`ElementId`] and label.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value: SharedString::default(),
            helper: None,
            rules: Vec::new(),
            on_submit: None,
        }
    }

    /// Sets the current field value (used for validation).
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets an optional helper text shown below the field.
    pub fn helper(mut self, helper: impl Into<SharedString>) -> Self {
        self.helper = Some(helper.into());
        self
    }

    /// Registers a validation rule.
    pub fn rule(mut self, rule: ValidationRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Registers multiple validation rules at once.
    pub fn rules(mut self, rules: Vec<ValidationRule>) -> Self {
        self.rules.extend(rules);
        self
    }

    /// Registers an optional submit / action click handler.
    pub fn on_submit(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_submit = Some(Rc::new(handler));
        self
    }

    /// Validates the current value against all rules and returns the first
    /// error message, or `None` if all rules pass.
    fn validate(&self) -> Option<SharedString> {
        self.rules.iter().find_map(|rule| rule(self.value.as_ref()))
    }
}

impl RenderOnce for Field {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let error = self.validate();
        let has_error = error.is_some();

        let mut input = div()
            .id(self.id)
            .w_full()
            .h(px(36.))
            .px_3()
            .h_flex()
            .rounded(theme.radius)
            .border_1()
            .border_color(if has_error { c.danger } else { c.border })
            .bg(c.background)
            .text_color(if self.value.is_empty() {
                c.muted_foreground
            } else {
                c.foreground
            })
            .text_size(theme.font_sizes.body);

        if let Some(ref handler) = self.on_submit {
            let handler = handler.clone();
            input = input
                .cursor_pointer()
                .hover(|style| style.border_color(c.ring))
                .on_click(
                    move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                        handler(event, window, cx);
                    },
                );
        }

        let display_text = if self.value.is_empty() {
            SharedString::from("\u{00a0}") // non-breaking space for height
        } else {
            self.value.clone()
        };

        input = input.child(div().flex_1().child(display_text));

        div()
            .v_flex()
            .gap_1()
            .child(
                div()
                    .text_color(c.foreground)
                    .text_size(theme.font_sizes.caption)
                    .child(self.label),
            )
            .child(input)
            .when_some(error, |this, msg| {
                this.child(
                    div()
                        .text_color(c.danger)
                        .text_size(theme.font_sizes.caption)
                        .child(msg),
                )
            })
            .when_some(self.helper, |this, helper| {
                this.when(!has_error, |this| {
                    this.child(
                        div()
                            .text_color(c.muted_foreground)
                            .text_size(theme.font_sizes.caption)
                            .child(helper),
                    )
                })
            })
    }
}

// ── Form ──────────────────────────────────────────────────────────────────────

/// A form container that groups fields and provides a submit button.
///
/// # Example
///
/// ```ignore
/// Form::new("contact-form")
///     .fields(vec![
///         Field::new("name", "Name")
///             .value("John")
///             .rule(validators::required()),
///         Field::new("email", "Email")
///             .value("john@example.com")
///             .rule(validators::email()),
///     ])
///     .on_submit(|_event, _window, _cx| {
///         // handle form submission
///     })
/// ```
#[derive(IntoElement)]
pub struct Form {
    id: ElementId,
    fields: Vec<Field>,
    submit_label: SharedString,
    on_submit: Option<ClickHandler>,
}

impl Form {
    /// Creates a new form with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            fields: Vec::new(),
            submit_label: SharedString::from("Submit"),
            on_submit: None,
        }
    }

    /// Sets the fields inside this form.
    pub fn fields(mut self, fields: Vec<Field>) -> Self {
        self.fields = fields;
        self
    }

    /// Sets the label of the submit button.
    pub fn submit_label(mut self, label: impl Into<SharedString>) -> Self {
        self.submit_label = label.into();
        self
    }

    /// Registers the form submit handler.
    pub fn on_submit(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_submit = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Form {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let _c = theme.colors;

        let any_error = self
            .fields
            .iter()
            .any(|f| f.rules.iter().any(|rule| rule(f.value.as_ref()).is_some()));

        let btn_id = ElementId::Name(SharedString::from(format!("{:?}-submit", self.id)));
        let mut btn = crate::Button::new(btn_id)
            .label(self.submit_label)
            .primary();

        if any_error {
            btn = btn.disabled(true);
        }

        if let Some(ref handler) = self.on_submit {
            if !any_error {
                let handler = handler.clone();
                btn = btn.on_click(
                    move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                        handler(event, window, cx);
                    },
                );
            }
        }

        div()
            .id(self.id)
            .v_flex()
            .gap(theme.spacing.group)
            .child(
                div()
                    .v_flex()
                    .gap(theme.spacing.widget)
                    .children(self.fields),
            )
            .child(div().h_flex().justify_end().child(btn))
    }
}
