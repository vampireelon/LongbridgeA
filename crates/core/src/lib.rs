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

/// A normalized security identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol {
    pub market: Market,
    /// Bare 6-digit (or index) code without prefix, e.g. `600519`.
    pub code: String,
}

impl Symbol {
    pub fn tencent_wire(&self) -> String {
        let prefix = match self.market {
            Market::Sh => "sh",
            Market::Sz => "sz",
            Market::Bj => "bj",
            Market::Unknown => "",
        };
        format!("{prefix}{}", self.code)
    }
}

/// Snapshot quote used across UI and screener (subset; expand in Phase 1).
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
    /// Volume in hands (typical for main board); document unit at parse site.
    pub volume: f64,
    /// Turnover in 万元 when sourced from Tencent field 37.
    pub amount: f64,
    pub turnover_rate: Option<f64>,
    pub pe: Option<f64>,
    pub pb: Option<f64>,
}

/// One OHLCV bar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    pub ts: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Placeholder error type for domain operations.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid symbol: {0}")]
    InvalidSymbol(String),
    #[error("{0}")]
    Msg(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
