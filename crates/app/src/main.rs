//! LongbridgeA application entry.
//!
//! Headless watchlist data-flow (no GUI window):
//!   LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app
//! Optional:
//!   LONGBRIDGEA_WATCH_SECS=12  — how long to poll (default 10)
//!   LONGBRIDGEA_SYMBOLS=sh600519,sz000001,sh000001

mod watchlist_session;

use longbridgea_core::Symbol;
use longbridgea_ui::{AppShell, Route};
use std::time::Duration;
use tencent_quote::{ClientConfig, PollConfig, TencentQuoteClient};
use watchlist_session::WatchlistSession;

fn parse_symbols(raw: &str) -> anyhow::Result<Vec<Symbol>> {
    let mut out = Vec::new();
    for part in raw.split(|c| c == ',' || c == ' ' || c == ';') {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        out.push(Symbol::parse(p)?);
    }
    Ok(out)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("LongbridgeA {}", env!("CARGO_PKG_VERSION"));
    println!("Scope: A-share selection & analysis (no trading).");
    println!("{}", longbridgea_ui::skeleton_banner());

    let mut shell = AppShell::default();
    shell.navigate(Route::Watchlist);
    println!("  {}", shell.debug_layout());

    let client = TencentQuoteClient::new(ClientConfig::default())?;

    if std::env::var("LONGBRIDGEA_LIVE").is_ok() {
        let secs: u64 = std::env::var("LONGBRIDGEA_WATCH_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        let symbols = if let Ok(raw) = std::env::var("LONGBRIDGEA_SYMBOLS") {
            parse_symbols(&raw)?
        } else {
            vec![
                Symbol::parse("sh600519")?,
                Symbol::parse("sz000001")?,
                Symbol::parse("sh000001")?,
            ]
        };

        println!(
            "WatchlistSession headless: {} symbols, poll {}s, interval 3s",
            symbols.len(),
            secs
        );
        for s in &symbols {
            println!("  · {}", s.tencent_wire());
        }

        let session = WatchlistSession::new(
            client,
            PollConfig {
                interval: Duration::from_secs(3),
                broadcast_capacity: 32,
            },
        );
        session.set_symbols(&symbols).await?;

        // Mirror first symbol into shell tab (state only)
        if let Some(s) = symbols.first() {
            shell.open_stock(s, "");
        }

        let ticks = session
            .run_headless(Duration::from_secs(secs), true)
            .await?;
        println!(
            "Done. bg_ticks={} rows={} shell={}",
            ticks,
            session.row_count().await,
            shell.debug_layout()
        );
    } else {
        println!("Tip: LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app");
        println!("      LONGBRIDGEA_WATCH_SECS=12 LONGBRIDGEA_SYMBOLS=sh600519,sz000001");
    }

    Ok(())
}
