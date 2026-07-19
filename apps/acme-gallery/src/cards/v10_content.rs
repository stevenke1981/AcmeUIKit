use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn v10_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let _c = cx.theme().colors;
        Card::new()
                    .title("V10 Content & Media")
                    .description("RichText, HtmlView, LineNumbers, DiffViewer, MarkdownPreview, DocumentOutline, FindReplace, LogViewer, HexViewer, ImageView, AvatarGroup, Carousel, Lightbox, Canvas, ZoomView, PanView, ThumbnailStrip, Cropper, AnnotationLayer")
                    // RichText
                    .child(Separator::new())
                    .child(div().child("RichText (V10):"))
                    .child(RichText::new("rt-demo").text("bold", "Bold text").text("italic", "Italic text").text("normal", "Plain text"))
                    // HtmlView
                    .child(Separator::new())
                    .child(div().child("HtmlView (V10):"))
                    .child(HtmlView::new("html-demo").html("<h1>Hello</h1><p>World</p>"))
                    // LineNumbers
                    .child(Separator::new())
                    .child(div().child("LineNumbers (V10):"))
                    .child(LineNumbers::new("ln-demo").lines(15).active_line(3))
                    // DiffViewer
                    .child(Separator::new())
                    .child(div().child("DiffViewer (V10):"))
                    .child(DiffViewer::new("diff-demo").old_text("Hello World").new_text("Hello GPUI"))
                    // MarkdownPreview
                    .child(Separator::new())
                    .child(div().child("MarkdownPreview (V10):"))
                    .child(MarkdownPreview::new("md-demo").markdown("# Title\n**bold** and *italic*"))
                    // DocumentOutline
                    .child(Separator::new())
                    .child(div().child("DocumentOutline (V10):"))
                    .child(DocumentOutline::new("outline").heading(1, "Introduction").heading(2, "Getting Started").heading(2, "Advanced"))
                    // FindReplace
                    .child(Separator::new())
                    .child(div().child("FindReplace (V10):"))
                    .child(FindReplace::new("fr-demo").find_text("search").replace_text("replace").matches(3, 1))
                    // LogViewer
                    .child(Separator::new())
                    .child(div().child("LogViewer (V10):"))
                    .child(LogViewer::new("log-demo").entry(LogLevel::Info, "10:00:00", "Application started").entry(LogLevel::Warn, "10:00:05", "Deprecated API used").entry(LogLevel::Error, "10:01:00", "Connection failed"))
                    // HexViewer
                    .child(Separator::new())
                    .child(div().child("HexViewer (V10):"))
                    .child(HexViewer::new("hex-demo").address(0x1000).data(vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21]))
                    // ImageView
                    .child(Separator::new())
                    .child(div().child("ImageView (V10):"))
                    .child(ImageView::new("img-demo").src("placeholder.png").alt("Sample image").width(px(120.)).height(px(80.)))
                    // AvatarGroup
                    .child(Separator::new())
                    .child(div().child("AvatarGroup (V10):"))
                    .child(AvatarGroup::new("av-group").add("Alice").add("Bob").add("Charlie").add("Diana").max_visible(3))
                    // Carousel
                    .child(Separator::new())
                    .child(div().child("Carousel (V10):"))
                    .child(Carousel::new("car-demo").slide("Slide 1", 0.6, 0.7, 0.5).slide("Slide 2", 0.3, 0.8, 0.6).slide("Slide 3", 0.5, 0.6, 0.7).current(0))
                    // Lightbox
                    .child(Separator::new())
                    .child(div().child("Lightbox (V10):"))
                    .child(Lightbox::new("lb-demo").src("image.png").caption("Sample image"))
                    // Canvas
                    .child(Separator::new())
                    .child(div().child("Canvas (V10):"))
                    .child(Canvas::new("canvas-demo").size(px(150.), px(80.)))
                    // ZoomView
                    .child(Separator::new())
                    .child(div().child("ZoomView (V10):"))
                    .child(ZoomView::new("zoom-demo").zoom(1.5).label("Content"))
                    // PanView
                    .child(Separator::new())
                    .child(div().child("PanView (V10):"))
                    .child(PanView::new("pan-demo").offset(10., 20.).label("Pannable content"))
                    // ThumbnailStrip
                    .child(Separator::new())
                    .child(div().child("ThumbnailStrip (V10):"))
                    .child(ThumbnailStrip::new("strip-demo").item("Frame 1").item("Frame 2").item("Frame 3").item("Frame 4").selected(1))
                    // Cropper
                    .child(Separator::new())
                    .child(div().child("Cropper (V10):"))
                    .child(Cropper::new("crop-demo").label("Crop area").aspect_ratio(16. / 9.))
                    // AnnotationLayer
                    .child(Separator::new())
                    .child(div().child("AnnotationLayer (V10):"))
                    .child(AnnotationLayer::new("ann-demo").add("Note 1", 20., 10.).add("Note 2", 100., 40.))
    }
}
