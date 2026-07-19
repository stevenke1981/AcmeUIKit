use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div};

impl Gallery {
    pub fn v8_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let _c = cx.theme().colors;
        Card::new()
                    .title("V8 Desktop Shell")
                    .description("TitleBarControls, AppMenuBar, NavigationRail, NavigationView, SplitView, InspectorPanel, ContextToolbar, ShortcutManager, SystemTray, FocusRing, FocusScope, DragRegion, DropZone, ResizeHandleOverlay, AboutDialog")
                    // TitleBar
                    .child(Separator::new())
                    .child(div().child("TitleBar (V8):"))
                    .child(TitleBar::new("title-bar").title("My App").subtitle("v1.0.0"))
                    // AppMenuBar
                    .child(Separator::new())
                    .child(div().child("AppMenuBar (V8):"))
                    .child(AppMenuBar::new().add("File", &["New", "Open", "Save"]).add("Edit", &["Undo", "Redo"]).add("Help", &["About"]))
                    // NavigationRail + NavigationView
                    .child(Separator::new())
                    .child(div().child("NavigationRail + NavigationView (V8):"))
                    .child(NavigationView::new("nav-view").rail(NavigationRail::new("nav-rail").item(IconName::Menu, "Home").item(IconName::Settings, "Settings").item(IconName::User, "Profile").selected(0)))
                    // SplitView
                    .child(Separator::new())
                    .child(div().child("SplitView (V8):"))
                    .child(SplitView::new("split-view").ratio(0.4).left_label("Left Panel").right_label("Right Panel"))
                    // InspectorPanel
                    .child(Separator::new())
                    .child(div().child("InspectorPanel (V8):"))
                    .child(InspectorPanel::new("inspector").section("Properties", &[("Width", "800"), ("Height", "600"), ("Opacity", "1.0")]))
                    // ContextToolbar
                    .child(Separator::new())
                    .child(div().child("ContextToolbar (V8):"))
                    .child(ContextToolbar::new("ctx-bar").items(&["Bold", "Italic", "Underline"]))
                    // ShortcutManager
                    .child(Separator::new())
                    .child(div().child("ShortcutManager (V8):"))
                    .child(ShortcutManager::new("shortcuts").shortcut("Ctrl+S", "Save").shortcut("Ctrl+Z", "Undo").shortcut("Ctrl+Shift+Z", "Redo"))
                    // SystemTray
                    .child(Separator::new())
                    .child(div().child("SystemTray (V8):"))
                    .child(div().h_flex().gap_2().child(SystemTray::new("tray-1").icon(IconName::Settings)).child(SystemTray::new("tray-2").icon(IconName::User)))
                    // FocusRing + FocusScope
                    .child(Separator::new())
                    .child(div().child("FocusRing + FocusScope (V8):"))
                    .child(FocusScope::new("focus-scope").label("Focus Group"))
                    .child(div().child(FocusRing::new("ring-1").label("Input 1").focused(true)))
                    .child(div().child(FocusRing::new("ring-2").label("Input 2")))
                    // DragRegion
                    .child(Separator::new())
                    .child(div().child("DragRegion (V8):"))
                    .child(DragRegion::new("drag-region").label("Drag me"))
                    // DropZone
                    .child(Separator::new())
                    .child(div().child("DropZone (V8):"))
                    .child(DropZone::new("drop-zone").label("Drop files here").hint("Supports PNG, JPG, SVG"))
                    // ResizeHandle
                    .child(Separator::new())
                    .child(div().child("ResizeHandle (V8):"))
                    .child(ResizeHandle::new("resize-grip"))
                    // WindowOverlay
                    .child(Separator::new())
                    .child(div().child("WindowOverlay (V8) ??always visible in demo:"))
                    .child(WindowOverlay::new("overlay").title("Modal Title"))
                    // AboutDialog
                    .child(Separator::new())
                    .child(div().child("AboutDialog (V8):"))
                    .child(AboutDialog::new("about").app_name("Acme UI Kit").version("v1.0.0").description("A GPUI component library"))
    }
}
