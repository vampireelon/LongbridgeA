//! Chart panel component ← `StockDetailPage` series.

use crate::pages::stock::ChartPeriod;
use crate::pages::StockDetailPage;

#[derive(Debug, Clone)]
pub struct ChartPointView {
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct ChartPanelView {
    pub period_label: String,
    pub point_count: usize,
    pub tail: Vec<ChartPointView>,
    pub error: Option<String>,
}

impl ChartPanelView {
    pub fn bind(page: &StockDetailPage) -> Self {
        let period_label = page.period.label_zh().to_string();
        let error = page.last_chart_error.clone();

        match page.period {
            ChartPeriod::Minute => {
                let points = page
                    .minute
                    .as_ref()
                    .map(|m| m.points.as_slice())
                    .unwrap_or(&[]);
                let tail: Vec<_> = points
                    .iter()
                    .rev()
                    .take(5)
                    .map(|p| ChartPointView {
                        label: p.time.clone(),
                        detail: format!("px={:.2} vol_cum={:.0}", p.price, p.volume_cum),
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                Self {
                    period_label,
                    point_count: points.len(),
                    tail,
                    error,
                }
            }
            _ => {
                let tail: Vec<_> = page
                    .bars
                    .iter()
                    .rev()
                    .take(5)
                    .map(|b| ChartPointView {
                        label: b.date.clone(),
                        detail: format!(
                            "O={:.2} C={:.2} H={:.2} L={:.2} V={:.0}",
                            b.open, b.close, b.high, b.low, b.volume
                        ),
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                Self {
                    period_label,
                    point_count: page.bars.len(),
                    tail,
                    error,
                }
            }
        }
    }

    pub fn render_text(&self) -> String {
        let mut out = format!(
            "  图表·{}  points/bars={}\n",
            self.period_label, self.point_count
        );
        if let Some(err) = &self.error {
            out.push_str(&format!("  chart error: {err}\n"));
        }
        for p in &self.tail {
            out.push_str(&format!("    {}  {}\n", p.label, p.detail));
        }
        if self.tail.is_empty() && self.error.is_none() {
            out.push_str("    (no series)\n");
        }
        out
    }
}
