//! Application state: shell + sessions + component binding helpers.
//!
//! ```text
//! QuoteHub / Client  --(session)-->  Page model  --(bind)-->  Component view
//!                                      ↑
//!                                   AppState
//! ```

use crate::stock_session::StockDetailSession;
use crate::watchlist_session::WatchlistSession;
use longbridgea_core::Symbol;
use longbridgea_ui::components::{
    ChartPanelView, DepthBookView, QuoteHeaderView, WatchlistTableView,
};
use longbridgea_ui::pages::stock::ChartPeriod;
use longbridgea_ui::{AppShell, Route};
use std::sync::Arc;
use std::time::Duration;
use tencent_quote::{ClientConfig, PollConfig, TencentQuoteClient};

/// Owns chrome state and data sessions. UI components bind from session pages.
pub struct AppState {
    pub shell: AppShell,
    pub watchlist: WatchlistSession,
    pub stock: StockDetailSession,
}

impl AppState {
    pub fn new(client: TencentQuoteClient, poll: PollConfig) -> anyhow::Result<Self> {
        // Separate clients so watchlist + stock can poll independently.
        let client_w = TencentQuoteClient::new(ClientConfig {
            quote_base: client.config().quote_base.clone(),
            ifzq_base: client.config().ifzq_base.clone(),
            mkline_base: client.config().mkline_base.clone(),
            smartbox_base: client.config().smartbox_base.clone(),
            timeout_ms: client.config().timeout_ms,
            user_agent: client.config().user_agent.clone(),
            referer: client.config().referer.clone(),
        })?;
        let client_s = TencentQuoteClient::new(ClientConfig {
            quote_base: client.config().quote_base.clone(),
            ifzq_base: client.config().ifzq_base.clone(),
            mkline_base: client.config().mkline_base.clone(),
            smartbox_base: client.config().smartbox_base.clone(),
            timeout_ms: client.config().timeout_ms,
            user_agent: client.config().user_agent.clone(),
            referer: client.config().referer.clone(),
        })?;

        Ok(Self {
            shell: AppShell::default(),
            watchlist: WatchlistSession::new(client_w, poll.clone()),
            stock: StockDetailSession::new(client_s, poll),
        })
    }

    pub fn with_defaults() -> anyhow::Result<Self> {
        let client = TencentQuoteClient::new(ClientConfig::default())?;
        Self::new(
            client,
            PollConfig {
                interval: Duration::from_secs(3),
                broadcast_capacity: 32,
            },
        )
    }

    // ----- navigation / focus -----

    pub fn focus_watchlist(&mut self) {
        self.shell.navigate(Route::Watchlist);
    }

    pub async fn focus_stock(&mut self, symbol: Symbol, period: ChartPeriod) {
        let name = {
            // Prefer name already on watchlist rows if present
            let page = self.watchlist.page.read().await;
            page.rows
                .iter()
                .find(|q| q.symbol == symbol)
                .map(|q| q.name.clone())
                .unwrap_or_default()
        };
        self.shell.open_stock(&symbol, &name);
        self.shell.navigate(Route::Analysis);
        self.stock.open(symbol, period).await;
    }

    // ----- bind: session page → component -----

    pub async fn bind_watchlist_table(&self) -> WatchlistTableView {
        let page = self.watchlist.page.read().await;
        WatchlistTableView::bind(&page)
    }

    pub async fn bind_quote_header(&self) -> Option<QuoteHeaderView> {
        let page = self.stock.page.read().await;
        QuoteHeaderView::bind_page(&page)
    }

    pub async fn bind_depth_book(&self) -> DepthBookView {
        let page = self.stock.page.read().await;
        DepthBookView::bind_page(&page)
    }

    pub async fn bind_chart_panel(&self) -> ChartPanelView {
        let page = self.stock.page.read().await;
        ChartPanelView::bind(&page)
    }

    /// Full stock detail panel text (header + depth + chart).
    pub async fn render_stock_bound(&self) -> String {
        let mut out = String::new();
        if let Some(h) = self.bind_quote_header().await {
            out.push_str(&h.render_text());
        } else {
            out.push_str("(no quote)\n");
        }
        out.push_str(&self.bind_depth_book().await.render_text());
        out.push_str(&self.bind_chart_panel().await.render_text());
        out
    }

    pub async fn render_watchlist_bound(&self) -> String {
        self.bind_watchlist_table().await.render_text()
    }

    /// Shared page handle for future gpui (same Arc the session mutates).
    pub fn watchlist_page(&self) -> Arc<tokio::sync::RwLock<longbridgea_ui::pages::WatchlistPage>> {
        Arc::clone(&self.watchlist.page)
    }

    pub fn stock_page(&self) -> Arc<tokio::sync::RwLock<longbridgea_ui::pages::StockDetailPage>> {
        Arc::clone(&self.stock.page)
    }
}
