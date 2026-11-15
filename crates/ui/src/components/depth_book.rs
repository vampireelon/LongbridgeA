//! Order-book (five levels, read-only) ← `Quote`.

use crate::pages::StockDetailPage;
use longbridgea_core::Quote;

#[derive(Debug, Clone)]
pub struct DepthLevelView {
    pub label: String,
    pub price_text: String,
    pub volume_text: String,
}

#[derive(Debug, Clone, Default)]
pub struct DepthBookView {
    pub asks: Vec<DepthLevelView>,
    pub bids: Vec<DepthLevelView>,
}

impl DepthBookView {
    pub fn from_quote(q: &Quote) -> Self {
        let asks: Vec<_> = q
            .asks
            .iter()
            .rev()
            .take(5)
            .enumerate()
            .map(|(i, l)| DepthLevelView {
                label: format!("卖{}", 5 - i),
                price_text: format!("{:.2}", l.price),
                volume_text: format!("{:.0}", l.volume),
            })
            .collect();
        let bids: Vec<_> = q
            .bids
            .iter()
            .take(5)
            .enumerate()
            .map(|(i, l)| DepthLevelView {
                label: format!("买{}", i + 1),
                price_text: format!("{:.2}", l.price),
                volume_text: format!("{:.0}", l.volume),
            })
            .collect();
        Self { asks, bids }
    }

    pub fn bind_page(page: &StockDetailPage) -> Self {
        page.quote
            .as_ref()
            .map(Self::from_quote)
            .unwrap_or_default()
    }

    pub fn render_text(&self) -> String {
        if self.asks.is_empty() && self.bids.is_empty() {
            return "  五档: (empty)\n".into();
        }
        let mut out = String::from("  五档(只读)\n");
        for a in &self.asks {
            out.push_str(&format!(
                "    {:<4} {:>8}  {:>8}\n",
                a.label, a.price_text, a.volume_text
            ));
        }
        for b in &self.bids {
            out.push_str(&format!(
                "    {:<4} {:>8}  {:>8}\n",
                b.label, b.price_text, b.volume_text
            ));
        }
        out
    }
}
