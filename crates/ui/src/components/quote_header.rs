//! Quote header component ← `StockDetailPage` / `Quote`.

use crate::pages::StockDetailPage;
use crate::widgets::PriceChangeView;
use longbridgea_core::Quote;

#[derive(Debug, Clone)]
pub struct QuoteHeaderView {
    pub wire: String,
    pub name: String,
    pub price_text: String,
    pub change_pct_text: String,
    pub change_pct_color: &'static str,
    pub change_text: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub prev_close: String,
    pub volume: String,
    pub amount: String,
    pub turnover: String,
    pub pe: String,
    pub pb: String,
    pub total_mv: String,
}

impl QuoteHeaderView {
    pub fn from_quote(q: &Quote) -> Self {
        let pct = PriceChangeView::with_default_dark_cn(q.change_pct);
        let theme = crate::theme::ColorTheme::resolve(
            crate::theme::ThemeMode::Dark,
            crate::theme::ChangeColorScheme::CnRedUpGreenDown,
        );
        let ch = PriceChangeView::from_amount(q.change, &theme);
        Self {
            wire: q.symbol.tencent_wire(),
            name: q.name.clone(),
            price_text: format!("{:.2}", q.price),
            change_pct_text: pct.text,
            change_pct_color: pct.color_hex,
            change_text: ch.text,
            open: format!("{:.2}", q.open),
            high: format!("{:.2}", q.high),
            low: format!("{:.2}", q.low),
            prev_close: format!("{:.2}", q.prev_close),
            volume: format!("{:.0}", q.volume),
            amount: format!("{:.0}", q.amount),
            turnover: opt(q.turnover_rate),
            pe: opt(q.pe),
            pb: opt(q.pb),
            total_mv: opt(q.total_mv),
        }
    }

    pub fn bind_page(page: &StockDetailPage) -> Option<Self> {
        page.quote.as_ref().map(Self::from_quote)
    }

    pub fn render_text(&self) -> String {
        format!(
            "{}  {}  {}  {} ({})\n  开 {}  高 {}  低 {}  昨收 {}\n  量 {}手  额 {}万  换手 {}  PE {}  PB {}  市值 {}亿\n",
            self.wire,
            self.name,
            self.price_text,
            self.change_pct_text,
            self.change_text,
            self.open,
            self.high,
            self.low,
            self.prev_close,
            self.volume,
            self.amount,
            self.turnover,
            self.pe,
            self.pb,
            self.total_mv
        )
    }
}

fn opt(v: Option<f64>) -> String {
    v.map(|x| format!("{x:.2}")).unwrap_or_else(|| "-".into())
}
