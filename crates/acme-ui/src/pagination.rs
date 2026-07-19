use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

type PageChangeHandler = Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>;

/// Internal representation of a single entry in the pagination strip.
#[derive(Debug, Clone, PartialEq, Eq)]
enum PageItem {
    Page(usize),
    Ellipsis,
}

/// Builds the visible sequence of page items, inserting ellipsis for gaps.
///
/// Always includes the first page, last page, current page, and one page on
/// each side of the current page. When the gap between two consecutive items
/// is larger than one, a [`PageItem::Ellipsis`] placeholder is inserted.
fn collect_pages(current: usize, total: usize) -> Vec<PageItem> {
    if total == 0 {
        return Vec::new();
    }

    let mut set: Vec<usize> = Vec::new();
    set.push(1);
    if current > 1 {
        set.push(current - 1);
    }
    set.push(current);
    if current < total {
        set.push(current + 1);
    }
    if total > 1 {
        set.push(total);
    }

    set.sort_unstable();
    set.dedup();
    set.retain(|&p| p >= 1 && p <= total);

    let mut items: Vec<PageItem> = Vec::new();
    let mut prev: Option<usize> = None;

    for &p in &set {
        if let Some(prev_val) = prev {
            if p - prev_val > 1 {
                items.push(PageItem::Ellipsis);
            }
        }
        items.push(PageItem::Page(p));
        prev = Some(p);
    }

    items
}

/// Stateless pagination navigator.
///
/// Renders a horizontal row of page-number buttons with previous / next
/// arrows and smart ellipsis for large page counts. The caller manages
/// the current page and total page count in their own state.
///
/// # Example
///
/// ```ignore
/// Pagination::new("table-pages")
///     .current(5)
///     .total(20)
///     .on_page_change(|page, _event, _window, _cx| {
///         // update application state
///     });
/// ```
#[derive(IntoElement)]
pub struct Pagination {
    id: ElementId,
    current: usize,
    total: usize,
    disabled: bool,
    on_page_change: Option<PageChangeHandler>,
}

