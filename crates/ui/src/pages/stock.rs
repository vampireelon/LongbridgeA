//! Stock detail page model.

use longbridgea_core::{Bar, MinuteInterval, MinuteSeries, Quote, Symbol};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChartPeriod {
    Minute,
    Day,
    Week,
    Month,
    M1,
    M5,
    M15,
    M30,
    M60,
}

impl ChartPeriod {
    pub const TOOLBAR: &'static [ChartPeriod] = &[
        ChartPeriod::Minute,
        ChartPeriod::Day,
        ChartPeriod::Week,
        ChartPeriod::Month,
        ChartPeriod::M5,
        ChartPeriod::M15,
        ChartPeriod::M30,
        ChartPeriod::M60,
    ];

    pub fn label_zh(self) -> &'static str {
        match self {
            ChartPeriod::Minute => "分时",
            ChartPeriod::Day => "日K",
            ChartPeriod::Week => "周K",
            ChartPeriod::Month => "月K",
            ChartPeriod::M1 => "1分",
            ChartPeriod::M5 => "5分",
            ChartPeriod::M15 => "15分",
            ChartPeriod::M30 => "30分",
            ChartPeriod::M60 => "60分",
        }
    }

    pub fn as_minute_interval(self) -> Option<MinuteInterval> {
        match self {
            ChartPeriod::M1 => Some(MinuteInterval::M1),
            ChartPeriod::M5 => Some(MinuteInterval::M5),
            ChartPeriod::M15 => Some(MinuteInterval::M15),
            ChartPeriod::M30 => Some(MinuteInterval::M30),
            ChartPeriod::M60 => Some(MinuteInterval::M60),
            _ => None,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "minute" | "min" | "fs" | "分时" => Some(ChartPeriod::Minute),
            "day" | "d" | "日" | "日k" => Some(ChartPeriod::Day),
            "week" | "w" | "周" | "周k" => Some(ChartPeriod::Week),
            "month" | "m" | "月" | "月k" => Some(ChartPeriod::Month),
            "m1" | "1" | "1分" => Some(ChartPeriod::M1),
            "m5" | "5" | "5分" => Some(ChartPeriod::M5),
            "m15" | "15" | "15分" => Some(ChartPeriod::M15),
            "m30" | "30" | "30分" => Some(ChartPeriod::M30),
            "m60" | "60" | "60分" => Some(ChartPeriod::M60),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StockDetailPage {
    pub symbol: Option<Symbol>,
    pub quote: Option<Quote>,
    pub period: ChartPeriod,
    pub minute: Option<MinuteSeries>,
    pub bars: Vec<Bar>,
    pub in_watchlist: bool,
    /// Last chart load error (non-fatal for quote path).
    pub last_chart_error: Option<String>,
}

impl Default for StockDetailPage {
    fn default() -> Self {
        Self {
            symbol: None,
            quote: None,
            period: ChartPeriod::Minute,
            minute: None,
            bars: Vec::new(),
            in_watchlist: false,
            last_chart_error: None,
        }
    }
}

impl StockDetailPage {
    pub fn open(symbol: Symbol) -> Self {
        Self {
            symbol: Some(symbol),
            ..Self::default()
        }
    }

    pub fn set_period(&mut self, period: ChartPeriod) {
        if self.period != period {
            self.period = period;
            // Clear series that no longer match the selected period.
            match period {
                ChartPeriod::Minute => {
                    self.bars.clear();
                }
                _ => {
                    self.minute = None;
                }
            }
        }
    }

    pub fn apply_quote(&mut self, quote: Quote) {
        self.symbol = Some(quote.symbol.clone());
        self.quote = Some(quote);
    }

    pub fn apply_minute(&mut self, series: MinuteSeries) {
        self.minute = Some(series);
        self.last_chart_error = None;
    }

    pub fn apply_bars(&mut self, bars: Vec<Bar>) {
        self.bars = bars;
        self.last_chart_error = None;
    }

    pub fn set_chart_error(&mut self, err: impl Into<String>) {
        self.last_chart_error = Some(err.into());
    }

    /// CLI / log summary: header + depth + chart tail.
    pub fn render_summary(&self) -> String {
        use crate::widgets::PriceChangeView;
        let mut out = String::new();

        match (&self.symbol, &self.quote) {
            (Some(sym), Some(q)) => {
                let pct = PriceChangeView::with_default_dark_cn(q.change_pct);
                out.push_str(&format!(
                    "{}  {}  {}  {} ({:+.2})\n",
                    sym.tencent_wire(),
                    q.name,
                    q.price,
                    pct.text,
                    q.change
                ));
                out.push_str(&format!(
                    "  开 {:.2}  高 {:.2}  低 {:.2}  昨收 {:.2}\n",
                    q.open, q.high, q.low, q.prev_close
                ));
                out.push_str(&format!(
                    "  量 {:.0}手  额 {:.0}万  换手 {}  量比 {}\n",
                    q.volume,
                    q.amount,
                    opt_f64(q.turnover_rate),
                    opt_f64(q.volume_ratio)
                ));
                out.push_str(&format!(
                    "  市值 {}亿  PE {}  PB {}  振幅 {}\n",
                    opt_f64(q.total_mv),
                    opt_f64(q.pe),
                    opt_f64(q.pb),
                    opt_f64(q.amplitude)
                ));
                if !q.bids.is_empty() || !q.asks.is_empty() {
                    out.push_str("  五档(只读)\n");
                    for (i, a) in q.asks.iter().rev().take(5).enumerate() {
                        out.push_str(&format!(
                            "    卖{:<2} {:>8.2}  {:>8.0}\n",
                            5 - i,
                            a.price,
                            a.volume
                        ));
                    }
                    for (i, b) in q.bids.iter().take(5).enumerate() {
                        out.push_str(&format!(
                            "    买{:<2} {:>8.2}  {:>8.0}\n",
                            i + 1,
                            b.price,
                            b.volume
                        ));
                    }
                }
            }
            (Some(sym), None) => {
                out.push_str(&format!("{}  (no quote yet)\n", sym.tencent_wire()));
            }
            _ => out.push_str("(no symbol)\n"),
        }

        out.push_str(&format!("  周期: {}\n", self.period.label_zh()));
        if let Some(err) = &self.last_chart_error {
            out.push_str(&format!("  chart error: {err}\n"));
        }

        match self.period {
            ChartPeriod::Minute => {
                if let Some(m) = &self.minute {
                    out.push_str(&format!(
                        "  分时 points={} date={:?}\n",
                        m.points.len(),
                        m.date
                    ));
                    for p in m.points.iter().rev().take(3).collect::<Vec<_>>().into_iter().rev()
                    {
                        out.push_str(&format!(
                            "    {}  px={:.2}  vol_cum={:.0}\n",
                            p.time, p.price, p.volume_cum
                        ));
                    }
                } else {
                    out.push_str("  分时: (empty)\n");
                }
            }
            _ => {
                out.push_str(&format!("  K线 bars={}\n", self.bars.len()));
                for b in self.bars.iter().rev().take(5).collect::<Vec<_>>().into_iter().rev()
                {
                    out.push_str(&format!(
                        "    {} O={:.2} C={:.2} H={:.2} L={:.2} V={:.0}\n",
                        b.date, b.open, b.close, b.high, b.low, b.volume
                    ));
                }
            }
        }

        out
    }
}

fn opt_f64(v: Option<f64>) -> String {
    match v {
        Some(x) => format!("{x:.2}"),
        None => "-".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn period_parse() {
        assert_eq!(ChartPeriod::parse("m5"), Some(ChartPeriod::M5));
        assert_eq!(ChartPeriod::parse("分时"), Some(ChartPeriod::Minute));
        assert_eq!(ChartPeriod::parse("day"), Some(ChartPeriod::Day));
    }
}
