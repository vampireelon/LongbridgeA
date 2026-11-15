//! LongbridgeA application entry.
//!
//! Headless data-flows (no GUI):
//!
//! ```text
//! # 自选表
//! LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app
//! LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=watchlist LONGBRIDGEA_SYMBOLS=sh600519,sz000001
//!
//! # 个股详情（报价轮询 + 分时/K线）
//! LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=stock \
//!   LONGBRIDGEA_SYMBOL=sh600519 LONGBRIDGEA_PERIOD=minute
//! # period: minute|day|week|month|m5|m15|m30|m60
//! ```

mod stock_session;
mod watchlist_session;

use longbridgea_core::Symbol;
use longbridgea_ui::pages::stock::ChartPeriod;
use longbridgea_ui::{AppShell, Route};
use std::time::Duration;
use stock_session::StockDetailSession;
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
    let client = TencentQuoteClient::new(ClientConfig::default())?;

    if std::env::var("LONGBRIDGEA_LIVE").is_err() {
        println!("Tip: LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=watchlist|stock cargo run -p longbridgea-app");
        return Ok(());
    }

    let mode = std::env::var("LONGBRIDGEA_MODE").unwrap_or_else(|_| "watchlist".into());
    let secs: u64 = std::env::var("LONGBRIDGEA_WATCH_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    match mode.to_ascii_lowercase().as_str() {
        "stock" | "detail" | "quote" => {
            let wire = std::env::var("LONGBRIDGEA_SYMBOL").unwrap_or_else(|_| "sh600519".into());
            let period = std::env::var("LONGBRIDGEA_PERIOD")
                .ok()
                .and_then(|s| ChartPeriod::parse(&s))
                .unwrap_or(ChartPeriod::Minute);
            let sym = Symbol::parse(&wire)?;

            println!(
                "StockDetailSession headless: {} period={} poll {}s",
                sym.tencent_wire(),
                period.label_zh(),
                secs
            );

            shell.open_stock(&sym, "");
            shell.navigate(Route::Analysis);
            println!("  {}", shell.debug_layout());

            let session = StockDetailSession::new(
                client,
                PollConfig {
                    interval: Duration::from_secs(3),
                    broadcast_capacity: 32,
                },
            );
            session.open(sym, period).await;

            // Refresh chart every 30s while quotes poll every 3s
            let ticks = session
                .run_headless(
                    Duration::from_secs(secs),
                    Some(Duration::from_secs(30)),
                    true,
                )
                .await?;
            println!("Done. bg_ticks={} shell={}", ticks, shell.debug_layout());
        }
        _ => {
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
                "WatchlistSession headless: {} symbols, poll {}s",
                symbols.len(),
                secs
            );
            for s in &symbols {
                println!("  · {}", s.tencent_wire());
            }

            shell.navigate(Route::Watchlist);
            if let Some(s) = symbols.first() {
                shell.open_stock(s, "");
            }
            println!("  {}", shell.debug_layout());

            let session = WatchlistSession::new(
                client,
                PollConfig {
                    interval: Duration::from_secs(3),
                    broadcast_capacity: 32,
                },
            );
            session.set_symbols(&symbols).await?;
            let ticks = session
                .run_headless(Duration::from_secs(secs), true)
                .await?;
            println!(
                "Done. bg_ticks={} rows={} shell={}",
                ticks,
                session.row_count().await,
                shell.debug_layout()
            );
        }
    }

    Ok(())
}