impl Pagination {
    /// Create a new [`Pagination`] with the given stable [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            current: 1,
            total: 1,
            disabled: false,
            on_page_change: None,
        }
    }

    /// Set the 1-indexed current page (default: `1`).
    pub fn current(mut self, page: usize) -> Self {
        self.current = page;
        self
    }

    /// Set the total number of pages (default: `1`).
    pub fn total(mut self, count: usize) -> Self {
        self.total = count.max(1);
        self
    }

    /// Register a callback that fires when a page button is clicked.
    ///
    /// The callback receives the target page number, the click event, the
    /// window, and the application context. It is not invoked when the
    /// component is disabled or when the already-active page is clicked.
    pub fn on_page_change(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_page_change = Some(Rc::new(handler));
        self
    }

    /// Disable all interaction across every button.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = cx.theme().colors;
        let radius = cx.theme().radius;
        let disabled = self.disabled;
        let handler = self.on_page_change;
        let total = self.total.max(1);
        let current = self.current.clamp(1, total);

        // Derive a string prefix so child element IDs are stable and unique.
        let id_prefix = match &self.id {
            ElementId::Name(s) => s.to_string(),
            ElementId::Integer(n) => format!("paginator-{n}"),
            _ => "paginator".to_string(),
        };

        let pages = collect_pages(current, total);

        // ── Previous arrow ────────────────────────────────────────────────
        let prev_disabled = disabled || current <= 1;
        let prev_id = ElementId::Name(format!("{id_prefix}-prev").into());
        let prev_page = current.saturating_sub(1);

        let mut prev = div()
            .id(prev_id)
            .size(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(radius)
            .bg(c.surface)
            .text_color(if prev_disabled {
                c.muted_foreground
            } else {
                c.foreground
            })
            .text_size(cx.theme().font_sizes.body)
            .child("⟨");

        if !prev_disabled {
            prev = prev.cursor_pointer().hover(|style| style.bg(c.muted));
        }

        let prev = match &handler {
            Some(h) if !prev_disabled => {
                let h = h.clone();
                prev.on_click(move |event, window, cx| h(prev_page, event, window, cx))
                    .into_any_element()
            }
            _ => prev.into_any_element(),
        };

        // ── Next arrow ────────────────────────────────────────────────────
        let next_disabled = disabled || current >= total;
        let next_id = ElementId::Name(format!("{id_prefix}-next").into());
        let next_page = current + 1;

        let mut next = div()
            .id(next_id)
            .size(px(32.0))
            .flex()
            .items_center()
            .justify_center()
            .rounded(radius)
            .bg(c.surface)
            .text_color(if next_disabled {
                c.muted_foreground
            } else {
                c.foreground
            })
            .text_size(cx.theme().font_sizes.body)
            .child("⟩");

        if !next_disabled {
            next = next.cursor_pointer().hover(|style| style.bg(c.muted));
        }

        let next = match &handler {
            Some(h) if !next_disabled => {
                let h = h.clone();
                next.on_click(move |event, window, cx| h(next_page, event, window, cx))
                    .into_any_element()
            }
            _ => next.into_any_element(),
        };

        // ── Page number buttons with ellipsis ──────────────────────────────
        let page_buttons = pages.into_iter().map(|item| match item {
            PageItem::Page(n) => {
                let is_current = n == current;
                let page_disabled = disabled || is_current;

                let bg = if is_current { c.primary } else { c.surface };
                let fg = if is_current {
                    c.primary_foreground
                } else {
                    c.foreground
                };
                let hover_bg = if is_current { c.primary_hover } else { c.muted };

                let page_id = ElementId::Name(format!("{id_prefix}-page-{n}").into());

                let mut btn = div()
                    .id(page_id)
                    .min_w(px(32.0))
                    .h(px(32.0))
                    .px_2()
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(radius)
                    .bg(bg)
                    .text_color(if page_disabled {
                        c.muted_foreground
                    } else {
                        fg
                    })
                    .text_size(cx.theme().font_sizes.body)
                    .child(n.to_string());

                if !page_disabled {
                    btn = btn.cursor_pointer().hover(move |style| style.bg(hover_bg));
                }

                if is_current {
                    btn = btn.border_1().border_color(c.primary);
                }

                match &handler {
                    Some(h) if !page_disabled => {
                        let h = h.clone();
                        btn.on_click(move |event, window, cx| h(n, event, window, cx))
                            .into_any_element()
                    }
                    _ => btn.into_any_element(),
                }
            }
            PageItem::Ellipsis => div()
                .h(px(32.0))
                .px_2()
                .flex()
                .items_center()
                .justify_center()
                .text_color(c.muted_foreground)
                .text_size(cx.theme().font_sizes.body)
                .child("...")
                .into_any_element(),
        });

        // ── Assemble the pagination row ───────────────────────────────────
        div()
            .id(self.id)
            .h_flex()
            .gap(px(4.0))
            .child(prev)
            .children(page_buttons)
            .child(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_pages_single_page() {
        let items = collect_pages(1, 1);
        assert_eq!(items, vec![PageItem::Page(1)]);
    }

    #[test]
    fn collect_pages_first_page() {
        let items = collect_pages(1, 5);
        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Page(2),
                PageItem::Ellipsis,
                PageItem::Page(5),
            ]
        );
    }

    #[test]
    fn collect_pages_middle_page() {
        let items = collect_pages(5, 10);
        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Ellipsis,
                PageItem::Page(4),
                PageItem::Page(5),
                PageItem::Page(6),
                PageItem::Ellipsis,
                PageItem::Page(10),
            ]
        );
    }

    #[test]
    fn collect_pages_last_page() {
        let items = collect_pages(10, 10);
        assert_eq!(
            items,
            vec![
                PageItem::Page(1),
                PageItem::Ellipsis,
                PageItem::Page(9),
                PageItem::Page(10),
            ]
        );
    }

    #[test]
    fn collect_pages_small_total() {
        let items = collect_pages(2, 3);
        assert_eq!(
            items,
            vec![PageItem::Page(1), PageItem::Page(2), PageItem::Page(3),]
        );
    }

    #[test]
    fn collect_pages_zero_total() {
        let items = collect_pages(1, 0);
        assert!(items.is_empty());
    }

    #[test]
    fn pagination_builder_defaults() {
        let p = Pagination::new("test");
        assert_eq!(p.current, 1);
        assert_eq!(p.total, 1);
        assert!(!p.disabled);
        assert!(p.on_page_change.is_none());
    }

    #[test]
    fn pagination_builder_chaining() {
        let p = Pagination::new("test").current(3).total(7).disabled(true);
        assert_eq!(p.current, 3);
        assert_eq!(p.total, 7);
        assert!(p.disabled);
        assert!(p.on_page_change.is_none());
    }

    #[test]
    fn pagination_total_clamps_to_one() {
        let p = Pagination::new("test").total(0);
        assert_eq!(p.total, 1);
    }

    #[test]
    fn pagination_on_page_change_stores_handler() {
        let p = Pagination::new("test")
            .on_page_change(|_: usize, _: &ClickEvent, _: &mut Window, _: &mut App| {});
        assert!(p.on_page_change.is_some());
    }
}
