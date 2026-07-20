use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div,
    prelude::FluentBuilder as _,
};

use crate::{ActiveTheme, Size};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Visual variant for a button.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Secondary,
    Danger,
    Ghost,
}

/// Stateless GPUI button with semantic variants and sizes.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    variant: ButtonVariant,
    size: Size,
    disabled: bool,
    selected: bool,
    loading: bool,
    on_click: Option<ClickHandler>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: SharedString::default(),
            variant: ButtonVariant::Default,
            size: Size::Medium,
            disabled: false,
            selected: false,
            loading: false,
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn primary(self) -> Self {
        self.with_variant(ButtonVariant::Primary)
    }

    pub fn secondary(self) -> Self {
        self.with_variant(ButtonVariant::Secondary)
    }

    pub fn danger(self) -> Self {
        self.with_variant(ButtonVariant::Danger)
    }

    pub fn ghost(self) -> Self {
        self.with_variant(ButtonVariant::Ghost)
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn extra_small(self) -> Self {
        self.with_size(Size::ExtraSmall)
    }

    pub fn small(self) -> Self {
        self.with_size(Size::Small)
    }

    pub fn medium(self) -> Self {
        self.with_size(Size::Medium)
    }

    pub fn large(self) -> Self {
        self.with_size(Size::Large)
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Shows a compact loading indicator while preserving the button size.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        let (background, foreground, hover, border) = match self.variant {
            ButtonVariant::Default => (c.surface, c.foreground, c.muted, c.border),
            ButtonVariant::Primary => (c.primary, c.primary_foreground, c.primary_hover, c.primary),
            ButtonVariant::Secondary => (c.secondary, c.foreground, c.secondary_hover, c.border),
            ButtonVariant::Danger => (c.danger, c.primary_foreground, c.danger_hover, c.danger),
            ButtonVariant::Ghost => (c.background, c.foreground, c.muted, c.background),
        };

        let selected_border = if self.selected { c.ring } else { border };
        let disabled = self.disabled;
        let loading = self.loading;
        let handler = self.on_click;
        let control_height = match self.size {
            Size::ExtraSmall => theme.controls.xs,
            Size::Small => theme.controls.small,
            Size::Medium => theme.controls.medium,
            Size::Large => theme.controls.large,
        };
        let text_size = match self.size {
            Size::ExtraSmall => theme.typography.caption.size,
            Size::Small => theme.typography.body_compact.size,
            Size::Medium => theme.typography.body.size,
            Size::Large => theme.typography.body.size,
        };

        let button = div()
            .id(self.id)
            .h(control_height)
            .px(self.size.horizontal_padding())
            .flex()
            .items_center()
            .justify_center()
            .rounded(theme.radius)
            .border_1()
            .border_color(selected_border)
            .bg(background)
            .text_color(if disabled || loading {
                c.muted_foreground
            } else {
                foreground
            })
            .text_size(text_size)
            .child(if loading {
                SharedString::from("…")
            } else {
                self.label
            })
            .when(!disabled && !loading, |this| {
                this.cursor_pointer().hover(move |style| style.bg(hover))
            });

        match handler {
            Some(handler) if !disabled && !loading => button
                .on_click(move |event, window, cx| handler(event, window, cx))
                .into_any_element(),
            _ => button.into_any_element(),
        }
    }
}
