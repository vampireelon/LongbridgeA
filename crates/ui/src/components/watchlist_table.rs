//! Watchlist table component ← `WatchlistPage`.

use crate::pages::WatchlistPage;
use crate::widgets::PriceChangeView;
use longbridgea_core::Quote;

#[derive(Debug, Clone)]
pub struct WatchlistRowView {
    pub wire: String,
    pub name: String,
    pub price_text: String,
    pub change_pct_text: String,
    pub change_pct_color: &'static str,
    pub change_text: String,
    pub amount_text: String,
}

impl WatchlistRowView {
    pub fn from_quote(q: &Quote) -> Self {
        let pct = PriceChangeView::with_default_dark_cn(q.change_pct);
        let ch = PriceChangeView::from_amount(
            q.change,
            &crate::theme::ColorTheme::resolve(
                crate::theme::ThemeMode::Dark,
                crate::theme::ChangeColorScheme::CnRedUpGreenDown,
            ),
        );
        Self {
            wire: q.symbol.tencent_wire(),
            name: q.name.clone(),
            price_text: format!("{:.2}", q.price),
            change_pct_text: pct.text,
            change_pct_color: pct.color_hex,
            change_text: ch.text,
            amount_text: format!("{:.0}", q.amount),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WatchlistTableView {
    pub group_name: String,
    pub symbol_count: usize,
    pub rows: Vec<WatchlistRowView>,
}

impl WatchlistTableView {
    /// Bind component to current page model snapshot.
    pub fn bind(page: &WatchlistPage) -> Self {
        let group_name = page
            .active_group()
            .map(|g| g.name.clone())
            .unwrap_or_else(|| "-".into());
        let symbol_count = page.watched_wires().len();
        let rows = page.rows.iter().map(WatchlistRowView::from_quote).collect();
        Self {
            group_name,
            symbol_count,
            rows,
        }
    }

    pub fn render_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[自选·{}] symbols={} rows={}\n",
            self.group_name, self.symbol_count, self.rows.len()
        ));
        out.push_str(&format!(
            "{:<10} {:<10} {:>10} {:>9} {:>10} {:>12}\n",
            "代码", "名称", "最新", "涨跌幅", "涨跌额", "成交额(万)"
        ));
        out.push_str(&format!("{}\n", "-".repeat(66)));
        for r in &self.rows {
            out.push_str(&format!(
                "{:<10} {:<10} {:>10} {:>9} {:>10} {:>12}\n",
                r.wire,
                truncate(&r.name, 10),
                r.price_text,
                r.change_pct_text,
                r.change_text,
                r.amount_text
            ));
        }
        if self.rows.is_empty() {
            out.push_str("(empty — add symbols / wait for poll)\n");
        }
        out
    }
}

fn truncate(s: &str, max_chars: usize) -> String {
    s.chars().take(max_chars).collect()
}
