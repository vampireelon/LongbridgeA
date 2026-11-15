//! LongbridgeA application entry.
//!
//! GUI bootstrap (gpui-kit) lands in Phase 2.
//! Optional live smoke: `LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app`

use longbridgea_core::{Adjust, KlinePeriod, MinuteInterval, Symbol};
use std::sync::Arc;
use std::time::Duration;
use tencent_quote::{ClientConfig, PollConfig, QuoteHub, TencentQuoteClient};
use tokio::time::timeout;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("LongbridgeA {}", env!("CARGO_PKG_VERSION"));
    println!("Scope: A-share selection & analysis (no trading).");
    println!("UI: {}", longbridgea_ui::pages::placeholder_watchlist());

    let client = TencentQuoteClient::new(ClientConfig::default())?;
    println!("Quote base:    {}", client.config().quote_base);
    println!("Ifzq base:     {}", client.config().ifzq_base);
    println!("Mkline base:   {}", client.config().mkline_base);
    println!("Smartbox base: {}", client.config().smartbox_base);

    if std::env::var("LONGBRIDGEA_LIVE").is_ok() {
        // --- Hub: cache + one poll cycle ---
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
        let tick = hub.poll_once().await;
        println!(
            "HUB poll_once requested={} quotes={} errors={:?}",
            tick.requested.len(),
            tick.quotes.len(),
            tick.errors
        );
        for q in &tick.quotes {
            println!(
                "  {} {} {} ({:+.2}%)",
                q.symbol.tencent_wire(),
                q.name,
                q.price,
                q.change_pct
            );
        }
        // Cache hit path
        if let Ok(q) = hub.get_quote(&watch[0]).await {
            println!(
                "CACHE hit/fetch {} price={}",
                q.symbol.tencent_wire(),
                q.price
            );
        }

        // Background poll: wait for one tick then stop
        let mut rx = hub.subscribe_ticks();
        hub.start().await;
        match timeout(Duration::from_secs(8), rx.recv()).await {
            Ok(Ok(t)) => println!(
                "HUB bg tick quotes={} requested={:?}",
                t.quotes.len(),
                t.requested
            ),
            Ok(Err(e)) => eprintln!("bg recv closed: {e}"),
            Err(_) => eprintln!("bg tick timeout (market closed / network?)"),
        }
        hub.stop().await;

        match client.fetch_search("茅台").await {
            Ok(hits) => {
                println!("SEARCH '茅台' hits={}", hits.len());
                for h in hits.iter().take(5) {
                    let a = if h.is_a_share() { "A" } else { "-" };
                    println!(
                        "  [{a}] {} {} {} ({})",
                        h.market_tag, h.code, h.name, h.pinyin
                    );
                }
            }
            Err(e) => eprintln!("search failed: {e}"),
        }

        let sym = Symbol::parse("sh600519")?;
        match client.fetch_minute(&sym).await {
            Ok(m) => println!(
                "MINUTE points={} last={:?}",
                m.points.len(),
                m.points.last().map(|p| (&p.time, p.price))
            ),
            Err(e) => eprintln!("minute failed: {e}"),
        }

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
