use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn v3_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let c = cx.theme().colors;
        Card::new()
                    .title("V3 Components")
                    .description("Tree, Form+Validation, Sortable Table")
                    // Tree
                    .child(Separator::new())
                    .child(div().child("Tree (V3) ??click a node:"))
                    .child(
                        div()
                            .child(match self.tree_selected {
                                Some(i) => format!("Selected node: {i}"),
                                None => "No node selected".to_string(),
                            })
                            .text_size(px(11.))
                            .text_color(c.muted_foreground),
                    )
                    .child(Tree::new("demo-tree")
                        .nodes(vec![
                            TreeNode::new("src", 0).children(vec![
                                TreeNode::new("main.rs", 1),
                                TreeNode::new("lib.rs", 2),
                                TreeNode::new("components", 3).children(vec![
                                    TreeNode::new("button.rs", 4),
                                    TreeNode::new("card.rs", 5),
                                ]),
                            ]),
                            TreeNode::new("Cargo.toml", 6),
                            TreeNode::new("README.md", 7),
                        ])
                        .expanded(&[0, 3])
                        .on_click({
                            let handle = cx.entity().downgrade();
                            move |index, _event, _window, cx| {
                                if let Some(handle) = handle.upgrade() {
                                    handle.update(cx, |this, cx| {
                                        this.tree_selected = Some(index);
                                        cx.notify();
                                    });
                                }
                            }
                        })
                    )
                    // Form
                    .child(Separator::new())
                    .child(div().child("Form + Validation (V3):"))
                    .child(
                        Form::new("demo-form")
                            .submit_label("Register")
                            .fields(vec![
                                Field::new("form-name", "Name")
                                    .value("John")
                                    .rule(validators::required())
                                    .rule(validators::min_length(2)),
                                Field::new("form-email", "Email")
                                    .value("john@example")
                                    .rule(validators::email()),
                                Field::new("form-bio", "Bio")
                                    .value("")
                                    .rule(validators::required())
                                    .helper("Tell us about yourself"),
                            ])
                            .on_submit({
                                let handle = cx.entity().downgrade();
                                move |_event, _window, cx| {
                                    if let Some(handle) = handle.upgrade() {
                                        handle.update(cx, |_this, cx| {
                                            cx.notify();
                                        });
                                    }
                                }
                            }),
                    )
                    // Table
                    .child(Separator::new())
                    .child(div().child("Sortable Table (V3):"))
                    .child(
                        Table::new("demo-table")
                            .columns(vec![
                                TableColumn::new("Name").sortable("name").width(px(120.)),
                                TableColumn::new("Role").sortable("role").width(px(100.)),
                                TableColumn::new("Email"),
                            ])
                            .rows(vec![
                                vec!["Alice".into(), "Engineer".into(), "alice@acme.dev".into()],
                                vec!["Bob".into(), "Designer".into(), "bob@acme.dev".into()],
                                vec!["Charlie".into(), "Manager".into(), "charlie@acme.dev".into()],
                            ])
                            .sort_key(self.table_sort_key.clone())
                            .sort_direction(self.table_sort_dir)
                            .on_sort({
                                let handle = cx.entity().downgrade();
                                move |key, dir, _window, cx| {
                                    if let Some(handle) = handle.upgrade() {
                                        handle.update(cx, |this, cx| {
                                            this.table_sort_key = Some(key);
                                            this.table_sort_dir = dir;
                                            cx.notify();
                                        });
                                    }
                                }
                            })
                            .on_row_click({
                                let handle = cx.entity().downgrade();
                                move |_index, _event, _window, cx| {
                                    if let Some(handle) = handle.upgrade() {
                                        handle.update(cx, |_this, cx| {
                                            cx.notify();
                                        });
                                    }
                                }
                            }),
                    )
    }
}
