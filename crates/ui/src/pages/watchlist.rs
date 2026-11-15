//! Watchlist page model.

use longbridgea_core::{Quote, Symbol};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchGroup {
    pub id: String,
    pub name: String,
    pub symbols: Vec<String>,
}

impl WatchGroup {
    pub fn default_group() -> Self {
        Self {
            id: "default".into(),
            name: "默认".into(),
            symbols: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchColumn {
    Code,
    Name,
    Price,
    ChangePct,
    Change,
    Amount,
    Turnover,
    VolumeRatio,
    High,
    Low,
}

impl WatchColumn {
    pub const DEFAULT: &'static [WatchColumn] = &[
        WatchColumn::Code,
        WatchColumn::Name,
        WatchColumn::Price,
        WatchColumn::ChangePct,
        WatchColumn::Change,
        WatchColumn::Amount,
        WatchColumn::Turnover,
    ];

    pub fn label_zh(self) -> &'static str {
        match self {
            WatchColumn::Code => "代码",
            WatchColumn::Name => "名称",
            WatchColumn::Price => "最新",
            WatchColumn::ChangePct => "涨跌幅",
            WatchColumn::Change => "涨跌额",
            WatchColumn::Amount => "成交额",
            WatchColumn::Turnover => "换手",
            WatchColumn::VolumeRatio => "量比",
            WatchColumn::High => "最高",
            WatchColumn::Low => "最低",
        }
    }
}

#[derive(Debug, Clone)]
pub struct WatchlistPage {
    pub groups: Vec<WatchGroup>,
    pub active_group_id: String,
    pub columns: Vec<WatchColumn>,
    pub rows: Vec<Quote>,
    pub filter: String,
    pub sort_column: Option<WatchColumn>,
    pub sort_desc: bool,
    /// When true, `apply_quotes` keeps active-group order instead of sort.
    pub lock_group_order: bool,
}

impl Default for WatchlistPage {
    fn default() -> Self {
        let g = WatchGroup::default_group();
        Self {
            active_group_id: g.id.clone(),
            groups: vec![g],
            columns: WatchColumn::DEFAULT.to_vec(),
            rows: Vec::new(),
            filter: String::new(),
            sort_column: Some(WatchColumn::ChangePct),
            sort_desc: true,
            lock_group_order: true,
        }
    }
}

impl WatchlistPage {
    pub fn active_group(&self) -> Option<&WatchGroup> {
        self.groups.iter().find(|g| g.id == self.active_group_id)
    }

    pub fn active_group_mut(&mut self) -> Option<&mut WatchGroup> {
        let id = self.active_group_id.clone();
        self.groups.iter_mut().find(|g| g.id == id)
    }

    pub fn add_symbol(&mut self, symbol: &Symbol) {
        let wire = symbol.tencent_wire();
        if let Some(g) = self.active_group_mut() {
            if !g.symbols.contains(&wire) {
                g.symbols.push(wire);
            }
        }
    }

    pub fn remove_symbol(&mut self, symbol: &Symbol) {
        let wire = symbol.tencent_wire();
        if let Some(g) = self.active_group_mut() {
            g.symbols.retain(|s| s != &wire);
        }
        self.rows.retain(|q| q.symbol.tencent_wire() != wire);
    }

