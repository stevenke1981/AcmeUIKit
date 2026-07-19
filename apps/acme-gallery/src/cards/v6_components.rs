use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn v6_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let _c = cx.theme().colors;
        Card::new()
            .title("V6 More Components")
            .description("Label, ScrollArea, Stack, Grid, IconButton, ToggleButton, SegmentedControl, Spinner, Collapsible, Accordion, Drawer, CommandPalette, EmptyState, ErrorState, StatusBar, NumberInput, SearchInput, DatePicker, Calendar, ColorPicker, PropertyGrid")
            // Label
            .child(Separator::new())
            .child(div().child("Label (V6):"))
            .child(Label::new("Username").size(Size::Small))
            .child(Label::new("Full Name"))
            .child(Label::new("Section Heading").size(Size::Large))
            // Stack + Grid
            .child(Separator::new())
            .child(div().child("Stack + Grid (V6):"))
            .child(Stack::horizontal().gap(px(4.)).child(Button::new("s1").extra_small().label("A")).child(Button::new("s2").extra_small().label("B")).spacer().child(Button::new("s3").extra_small().label("C")))
            .child(Grid::new().cols(3).gap(px(4.)).child(Badge::new("A")).child(Badge::new("B")).child(Badge::new("C")).child(Badge::new("D")).child(Badge::new("E")))
            // ScrollArea
            .child(Separator::new())
            .child(div().child("ScrollArea (V6):"))
            .child(ScrollArea::new("demo-scroll").child(div().h(px(60.)).child("Scrollable content with overflow hidden")))
            // IconButton
            .child(Separator::new())
            .child(div().child("IconButton (V6):"))
            .child(div().h_flex().gap_2().child(IconButton::new("ib-search", IconName::Search)).child(IconButton::new("ib-settings", IconName::Settings)))
            // ToggleButton + SegmentedControl
            .child(Separator::new())
            .child(div().child("ToggleButton + SegmentedControl (V6):"))
            .child(div().h_flex().gap_2().child(ToggleButton::new("tb-dark", "Dark").selected(false)).child(ToggleButton::new("tb-light", "Light").selected(true)))
            .child(SegmentedControl::new("view-mode").items(&["List", "Grid", "Cards"]))
            // Spinner
            .child(Separator::new())
            .child(div().child("Spinner (V6):"))
            .child(Spinner::new().size(Size::Small))
            // Collapsible
            .child(Separator::new())
            .child(div().child("Collapsible + Accordion (V6):"))
            .child(Collapsible::new("Click to expand").child(div().p_2().child("Hidden content revealed!")))
            .child(Accordion::new().section("Section 1", true, div().child("Content 1")).section("Section 2", false, div().child("Content 2")).section("Section 3", false, div().child("Content 3")))
            // EmptyState + ErrorState
            .child(Separator::new())
            .child(div().child("EmptyState + ErrorState (V6):"))
            .child(EmptyState::new("No items found").description("Try adjusting your search or filter to find what you are looking for."))
            .child(ErrorState::new("Failed to load data").message("An unexpected error occurred. Please try again."))
            // Drawer + CommandPalette
            .child(Separator::new())
            .child(div().child("Drawer + CommandPalette (V6):"))
            .child(Separator::new())
            .child(div().child("SearchInput + NumberInput + DatePicker + Calendar + ColorPicker + PropertyGrid (V6):"))
            .child(SearchInput::new("search-demo").placeholder("Search..."))
            .child(NumberInput::new("num-demo").value(42).min(0).max(100))
            .child(DatePicker::new("2024-12-25"))
            .child(Calendar::new("December 2024"))
            .child(ColorPicker::new("Primary", hsl(217., 0.91, 0.59)))
            .child(PropertyGrid::new().property("Name", "Acme App").property("Version", "2.1.0").property("Author", "Acme Corp"))
    }
}
