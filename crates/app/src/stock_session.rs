//! Headless stock-detail data-plane: `StockDetailPage` ↔ client / QuoteHub.
//!
//! Loads quote + chart series (minute / day-week-month / mkline) without GUI.

use longbridgea_core::{Adjust, KlinePeriod, Symbol};
use longbridgea_ui::pages::stock::{ChartPeriod, StockDetailPage};
use std::sync::Arc;
use std::time::Duration;
use tencent_quote::{PollConfig, QuoteHub, QuoteTick, TencentQuoteClient};
use tokio::sync::RwLock;
use tokio::time::{timeout, Instant};

/// Shared session for CLI now and gpui stock tab later.
pub struct StockDetailSession {
    pub client: TencentQuoteClient,
    pub hub: Arc<QuoteHub>,
    pub page: Arc<RwLock<StockDetailPage>>,
    pub bar_count: u32,
    pub adjust: Adjust,
}

impl StockDetailSession {
    pub fn new(client: TencentQuoteClient, poll: PollConfig) -> Self {
        let hub = Arc::new(QuoteHub::new(
            client.clone(),
            Arc::new(tencent_quote::QuoteCache::new(poll.interval)),
            poll,
        ));
        Self {
            client,
            hub,
            page: Arc::new(RwLock::new(StockDetailPage::default())),
            bar_count: 120,
            adjust: Adjust::Qfq,
        }
    }

    pub fn with_defaults(client: TencentQuoteClient) -> Self {
        Self::new(client, PollConfig::default())
    }

    /// Focus one symbol; sync hub watch set to that symbol only.
    pub async fn open(&self, symbol: Symbol, period: ChartPeriod) {
        {
            let mut page = self.page.write().await;
            *page = StockDetailPage::open(symbol.clone());
            page.set_period(period);
        }
        self.hub.unwatch_all().await;
        self.hub.watch(std::slice::from_ref(&symbol)).await;
    }

    pub async fn set_period(&self, period: ChartPeriod) {
        self.page.write().await.set_period(period);
    }

    /// Snapshot quote via hub (cache-aware).
    pub async fn refresh_quote(&self) -> anyhow::Result<()> {
        let sym = {
            let page = self.page.read().await;
            page.symbol.clone()
        };
        let Some(sym) = sym else {
            return Ok(());
        };
        let q = self.hub.get_quote(&sym).await?;
        self.page.write().await.apply_quote(q);
        Ok(())
    }

    /// Load chart body for the current period.
    pub async fn refresh_chart(&self) -> anyhow::Result<()> {
        let (sym, period) = {
            let page = self.page.read().await;
            (page.symbol.clone(), page.period)
        };
        let Some(sym) = sym else {
            return Ok(());
        };

        match period {
            ChartPeriod::Minute => match self.client.fetch_minute(&sym).await {
                Ok(series) => {
                    self.page.write().await.apply_minute(series);
                    Ok(())
                }
                Err(e) => {
                    self.page.write().await.set_chart_error(e.to_string());
                    Err(e.into())
                }
            },
            ChartPeriod::Day | ChartPeriod::Week | ChartPeriod::Month => {
                let kp = match period {
                    ChartPeriod::Day => KlinePeriod::Day,
                    ChartPeriod::Week => KlinePeriod::Week,
                    ChartPeriod::Month => KlinePeriod::Month,
                    _ => unreachable!(),
                };
                match self
                    .client
                    .fetch_kline(&sym, kp, self.adjust, self.bar_count)
                    .await
                {
                    Ok(bars) => {
                        self.page.write().await.apply_bars(bars);
                        Ok(())
                    }
                    Err(e) => {
                        self.page.write().await.set_chart_error(e.to_string());
                        Err(e.into())
                    }
                }
            }
            ChartPeriod::M1
            | ChartPeriod::M5
            | ChartPeriod::M15
            | ChartPeriod::M30
            | ChartPeriod::M60 => {
                let interval = period.as_minute_interval().expect("mkline period");
                match self
                    .client
                    .fetch_mkline(&sym, interval, self.bar_count)
                    .await
                {
                    Ok(bars) => {
                        self.page.write().await.apply_bars(bars);
                        Ok(())
                    }
                    Err(e) => {
                        self.page.write().await.set_chart_error(e.to_string());
                        Err(e.into())
                    }
                }
            }
        }
    }

    /// Quote + chart in one shot (chart errors are stored on page, not always fatal).
    pub async fn load_all(&self) -> anyhow::Result<()> {
        self.refresh_quote().await?;
        let _ = self.refresh_chart().await;
        Ok(())
    }

    pub async fn on_tick(&self, tick: &QuoteTick) {
        if let Some(q) = tick.quotes.first() {
            self.page.write().await.apply_quote(q.clone());
        }
    }

    pub async fn summary(&self) -> String {
        self.page.read().await.render_summary()
    }

    /// Poll quote for `duration`; reload chart once at start (and optionally every `chart_every`).
    pub async fn run_headless(
        &self,
        duration: Duration,
        chart_every: Option<Duration>,
        print: bool,
    ) -> anyhow::Result<u32> {
        // Ensure hub watches current symbol
        if let Some(sym) = self.page.read().await.symbol.clone() {
            self.hub.unwatch_all().await;
            self.hub.watch(&[sym]).await;
        }

        self.load_all().await?;
        if print {
            println!("--- stock load#0 ---");
            print!("{}", self.summary().await);
        }

        let mut rx = self.hub.subscribe_ticks();
        self.hub.start().await;

        let deadline = Instant::now() + duration;
        let mut n = 0u32;
        let mut last_chart = Instant::now();

        while Instant::now() < deadline {
            let left = deadline.saturating_duration_since(Instant::now());
            match timeout(left, rx.recv()).await {
                Ok(Ok(tick)) => {
                    n += 1;
                    self.on_tick(&tick).await;
                    if let Some(every) = chart_every {
                        if last_chart.elapsed() >= every {
                            let _ = self.refresh_chart().await;
                            last_chart = Instant::now();
                        }
                    }
                    if print {
                        println!(
                            "--- stock tick#{n} quotes={} errors={:?} ---",
                            tick.quotes.len(),
                            tick.errors
                        );
                        print!("{}", self.summary().await);
                    }
                }
                Ok(Err(_)) => break,
                Err(_) => break,
            }
        }

        self.hub.stop().await;
        Ok(n)
    }
}
