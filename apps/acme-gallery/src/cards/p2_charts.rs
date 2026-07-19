use crate::Gallery;
use acme_ui::*;
use gpui::{Context, IntoElement, ParentElement as _, Styled as _, div, px};

impl Gallery {
    pub fn p2_charts_card(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let _c = cx.theme().colors;
        Card::new()
                    .title("P2 Charts")
                    .description("BarChart, LineChart, AreaChart, PieChart, DonutChart, ScatterChart, Gauge, Sparkline, Histogram, Heatmap, CandlestickChart, StreamingChart")
                    // BarChart (existing)
                    .child(Separator::new())
                    .child(div().child("BarChart (V2):"))
                    .child(BarChart::new("bc1").height(px(120.)).bars(vec![
                        BarEntry::new("A", 30.0).color(ChartColors::get(0)),
                        BarEntry::new("B", 50.0).color(ChartColors::get(1)),
                        BarEntry::new("C", 40.0).color(ChartColors::get(2)),
                        BarEntry::new("D", 70.0).color(ChartColors::get(3)),
                    ]))
                    // LineChart
                    .child(Separator::new())
                    .child(div().child("LineChart:"))
                    .child(LineChart::new("lc1").height(px(100.)).show_dots(true).series(vec![
                        ChartSeries::new("Series 1").color(ChartColors::get(0)).data(vec![10.0, 25.0, 15.0, 40.0, 30.0, 55.0, 45.0]),
                        ChartSeries::new("Series 2").color(ChartColors::get(1)).data(vec![20.0, 15.0, 35.0, 25.0, 45.0, 35.0, 50.0]),
                    ]))
                    // AreaChart
                    .child(Separator::new())
                    .child(div().child("AreaChart:"))
                    .child(AreaChart::new("ac1").height(px(100.)).fill_opacity(0.2).series(vec![
                        ChartSeries::new("Series").color(ChartColors::get(2)).data(vec![10.0, 30.0, 20.0, 50.0, 35.0, 60.0, 45.0]),
                    ]))
                    // PieChart
                    .child(Separator::new())
                    .child(div().child("PieChart:"))
                    .child(PieChart::new("pc1").size(px(120.)).slices(vec![
                        PieSlice::new("Red", 30.0).color(ChartColors::get(0)),
                        PieSlice::new("Blue", 25.0).color(ChartColors::get(1)),
                        PieSlice::new("Green", 20.0).color(ChartColors::get(2)),
                        PieSlice::new("Orange", 15.0).color(ChartColors::get(3)),
                        PieSlice::new("Purple", 10.0).color(ChartColors::get(4)),
                    ]))
                    // DonutChart
                    .child(Separator::new())
                    .child(div().child("DonutChart:"))
                    .child(DonutChart::new("dc1").size(px(120.)).hole_ratio(0.6).center_text("Total").slices(vec![
                        PieSlice::new("A", 40.0).color(ChartColors::get(0)),
                        PieSlice::new("B", 30.0).color(ChartColors::get(1)),
                        PieSlice::new("C", 20.0).color(ChartColors::get(2)),
                        PieSlice::new("D", 10.0).color(ChartColors::get(3)),
                    ]))
                    // ScatterChart
                    .child(Separator::new())
                    .child(div().child("ScatterChart:"))
                    .child(ScatterChart::new("sc1").height(px(100.)).show_grid(true).series(vec![
                        ScatterSeries::new("Series A").color(ChartColors::get(0))
                            .points(vec![
                                ScatterPoint::new(10.0, 20.0), ScatterPoint::new(30.0, 50.0),
                                ScatterPoint::new(50.0, 30.0), ScatterPoint::new(70.0, 80.0),
                                ScatterPoint::new(90.0, 45.0),
                            ]),
                        ScatterSeries::new("Series B").color(ChartColors::get(1))
                            .points(vec![
                                ScatterPoint::new(15.0, 60.0), ScatterPoint::new(35.0, 25.0),
                                ScatterPoint::new(55.0, 70.0), ScatterPoint::new(75.0, 35.0),
                                ScatterPoint::new(85.0, 90.0),
                            ]),
                    ]))
                    // Gauge
                    .child(Separator::new())
                    .child(div().child("Gauge:"))
                    .child(div().h_flex().gap_3().child(Gauge::new("g1").value(25.0).size(px(80.)).label("Low")).child(Gauge::new("g2").value(55.0).size(px(80.)).label("Mid")).child(Gauge::new("g3").value(85.0).size(px(80.)).label("High")))
                    // Sparkline
                    .child(Separator::new())
                    .child(div().child("Sparkline:"))
                    .child(Sparkline::new("sl1").data(vec![3.0, 8.0, 5.0, 12.0, 7.0, 15.0, 9.0, 18.0, 11.0, 22.0, 14.0, 20.0]).height(px(40.)).color(ChartColors::get(2)))
                    // Histogram
                    .child(Separator::new())
                    .child(div().child("Histogram:"))
                    .child(Histogram::new("h1").height(px(100.)).bins(vec![
                        HistogramBin::new("0-10", 5.0).color(ChartColors::get(0)),
                        HistogramBin::new("10-20", 12.0).color(ChartColors::get(1)),
                        HistogramBin::new("20-30", 18.0).color(ChartColors::get(2)),
                        HistogramBin::new("30-40", 25.0).color(ChartColors::get(3)),
                        HistogramBin::new("40-50", 15.0).color(ChartColors::get(4)),
                        HistogramBin::new("50-60", 8.0).color(ChartColors::get(5)),
                    ]))
                    // Heatmap
                    .child(Separator::new())
                    .child(div().child("Heatmap:"))
                    .child(Heatmap::new("hm1").cell_size(px(24.)).col_labels(vec!["Mon", "Tue", "Wed", "Thu", "Fri"]).rows(vec![
                        vec![
                            HeatmapCell::new(0.2), HeatmapCell::new(0.5), HeatmapCell::new(0.8), HeatmapCell::new(0.3), HeatmapCell::new(0.1),
                        ],
                        vec![
                            HeatmapCell::new(0.7), HeatmapCell::new(0.9), HeatmapCell::new(0.4), HeatmapCell::new(0.6), HeatmapCell::new(0.2),
                        ],
                        vec![
                            HeatmapCell::new(0.3), HeatmapCell::new(0.6), HeatmapCell::new(1.0), HeatmapCell::new(0.5), HeatmapCell::new(0.4),
                        ],
                    ]))
                    // CandlestickChart
                    .child(Separator::new())
                    .child(div().child("CandlestickChart:"))
                    .child(CandlestickChart::new("cc1").height(px(100.)).data(vec![
                        Candlestick::new("Mon", 25.0, 35.0, 20.0, 30.0),
                        Candlestick::new("Tue", 30.0, 45.0, 28.0, 42.0),
                        Candlestick::new("Wed", 42.0, 48.0, 38.0, 40.0),
                        Candlestick::new("Thu", 40.0, 55.0, 38.0, 50.0),
                        Candlestick::new("Fri", 50.0, 60.0, 45.0, 48.0),
                    ]))
                    // StreamingChart
                    .child(Separator::new())
                    .child(div().child("StreamingChart:"))
                    .child(StreamingChart::new("st1").height(px(60.)).color(ChartColors::get(3)).data(vec![
                        10.0, 12.0, 8.0, 15.0, 18.0, 14.0, 20.0, 22.0, 19.0, 25.0,
                        28.0, 24.0, 30.0, 27.0, 32.0, 35.0, 30.0, 38.0, 42.0, 40.0,
                    ]).show_latest_value(true))
                    // Legend
                    .child(Separator::new())
                    .child(div().child("Legend:"))
                    .child(Legend::new("leg1").items(vec![
                        LegendItem::new("Series A", ChartColors::get(0)),
                        LegendItem::new("Series B", ChartColors::get(1)),
                        LegendItem::new("Series C", ChartColors::get(2)),
                        LegendItem::new("Series D", ChartColors::get(3)),
                    ]).layout(LegendLayout::Horizontal))
    }
}
