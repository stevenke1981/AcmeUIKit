use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn states_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
                    .title("P2 States")
                    .description("Loading overlay, disabled state, validation, AriaLabel, sr-only label, StateStyling trait")
                    // Loading overlay
                    .child(Separator::new())
                    .child(div().child("Loading Overlay:"))
                    .child(
                        Button::new("toggle-loading")
                            .extra_small()
                            .secondary()
                            .label(if self.states_loading { "Hide" } else { "Show" })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.states_loading = !this.states_loading;
                                cx.notify();
                            })),
                    )
                    .child(
                        if self.states_loading {
                            render_loading_overlay(cx, Some("Loading...".into()))
                        } else {
                            render_loading_overlay(cx, None)
                        },
                    )
                    // Disabled overlay
                    .child(Separator::new())
                    .child(div().child("Disabled Overlay:"))
                    .child(
                        Button::new("toggle-disabled")
                            .extra_small()
                            .secondary()
                            .label(if self.states_disabled { "Enabled" } else { "Disabled" })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.states_disabled = !this.states_disabled;
                                cx.notify();
                            })),
                    )
                    .child(render_disabled_overlay())
                    // Validation border
                    .child(Separator::new())
                    .child(div().child("Validation border color:"))
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(
                                div()
                                    .rounded(cx.theme().radius)
                                    .px_3()
                                    .py_1()
                                    .border_1()
                                    .border_color(c.success)
                                    .child("Valid"),
                            )
                            .child(
                                div()
                                    .rounded(cx.theme().radius)
                                    .px_3()
                                    .py_1()
                                    .border_1()
                                    .border_color(c.danger)
                                    .child("Invalid"),
                            ),
                    )
                    // State styling (disabled vs normal buttons)
                    .child(Separator::new())
                    .child(div().child("State styling:"))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                Button::new("styling-normal")
                                    .extra_small()
                                    .label("Normal"),
                            )
                            .child(
                                Button::new("styling-disabled")
                                    .extra_small()
                                    .label("Disabled")
                                    .disabled(true),
                            )
                            .child(
                                Button::new("styling-danger")
                                    .extra_small()
                                    .danger()
                                    .label("Danger"),
                            )
                            .child(
                                Button::new("styling-primary")
                                    .extra_small()
                                    .primary()
                                    .label("Primary"),
                            ),
                    )
                    // sr-only label
                    .child(Separator::new())
                    .child(div().child("SR-only label (inspect DOM):"))
                    .child(sr_only_label("Hidden screen reader text"))
    }

    pub fn access_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
                    .title("P2 Accessibility")
                    .description("AriaRole, AriaAttrs, reduced-motion query, focus ring style")
                    // AriaRole
                    .child(Separator::new())
                    .child(div().child("AriaRole variants:"))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .text_size(px(11.))
                                    .child("Button"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .text_size(px(11.))
                                    .child("Dialog"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .text_size(px(11.))
                                    .child("Alert"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .text_size(px(11.))
                                    .child("TabPanel"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .text_size(px(11.))
                                    .child("Tooltip"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .text_size(px(11.))
                                    .child("Menu"),
                            ),
                    )
                    // AriaAttrs builder
                    .child(Separator::new())
                    .child(div().child("AriaAttrs builder (API reference):"))
                    .child(
                        div()
                            .v_flex()
                            .gap_1()
                            .text_size(px(11.))
                            .text_color(c.muted_foreground)
                            .child("40+ ARIA roles: Button, Dialog, Alert, TabPanel, Tooltip, Menu, ??")
                            .child("17 ARIA attributes: label, describedby, expanded, pressed, ??"),
                    )
                    // Reduced motion
                    .child(Separator::new())
                    .child(div().child("Reduced motion preference:"))
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(c.success)
                            .child("No reduced motion preference"),
                    )
                    // Focus ring style
                    .child(Separator::new())
                    .child(div().child("Focus ring style:"))
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(c.muted_foreground)
                            .child("Use focus_ring_style(cx) to get (width, color) at runtime"),
                    )
    }

    pub fn focus_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
                    .title("P2 Focus")
                    .description("FocusTrap, RovingTabIndex, keyboard handlers, DefaultCancelButtons")
                    // FocusTrap
                    .child(Separator::new())
                    .child(div().child("FocusTrap:"))
                    .child(
                        Button::new("toggle-focustrap")
                            .extra_small()
                            .secondary()
                            .label(if self.focus_trap_active {
                                "Trap Active"
                            } else {
                                "Trap Inactive"
                            })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.focus_trap_active = !this.focus_trap_active;
                                cx.notify();
                            })),
                    )
                    .child(
                        FocusTrap::new("demo-focus-trap")
                            .active(self.focus_trap_active)
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .p_2()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .child(Button::new("ft-btn-1").extra_small().label("Btn 1"))
                                    .child(Button::new("ft-btn-2").extra_small().label("Btn 2"))
                                    .child(Button::new("ft-btn-3").extra_small().label("Btn 3")),
                            ),
                    )
                    // RovingTabIndex
                    .child(Separator::new())
                    .child(div().child("RovingTabIndex demo:"))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(c.surface)
                                    .border_1()
                                    .border_color(c.border)
                                    .child("Item A"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(c.surface)
                                    .border_1()
                                    .border_color(c.ring)
                                    .child("Item B (focused)"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(cx.theme().radius)
                                    .bg(c.surface)
                                    .border_1()
                                    .border_color(c.border)
                                    .child("Item C"),
                            ),
                    )
                    // Keyboard handlers
                    .child(Separator::new())
                    .child(div().child("Keyboard handlers:"))
                    .child(
                        div()
                            .v_flex()
                            .gap_2()
                            .child(
                                Button::new("demo-escape")
                                    .extra_small()
                                    .secondary()
                                    .label("Press Escape ??callback")
                                    .on_click(cx.listener(|_, _, _, _| {})),
                            )
                            .child(
                                div()
                                    .text_size(px(10.))
                                    .text_color(c.muted_foreground)
                                    .child("escape_close_handler() / arrow_key_nav_handler() for on_key_down"),
                            ),
                    )
                    // DefaultCancelButtons
                    .child(Separator::new())
                    .child(div().child("DefaultCancelButtons:"))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(Button::new("dflt-ok").extra_small().primary().label("OK"))
                            .child(Button::new("dflt-cancel").extra_small().secondary().label("Cancel")),
                    )
    }

    pub fn overlay_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
                    .title("P2 Overlay")
                    .description("ModalBackdrop, OverlayDepth, AutoPositioner, ClickOutsideListener, FocusRestore")
                    // ModalBackdrop
                    .child(Separator::new())
                    .child(div().child("ModalBackdrop:"))
                    .child(
                        Button::new("toggle-backdrop")
                            .extra_small()
                            .secondary()
                            .label(if self.modal_backdrop_open {
                                "Hide Backdrop"
                            } else {
                                "Show Backdrop"
                            })
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.modal_backdrop_open = !this.modal_backdrop_open;
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .h(px(80.))
                            .relative()
                            .bg(c.surface)
                            .rounded(cx.theme().radius)
                            .child(
                                div().p_2().text_size(px(11.)).text_color(c.muted_foreground)
                                    .child(if self.modal_backdrop_open {
                                        "Backdrop visible"
                                    } else {
                                        "Click 'Show Backdrop' above"
                                    }),
                            )
                            .when(self.modal_backdrop_open, |this| {
                                this.child(ModalBackdrop::new("demo-backdrop").depth(OverlayDepth::Modal))
                            }),
                    )
                    // OverlayDepth
                    .child(Separator::new())
                    .child(div().child("OverlayDepth z-index layers:"))
                    .child(
                        div()
                            .v_flex()
                            .gap_1()
                            .text_size(px(11.))
                            .text_color(c.muted_foreground)
                            .child("Popover=100  Drawer=200  Dialog=300  Modal=400  Toast=500  DragDrop=600"),
                    )
                    // AutoPositioner
                    .child(Separator::new())
                    .child(div().child("AutoPositioner placements:"))
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .child("Bottom"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .child("Top"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .child("Left"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .child("Right"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .child("BottomStart"),
                            )
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .child("TopEnd"),
                            ),
                    )
                    // ClickOutsideListener
                    .child(Separator::new())
                    .child(div().child("ClickOutsideListener:"))
                    .child(
                        ClickOutsideListener::new("demo-outside")
                            .child(
                                div()
                                    .bg(c.surface)
                                    .rounded(cx.theme().radius)
                                    .p_3()
                                    .text_size(px(11.))
                                    .text_color(c.muted_foreground)
                                    .child("Click outside this box"),
                            ),
                    )
                    // FocusRestore
                    .child(Separator::new())
                    .child(div().child("FocusRestore:"))
                    .child(
                        div()
                            .v_flex()
                            .gap_1()
                            .text_size(px(11.))
                            .text_color(c.muted_foreground)
                            .child("FocusRestore saves and restores focus for overlay show/hide lifecycle")
                            .child("Used internally by Dialog, Drawer, Popover"),
                    )
    }
}
