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
}

#[derive(Debug, Clone)]
pub struct StockDetailPage {
    pub symbol: Option<Symbol>,
    pub quote: Option<Quote>,
    pub period: ChartPeriod,
    pub minute: Option<MinuteSeries>,
    pub bars: Vec<Bar>,
    pub in_watchlist: bool,
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
        self.period = period;
    }
}
