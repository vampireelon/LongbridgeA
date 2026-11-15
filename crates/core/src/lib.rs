//! LongbridgeA core domain types and screener primitives.
//!
//! This crate must stay free of GUI and concrete HTTP clients.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Market prefix used in Tencent-style symbols (`sh`, `sz`, `bj`, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Market {
    Sh,
    Sz,
    Bj,
    Unknown,
}

impl Market {
    pub fn as_str(self) -> &'static str {
        match self {
            Market::Sh => "sh",
            Market::Sz => "sz",
            Market::Bj => "bj",
            Market::Unknown => "",
        }
    }
}

/// A normalized security identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol {
    pub market: Market,
    /// Bare code without prefix, e.g. `600519` or index `000001`.
    pub code: String,
}

impl Symbol {
    pub fn new(market: Market, code: impl Into<String>) -> Self {
        Self {
            market,
            code: code.into(),
        }
    }

    /// Tencent wire form: `sh600519`, `sz000001`.
    pub fn tencent_wire(&self) -> String {
        format!("{}{}", self.market.as_str(), self.code)
    }

    /// Parse `sh600519`, `SZ000001`, bare `600519` (inferred), or `600519.SH`.
    pub fn parse(input: &str) -> Result<Self> {
        let s = input.trim();
        if s.is_empty() {
            return Err(CoreError::InvalidSymbol(input.to_string()));
        }

        let lower = s.to_ascii_lowercase();

        if let Some((code, mkt)) = lower.split_once('.') {
            let market = match mkt {
                "sh" | "xshg" => Market::Sh,
                "sz" | "xshe" => Market::Sz,
                "bj" => Market::Bj,
                _ => Market::Unknown,
            };
            if code.is_empty() || market == Market::Unknown {
                return Err(CoreError::InvalidSymbol(input.to_string()));
            }
            return Ok(Self::new(market, code));
        }

        if lower.len() > 2 {
            let prefix = &lower[..2];
            let rest = &lower[2..];
            if rest.chars().all(|c| c.is_ascii_digit()) {
                let market = match prefix {
                    "sh" => Market::Sh,
                    "sz" => Market::Sz,
                    "bj" => Market::Bj,
                    _ => Market::Unknown,
                };
                if market != Market::Unknown {
                    return Ok(Self::new(market, rest));
                }
            }
        }

        if lower.chars().all(|c| c.is_ascii_digit()) {
            let market = match lower.as_bytes().first() {
                Some(b'5') | Some(b'6') | Some(b'9') => Market::Sh,
                Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') => Market::Sz,
                Some(b'4') | Some(b'8') => Market::Bj,
                _ => Market::Unknown,
            };
            if market == Market::Unknown {
                return Err(CoreError::InvalidSymbol(input.to_string()));
            }
            return Ok(Self::new(market, lower));
        }

        Err(CoreError::InvalidSymbol(input.to_string()))
    }
}

/// One level of the order book (price + volume in hands).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepthLevel {
    pub price: f64,
    pub volume: f64,
}

/// Snapshot quote used across UI and screener.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: Symbol,
    pub name: String,
    pub price: f64,
    pub prev_close: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub change: f64,
    pub change_pct: f64,
    /// Volume in **hands** for most boards (Tencent field 6 / 36).
    pub volume: f64,
    /// Turnover in **万元** when from Tencent field 37.
    pub amount: f64,
    pub turnover_rate: Option<f64>,
    pub pe: Option<f64>,
    pub pb: Option<f64>,
    /// Amplitude % (Tencent field 43).
    pub amplitude: Option<f64>,
    /// 流通市值 亿元 (field 44).
    pub float_mv: Option<f64>,
    /// 总市值 亿元 (field 45).
    pub total_mv: Option<f64>,
    pub limit_up: Option<f64>,
    pub limit_down: Option<f64>,
    pub volume_ratio: Option<f64>,
    /// Tencent field 30, often `YYYYMMDDHHMMSS`.
    pub update_time: Option<String>,
    pub bids: Vec<DepthLevel>,
    pub asks: Vec<DepthLevel>,
}

/// One OHLCV bar (date/time encoded in `ts` as unix seconds when known, else 0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    pub ts: i64,
    /// Calendar date `YYYY-MM-DD` when from daily K; empty for pure minute aggregates.
    pub date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// One intraday minute point from Tencent minute API.
/// Volume/amount are **cumulative** from open unless noted otherwise.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinutePoint {
    /// `HHMM` e.g. `0930`, `1300`.
    pub time: String,
    pub price: f64,
    pub volume_cum: f64,
    pub amount_cum: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MinuteSeries {
    pub symbol: Symbol,
    /// Trade date `YYYYMMDD` when provided by API.
    pub date: Option<String>,
    pub points: Vec<MinutePoint>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KlinePeriod {
    Day,
    Week,
    Month,
}

impl KlinePeriod {
    pub fn as_tencent(self) -> &'static str {
        match self {
            KlinePeriod::Day => "day",
            KlinePeriod::Week => "week",
            KlinePeriod::Month => "month",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Adjust {
    /// 前复权
    Qfq,
    /// 后复权
    Hfq,
    /// 不复权
    None,
}

impl Adjust {
    pub fn as_tencent_suffix(self) -> &'static str {
        match self {
            Adjust::Qfq => "qfq",
            Adjust::Hfq => "hfq",
            Adjust::None => "",
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid symbol: {0}")]
    InvalidSymbol(String),
    #[error("{0}")]
    Msg(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_prefixed_and_bare() {
        let a = Symbol::parse("sh600519").unwrap();
        assert_eq!(a.market, Market::Sh);
        assert_eq!(a.code, "600519");
        assert_eq!(a.tencent_wire(), "sh600519");

        let b = Symbol::parse("000001").unwrap();
        assert_eq!(b.market, Market::Sz);

        let c = Symbol::parse("600519.SH").unwrap();
        assert_eq!(c.market, Market::Sh);
        assert_eq!(c.code, "600519");
    }
}