    /// Symbols in the active group, parsed (skips invalid wires).
    pub fn watched_symbols(&self) -> Vec<Symbol> {
        self.active_group()
            .map(|g| {
                g.symbols
                    .iter()
                    .filter_map(|w| Symbol::parse(w).ok())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn watched_wires(&self) -> Vec<String> {
        self.active_group()
            .map(|g| g.symbols.clone())
            .unwrap_or_default()
    }

    /// Replace rows from a poll tick. Prefer active-group order when `lock_group_order`.
    pub fn apply_quotes(&mut self, quotes: Vec<Quote>) {
        if self.lock_group_order {
            let by_wire: HashMap<String, Quote> = quotes
                .into_iter()
                .map(|q| (q.symbol.tencent_wire(), q))
                .collect();
            let wires = self.watched_wires();
            self.rows = wires
                .into_iter()
                .filter_map(|w| by_wire.get(&w).cloned())
                .collect();
        } else {
            self.rows = quotes;
            self.sort_rows();
        }
        self.apply_filter();
    }

    fn apply_filter(&mut self) {
        let f = self.filter.trim().to_ascii_lowercase();
        if f.is_empty() {
            return;
        }
        self.rows.retain(|q| {
            q.symbol.code.to_ascii_lowercase().contains(&f)
                || q.name.to_ascii_lowercase().contains(&f)
                || q.symbol.tencent_wire().to_ascii_lowercase().contains(&f)
        });
    }

    pub fn sort_rows(&mut self) {
        let col = self.sort_column;
        let desc = self.sort_desc;
        self.rows.sort_by(|a, b| {
            let ord = match col {
                Some(WatchColumn::ChangePct) => a
                    .change_pct
                    .partial_cmp(&b.change_pct)
                    .unwrap_or(std::cmp::Ordering::Equal),
                Some(WatchColumn::Price) => {
                    a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal)
                }
                Some(WatchColumn::Amount) => {
                    a.amount.partial_cmp(&b.amount).unwrap_or(std::cmp::Ordering::Equal)
                }
                Some(WatchColumn::Code) => a.symbol.code.cmp(&b.symbol.code),
                Some(WatchColumn::Name) => a.name.cmp(&b.name),
                _ => std::cmp::Ordering::Equal,
            };
            if desc {
                ord.reverse()
            } else {
                ord
            }
        });
    }

    /// Plain-text table for headless / log output.
    pub fn render_table(&self) -> String {
        use crate::widgets::PriceChangeView;
        let mut out = String::new();
        out.push_str(&format!(
            "{:<10} {:<10} {:>10} {:>9} {:>10} {:>12}\n",
            "代码", "名称", "最新", "涨跌幅", "涨跌额", "成交额(万)"
        ));
        out.push_str(&format!("{}\n", "-".repeat(66)));
        for q in &self.rows {
            let pct = PriceChangeView::with_default_dark_cn(q.change_pct);
            out.push_str(&format!(
                "{:<10} {:<10} {:>10.2} {:>9} {:>10.2} {:>12.0}\n",
                q.symbol.tencent_wire(),
                truncate_name(&q.name, 10),
                q.price,
                pct.text,
                q.change,
                q.amount
            ));
        }
        if self.rows.is_empty() {
            out.push_str("(empty)\n");
        }
        out
    }
}

fn truncate_name(s: &str, max_chars: usize) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i >= max_chars {
            break;
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use longbridgea_core::Market;

    fn q(code: &str, name: &str, pct: f64) -> Quote {
        Quote {
            symbol: Symbol::new(Market::Sh, code),
            name: name.into(),
            price: 10.0,
            prev_close: 9.0,
            open: 9.5,
            high: 11.0,
            low: 9.0,
            change: 1.0,
            change_pct: pct,
            volume: 1.0,
            amount: 100.0,
            turnover_rate: None,
            pe: None,
            pb: None,
            amplitude: None,
            float_mv: None,
            total_mv: None,
            limit_up: None,
            limit_down: None,
            volume_ratio: None,
            update_time: None,
            bids: vec![],
            asks: vec![],
        }
    }

    #[test]
    fn apply_quotes_keeps_group_order() {
        let mut page = WatchlistPage::default();
        page.add_symbol(&Symbol::parse("sh600000").unwrap());
        page.add_symbol(&Symbol::parse("sh600519").unwrap());
        page.apply_quotes(vec![q("600519", "茅台", 1.0), q("600000", "浦发", -1.0)]);
        assert_eq!(page.rows[0].symbol.code, "600000");
        assert_eq!(page.rows[1].symbol.code, "600519");
    }
}
