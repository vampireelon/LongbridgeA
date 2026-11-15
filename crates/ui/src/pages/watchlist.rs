//! Watchlist page model.

use longbridgea_core::{Quote, Symbol};
use serde::{Deserialize, Serialize};

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
        }
    }
}

impl WatchlistPage {
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

    pub fn apply_quotes(&mut self, quotes: Vec<Quote>) {
        self.rows = quotes;
        self.sort_rows();
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
}
