//! LongbridgeA application entry.
//!
//! Phase 2: UI shell state is available; gpui window is feature-gated later.
//! Optional live smoke: `LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app`

use longbridgea_core::{Adjust, KlinePeriod, MinuteInterval, Symbol};
use longbridgea_ui::pages::{ScreenerPage, WatchlistPage};
use longbridgea_ui::widgets::PriceChangeView;
use longbridgea_ui::{AppShell, Route};
use std::sync::Arc;
use std::time::Duration;
use tencent_quote::{ClientConfig, PollConfig, QuoteHub, TencentQuoteClient};
use tokio::time::timeout;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("LongbridgeA {}", env!("CARGO_PKG_VERSION"));
    println!("Scope: A-share selection & analysis (no trading).");
    println!("{}", longbridgea_ui::skeleton_banner());

    let mut shell = AppShell::default();
    println!("  {}", shell.debug_layout());
    shell.navigate(Route::Screener);
    println!("  after → screener: {}", shell.debug_layout());
    shell.navigate(Route::Watchlist);

    let pc = PriceChangeView::with_default_dark_cn(1.25);
    println!("  sample PriceChange: {} ({})", pc.text, pc.color_hex);

    let client = TencentQuoteClient::new(ClientConfig::default())?;
    println!("Quote base: {}", client.config().quote_base);

    if std::env::var("LONGBRIDGEA_LIVE").is_ok() {
        let hub = QuoteHub::new(
            client.clone(),
            Arc::new(tencent_quote::QuoteCache::new(Duration::from_secs(5))),
            PollConfig {
                interval: Duration::from_secs(3),
                broadcast_capacity: 16,
            },
        );
        let watch = [
            Symbol::parse("sh600519")?,
            Symbol::parse("sz000001")?,
        ];
        hub.watch(&watch).await;

        let mut watchlist = WatchlistPage::default();
        for s in &watch {
            watchlist.add_symbol(s);
        }

        let tick = hub.poll_once().await;
        watchlist.apply_quotes(tick.quotes.clone());
        println!(
            "WATCHLIST rows={} (group symbols={:?})",
            watchlist.rows.len(),
            watchlist.groups[0].symbols
        );
        for q in watchlist.rows.iter().take(5) {
            let ch = PriceChangeView::with_default_dark_cn(q.change_pct);
            println!(
                "  {} {} {} {}",
                q.symbol.tencent_wire(),
                q.name,
                q.price,
                ch.text
            );
        }

        // Open detail tab in shell
        if let Some(q) = watchlist.rows.first() {
            shell.open_stock(&q.symbol, &q.name);
            println!("  {}", shell.debug_layout());
        }

        let mut screener = ScreenerPage::default();
        screener.criteria.change_pct.min = Some(0.0);
        screener.apply_local_filter(&tick.quotes);
        println!(
            "SCREENER status={:?} hits={}",
            screener.status,
            screener.results.len()
        );

        let mut rx = hub.subscribe_ticks();
        hub.start().await;
        match timeout(Duration::from_secs(8), rx.recv()).await {
            Ok(Ok(t)) => println!("HUB bg tick quotes={}", t.quotes.len()),
            Ok(Err(e)) => eprintln!("bg recv closed: {e}"),
            Err(_) => eprintln!("bg tick timeout"),
        }
        hub.stop().await;

        let sym = Symbol::parse("sh600519")?;
        match client
            .fetch_kline(&sym, KlinePeriod::Day, Adjust::Qfq, 3)
            .await
        {
            Ok(bars) => println!("KLINE day bars={}", bars.len()),
            Err(e) => eprintln!("kline failed: {e}"),
        }
        match client.fetch_mkline(&sym, MinuteInterval::M5, 5).await {
            Ok(bars) => println!("MKLINE m5 bars={}", bars.len()),
            Err(e) => eprintln!("mkline failed: {e}"),
        }
    } else {
        println!("Tip: LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app");
    }

    Ok(())
}
