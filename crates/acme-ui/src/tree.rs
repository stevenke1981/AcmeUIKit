use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px,
};

use crate::{ActiveTheme, StyledExt};

type TreeClickHandler = Rc<dyn Fn(usize, &ClickEvent, &mut Window, &mut App)>;

/// A single node in a [`Tree`].
///
/// Each node has a label, an optional unique index, a set of child nodes, and
/// an expand/collapse state. The caller is responsible for managing the
/// expanded set (via `expanded`).
#[derive(Clone)]
pub struct TreeNode {
    /// Display label.
    pub label: SharedString,
    /// Unique index used in click/hover callbacks.
    pub index: usize,
    /// Whether this node is disabled.
    pub disabled: bool,
    /// Child nodes (rendered recursively when this node is expanded).
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    /// Creates a leaf node with the given label and index.
    pub fn new(label: impl Into<SharedString>, index: usize) -> Self {
        Self {
            label: label.into(),
            index,
            disabled: false,
            children: Vec::new(),
        }
    }

    /// Sets the disabled state.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Attaches child nodes to make this an expandable branch.
    pub fn children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
}

/// Tree / hierarchical list component.
///
/// Renders a recursive, collapsible tree from [`TreeNode`] values. The caller
/// controls which nodes are expanded by providing a set of expanded indices.
///
/// # Example
///
/// ```ignore
/// Tree::new("file-tree")
///     .nodes(vec![
///         TreeNode::new("src", 0).children(vec![
///             TreeNode::new("main.rs", 1),
///             TreeNode::new("lib.rs", 2),
///         ]),
///         TreeNode::new("README.md", 3),
///     ])
///     .expanded(&[0])
///     .on_click(|index, _event, _window, _cx| {
///         // handle node click
///     })
/// ```
#[derive(IntoElement)]
pub struct Tree {
    id: ElementId,
    nodes: Vec<TreeNode>,
    expanded: Rc<Vec<usize>>,
    on_click: Option<TreeClickHandler>,
}

impl Tree {
    /// Creates a new tree with the given [`ElementId`].
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            nodes: Vec::new(),
            expanded: Rc::new(Vec::new()),
            on_click: None,
        }
    }

    /// Sets the root-level tree nodes.
    pub fn nodes(mut self, nodes: Vec<TreeNode>) -> Self {
        self.nodes = nodes;
        self
    }

    /// Provides a set of expanded node indices.
    pub fn expanded(mut self, expanded: &[usize]) -> Self {
        self.expanded = Rc::new(expanded.to_vec());
        self
    }

    /// Registers a click handler fired when a non-disabled node is clicked.
    pub fn on_click(
        mut self,
        handler: impl Fn(usize, &ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Tree {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let c = theme.colors;

        div()
            .id(self.id)
            .v_flex()
            .min_w(px(180.))
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .children(self.nodes.into_iter().map(|node| {
                        render_node(
                            node,
                            self.expanded.clone(),
                            self.on_click.clone(),
                            theme.font_sizes.body,
                            c.foreground,
                            c.muted,
                            c.muted_foreground,
                            c.primary,
                            theme.radius,
                            0,
                        )
                    })),
            )
    }
}

#[allow(clippy::too_many_arguments, clippy::only_used_in_recursion)]
fn render_node(
    node: TreeNode,
    expanded: Rc<Vec<usize>>,
    on_click: Option<TreeClickHandler>,
    font_size: gpui::Pixels,
    foreground: gpui::Hsla,
    muted: gpui::Hsla,
    muted_foreground: gpui::Hsla,
    _primary: gpui::Hsla,
    radius: gpui::Pixels,
    depth: usize,
) -> gpui::AnyElement {
    let is_expanded = expanded.contains(&node.index);
    let has_children = !node.children.is_empty();
    let is_disabled = node.disabled;

    let indent = px(8.) + px(16. * depth as f32);

    let expand_icon = if has_children {
        if is_expanded { "▾" } else { "▸" }
    } else {
        " "
    };

    let mut row = div()
        .id(ElementId::Name(format!("tree-node-{}", node.index).into()))
        .h_flex()
        .h(px(28.))
        .pl(indent)
        .pr_2()
        .gap_1()
        .rounded(radius)
        .text_size(font_size);

    let text_color = if is_disabled {
        muted_foreground
    } else {
        foreground
    };

    row = row
        .child(
            div()
                .w(px(14.))
                .text_color(muted_foreground)
                .text_size(px(10.))
                .child(expand_icon),
        )
        .child(div().text_color(text_color).child(node.label));

    if !is_disabled {
        row = row.cursor_pointer().hover(|style| style.bg(muted));

        if let Some(ref handler) = on_click {
            let handler = handler.clone();
            let idx = node.index;
            row = row.on_click(
                move |event: &ClickEvent, window: &mut Window, cx: &mut App| {
                    handler(idx, event, window, cx);
                },
            );
        }
    }

    let mut container = div().v_flex().child(row);

    if is_expanded && has_children {
        container = container.child(div().v_flex().children(node.children.into_iter().map(
            |child| {
                render_node(
                    child,
                    expanded.clone(),
                    on_click.clone(),
                    font_size,
                    foreground,
                    muted,
                    muted_foreground,
                    _primary,
                    radius,
                    depth + 1,
                )
            },
        )));
    }

    container.into_any_element()
}
