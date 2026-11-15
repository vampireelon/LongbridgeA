//! Market overview page model.

use longbridgea_core::Quote;

#[derive(Debug, Clone, Default)]
pub struct MarketPage {
    /// Major indices (上证 / 深成 / 创业板 / 沪深300 …).
    pub indices: Vec<Quote>,
    pub gainers: Vec<Quote>,
    pub losers: Vec<Quote>,
    pub amount_leaders: Vec<Quote>,
}

impl MarketPage {
    pub fn set_indices(&mut self, quotes: Vec<Quote>) {
        self.indices = quotes;
    }
}
