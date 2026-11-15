//! LongbridgeA application entry.
//!
//! GUI bootstrap (gpui-kit) lands in Phase 2.
//! Optional live smoke: `LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app`

use longbridgea_core::{Adjust, KlinePeriod, MinuteInterval, Symbol};
use tencent_quote::{ClientConfig, TencentQuoteClient};

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
        match client.fetch_search("茅台").await {
            Ok(hits) => {
                println!("SEARCH '茅台' hits={}", hits.len());
                for h in hits.iter().take(8) {
                    let a = if h.is_a_share() { "A" } else { "-" };
                    println!(
                        "  [{a}] {} {} {} ({}) {}",
                        h.market_tag, h.code, h.name, h.pinyin, h.kind
                    );
                }
            }
            Err(e) => eprintln!("search failed: {e}"),
        }

        let sym = Symbol::parse("sh600519")?;

        match client.fetch_quotes(&[sym.clone()]).await {
            Ok(quotes) => {
                for q in quotes {
                    println!(
                        "QUOTE {} {} price={} pct={}%",
                        q.symbol.tencent_wire(),
                        q.name,
                        q.price,
                        q.change_pct
                    );
                }
            }
            Err(e) => eprintln!("quote failed: {e}"),
        }

        match client.fetch_minute(&sym).await {
            Ok(m) => println!(
                "MINUTE points={} date={:?} last={:?}",
                m.points.len(),
                m.date,
                m.points.last().map(|p| (&p.time, p.price))
            ),
            Err(e) => eprintln!("minute failed: {e}"),
        }

        match client
            .fetch_kline(&sym, KlinePeriod::Day, Adjust::Qfq, 5)
            .await
        {
            Ok(bars) => {
                println!("KLINE day bars={}", bars.len());
                for b in bars.iter().rev().take(3) {
                    println!(
                        "  {} O={} C={} H={} L={} V={}",
                        b.date, b.open, b.close, b.high, b.low, b.volume
                    );
                }
            }
            Err(e) => eprintln!("kline failed: {e}"),
        }

        match client.fetch_mkline(&sym, MinuteInterval::M5, 10).await {
            Ok(bars) => {
                println!("MKLINE m5 bars={}", bars.len());
                for b in bars.iter().rev().take(3) {
                    println!(
                        "  {} O={} C={} H={} L={} V={}",
                        b.date, b.open, b.close, b.high, b.low, b.volume
                    );
                }
            }
            Err(e) => eprintln!("mkline failed: {e}"),
        }
    } else {
        println!("Tip: LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app");
    }

    Ok(())
}
