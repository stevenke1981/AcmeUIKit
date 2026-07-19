use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div};

impl Gallery {
    pub fn v9_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let _c = cx.theme().colors;
        Card::new()
                    .title("V9 DataGrid")
                    .description("Entity-based data grid with sort, filter, selection, edit, keyboard nav, CSV export")
                    .child(Separator::new())
                    .child(div().child("DataGrid (V9) ??Entity-based:"))
                    .child(self.data_grid_entity.clone())
    }
}
