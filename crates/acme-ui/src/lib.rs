//! Acme UI is a compact, clean-room component starter library for GPUI.

// Categorized modules — see 각 category mod.rs for individual sub-modules.
pub mod charts;
pub mod data;
pub mod desktop;
pub mod foundations;
pub mod infra;
pub mod inputs;
pub mod layout;
pub mod media;
pub mod overlay;

// Re-exports (flat public API — backward compatible)
pub use foundations::primitives::{Size, Tone};
pub use foundations::styled::StyledExt;
pub use foundations::theme::{
    ActiveTheme, ControlSizes, Density, FontSizes, Motion, RadiusScale, Spacing, TextStyle, Theme,
    ThemeColors, ThemeMode, Typography, hsl,
};

pub use charts::area_chart::AreaChart;
pub use charts::candlestick::{Candlestick, CandlestickChart};
pub use charts::chart::{BarChart, BarEntry};
pub use charts::chart_base::{
    Axis, AxisOrientation, ChartColors, ChartSeries, Legend, LegendItem, LegendLayout, Scale,
    ScaleType,
};
pub use charts::gauge::Gauge;
pub use charts::heatmap::{Heatmap, HeatmapCell};
pub use charts::histogram::{Histogram, HistogramBin};
pub use charts::line_chart::LineChart;
pub use charts::pie_chart::{DonutChart, PieChart, PieSlice};
pub use charts::scatter_chart::{ScatterChart, ScatterPoint, ScatterSeries};
pub use charts::sparkline::Sparkline;
pub use charts::streaming_chart::StreamingChart;
pub use data::data_grid::{DataGrid, DataGridColumn, DataGridRow};
pub use data::list::{List, ListItem};
pub use data::pagination::Pagination;
pub use data::property_grid::PropertyGrid;
pub use data::table::{SortDirection, Table, TableColumn};
pub use data::tree::{Tree, TreeNode};
pub use data::virtual_list::VirtualList;
pub use desktop::app_menu_bar::AppMenuBar;
pub use desktop::context_toolbar::ContextToolbar;
pub use desktop::drag_region::DragRegion;
pub use desktop::drop_zone::DropZone;
pub use desktop::inspector_panel::InspectorPanel;
pub use desktop::navigation_rail::NavigationRail;
pub use desktop::navigation_view::NavigationView;
pub use desktop::resize_handle::ResizeHandle;
pub use desktop::shortcut_manager::ShortcutManager;
pub use desktop::system_tray::SystemTray;
pub use desktop::title_bar::TitleBar;
pub use desktop::window_controls::WindowControls;
pub use foundations::accordion::Accordion;
pub use foundations::alert::Alert;
pub use foundations::avatar::Avatar;
pub use foundations::badge::Badge;
pub use foundations::button::Button;
pub use foundations::card::Card;
pub use foundations::empty_state::EmptyState;
pub use foundations::error_state::ErrorState;
pub use foundations::icon_button::IconButton;
pub use foundations::icons::{Icon, IconName};
pub use foundations::kbd::Kbd;
pub use foundations::label::Label;
pub use foundations::loading_state::LoadingState;
pub use foundations::loading_state::render_loading_state;
pub use foundations::progress::Progress;
pub use foundations::separator::Separator;
pub use foundations::skeleton::Skeleton;
pub use foundations::spinner::Spinner;
pub use foundations::switch::Switch;
pub use foundations::tag::Tag;
pub use infra::accessibility::{
    AriaAttrs, AriaChecked, AriaLive, AriaPopup, AriaPressed, AriaRole,
    animation_duration_multiplier, focus_ring_style, prefers_high_contrast, prefers_reduced_motion,
};
pub use infra::focus::{
    DefaultCancelButtons, FocusTrap, RovingOrientation, RovingTabIndex, arrow_key_nav_handler,
    escape_close_handler,
};
pub use infra::focus_ring::FocusRing;
pub use infra::focus_scope::FocusScope;
pub use infra::states::{
    StateStyling, aria_label, disabled_opacity, render_disabled_overlay, render_loading_overlay,
    sr_only_label, validation_border_color,
};
pub use inputs::autocomplete::Autocomplete;
pub use inputs::calendar::Calendar;
pub use inputs::checkbox::Checkbox;
pub use inputs::color_picker::ColorPicker;
pub use inputs::combobox::{Combobox, ComboboxOption};
pub use inputs::date_picker::DatePicker;
pub use inputs::date_range_picker::DateRangePicker;
pub use inputs::field::FieldShell;
pub use inputs::file_picker::FilePicker;
pub use inputs::form::validators;
pub use inputs::form::{Field, Form};
pub use inputs::form_message::FormMessage;
pub use inputs::masked_input::MaskedInput;
pub use inputs::multi_select::MultiSelect;
pub use inputs::number_input::NumberInput;
pub use inputs::password_input::PasswordInput;
pub use inputs::pin_input::PinInput;
pub use inputs::radio::{Radio, RadioGroup};
pub use inputs::range_slider::RangeSlider;
pub use inputs::rating::Rating;
pub use inputs::search_input::SearchInput;
pub use inputs::segmented_control::SegmentedControl;
pub use inputs::select::{Select, SelectOption};
pub use inputs::slider::Slider;
pub use inputs::text_input::TextInput;
pub use inputs::textarea::Textarea;
pub use inputs::time_picker::TimePicker;
pub use inputs::toggle_button::ToggleButton;
pub use layout::breadcrumb::Breadcrumb;
pub use layout::collapsible::Collapsible;
pub use layout::dock::{Dock, DockArea, DockPanel};
pub use layout::grid::Grid;
pub use layout::resizable::{Direction, Resizable};
pub use layout::scroll_area::ScrollArea;
pub use layout::settings::{SettingsGroup, SettingsPage, SettingsRow};
pub use layout::sidebar::Sidebar;
pub use layout::split_view::SplitView;
pub use layout::stack::Stack;
pub use layout::status_bar::StatusBar;
pub use layout::stepper::Stepper;
pub use layout::tabs::Tabs;
pub use layout::tiles::{Tile, TileDirection, Tiles};
pub use layout::toolbar::Toolbar;
pub use media::annotation_layer::{Annotation, AnnotationLayer};
pub use media::avatar_group::AvatarGroup;
pub use media::canvas::Canvas;
pub use media::carousel::{Carousel, CarouselSlide};
pub use media::cropper::Cropper;
pub use media::diff_viewer::DiffViewer;
pub use media::document_outline::{DocumentOutline, OutlineEntry};
pub use media::find_replace::FindReplace;
pub use media::hex_viewer::HexViewer;
pub use media::html_view::HtmlView;
pub use media::image::ImageView;
pub use media::image_viewer::{ImageFit, ImageViewer};
pub use media::lightbox::Lightbox;
pub use media::line_numbers::LineNumbers;
pub use media::log_viewer::{LogEntry, LogLevel, LogViewer};
pub use media::markdown::Markdown;
pub use media::markdown_preview::MarkdownPreview;
pub use media::pan_view::PanView;
pub use media::rich_text::RichText;
pub use media::thumbnail_strip::{ThumbnailItem, ThumbnailStrip};
pub use media::zoom_view::ZoomView;
pub use overlay::about_dialog::AboutDialog;
pub use overlay::command_palette::CommandPalette;
pub use overlay::dialog::Dialog;
pub use overlay::drawer::Drawer;
pub use overlay::menu::{Menu, MenuItem};
pub use overlay::notification::{NotificationLevel, Notifications};
pub use overlay::overlay_manager::{
    AutoPositioner, ClickOutsideListener, FocusRestore, ModalBackdrop, OverlayDepth, OverlayEntry,
    Placement,
};
pub use overlay::popover::Popover;
pub use overlay::tooltip::Tooltip;
pub use overlay::window_overlay::WindowOverlay;

use gpui::App;

/// Initializes Acme UI global state.
pub fn init(cx: &mut App) {
    foundations::theme::init(cx);
}
