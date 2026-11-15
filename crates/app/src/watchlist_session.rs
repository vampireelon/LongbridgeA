//! Headless watchlist data-plane: `WatchlistPage` ↔ `QuoteHub`.
//!
//! No GUI. Drives poll → apply_quotes → optional table print.

use longbridgea_core::Symbol;
use longbridgea_ui::pages::WatchlistPage;
use std::sync::Arc;
use std::time::Duration;
use tencent_quote::{PollConfig, QuoteHub, QuoteTick, TencentQuoteClient};
use tokio::sync::RwLock;
use tokio::time::{timeout, Instant};

/// Shared session used by CLI now and by gpui views later.
pub struct WatchlistSession {
    pub hub: Arc<QuoteHub>,
    pub page: Arc<RwLock<WatchlistPage>>,
}

impl WatchlistSession {
    pub fn new(client: TencentQuoteClient, poll: PollConfig) -> Self {
        let hub = Arc::new(QuoteHub::new(
            client,
            Arc::new(tencent_quote::QuoteCache::new(poll.interval)),
            poll,
        ));
        Self {
            hub,
            page: Arc::new(RwLock::new(WatchlistPage::default())),
        }
    }

    pub fn with_defaults(client: TencentQuoteClient) -> Self {
        Self::new(client, PollConfig::default())
    }

    /// Replace active-group symbols and sync hub watch set.
    pub async fn set_symbols(&self, symbols: &[Symbol]) -> anyhow::Result<()> {
        {
            let mut page = self.page.write().await;
            if let Some(g) = page.active_group_mut() {
                g.symbols.clear();
            }
            for s in symbols {
                page.add_symbol(s);
            }
        }
        self.sync_hub_watch().await;
        Ok(())
    }

    pub async fn add_symbol(&self, symbol: &Symbol) {
        self.page.write().await.add_symbol(symbol);
        self.hub.watch(std::slice::from_ref(symbol)).await;
    }

    /// Push page active-group wires into hub.
    pub async fn sync_hub_watch(&self) {
        let symbols = self.page.read().await.watched_symbols();
        self.hub.unwatch_all().await;
        if !symbols.is_empty() {
            self.hub.watch(&symbols).await;
        }
    }

    /// One poll cycle → update page rows.
    pub async fn refresh_once(&self) -> QuoteTick {
        let tick = self.hub.poll_once().await;
        self.page.write().await.apply_quotes(tick.quotes.clone());
        tick
    }

    /// Apply an external tick (from `subscribe_ticks`).
    pub async fn on_tick(&self, tick: &QuoteTick) {
        self.page.write().await.apply_quotes(tick.quotes.clone());
    }

    pub async fn table(&self) -> String {
        self.page.read().await.render_table()
    }

    pub async fn row_count(&self) -> usize {
        self.page.read().await.rows.len()
    }

    /// Background poll for `duration`, printing a table each tick.
    /// Returns number of ticks received.
    pub async fn run_headless(&self, duration: Duration, print_tables: bool) -> anyhow::Result<u32> {
        self.sync_hub_watch().await;

        // Immediate first paint
        let first = self.refresh_once().await;
        if print_tables {
            println!(
                "--- tick#0 requested={} errors={:?} ---",
                first.requested.len(),
                first.errors
            );
            print!("{}", self.table().await);
        }

        let mut rx = self.hub.subscribe_ticks();
        self.hub.start().await;

        let deadline = Instant::now() + duration;
        let mut n = 0u32;
        while Instant::now() < deadline {
            let left = deadline.saturating_duration_since(Instant::now());
            match timeout(left, rx.recv()).await {
                Ok(Ok(tick)) => {
                    n += 1;
                    self.on_tick(&tick).await;
                    if print_tables {
                        println!(
                            "--- tick#{n} quotes={} errors={:?} ---",
                            tick.quotes.len(),
                            tick.errors
                        );
                        print!("{}", self.table().await);
                    }
                }
                Ok(Err(_)) => break,
                Err(_) => break, // duration elapsed
            }
        }

        self.hub.stop().await;
        Ok(n)
    }
}
