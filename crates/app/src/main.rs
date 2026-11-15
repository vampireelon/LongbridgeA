//! LongbridgeA application entry.
//!
//! Sessions write page models; components **bind** from those models via `AppState`.
//!
//! ```text
//! LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=watchlist cargo run -p longbridgea-app
//! LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=stock LONGBRIDGEA_SYMBOL=sh600519 \
//!   LONGBRIDGEA_PERIOD=minute cargo run -p longbridgea-app
//! ```

mod state;
mod stock_session;
mod watchlist_session;

use longbridgea_core::Symbol;
use longbridgea_ui::pages::stock::ChartPeriod;
use state::AppState;
use std::time::Duration;
use tokio::time::{timeout, Instant};

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

    if std::env::var("LONGBRIDGEA_LIVE").is_err() {
        println!("Tip: LONGBRIDGEA_LIVE=1 LONGBRIDGEA_MODE=watchlist|stock cargo run -p longbridgea-app");
        return Ok(());
    }

    let mut app = AppState::with_defaults()?;
    let mode = std::env::var("LONGBRIDGEA_MODE").unwrap_or_else(|_| "watchlist".into());
    let secs: u64 = std::env::var("LONGBRIDGEA_WATCH_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    match mode.to_ascii_lowercase().as_str() {
        "stock" | "detail" | "quote" => run_stock_bound(&mut app, secs).await?,
        _ => run_watchlist_bound(&mut app, secs).await?,
    }

    Ok(())
}

async fn run_watchlist_bound(app: &mut AppState, secs: u64) -> anyhow::Result<()> {
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
        "AppState→WatchlistTableView | {} symbols | {}s",
        symbols.len(),
        secs
    );
    app.focus_watchlist();
    app.watchlist.set_symbols(&symbols).await?;
    println!("  shell {}", app.shell.debug_layout());

    // First paint through component bind
    app.watchlist.refresh_once().await;
    print!("{}", app.render_watchlist_bound().await);

    let mut rx = app.watchlist.hub.subscribe_ticks();
    app.watchlist.hub.start().await;
    let deadline = Instant::now() + Duration::from_secs(secs);
    let mut n = 0u32;
    while Instant::now() < deadline {
        let left = deadline.saturating_duration_since(Instant::now());
        match timeout(left, rx.recv()).await {
            Ok(Ok(tick)) => {
                n += 1;
                app.watchlist.on_tick(&tick).await;
                println!("--- bind tick#{n} ---");
                print!("{}", app.render_watchlist_bound().await);
            }
            Ok(Err(_)) | Err(_) => break,
        }
    }
    app.watchlist.hub.stop().await;
    println!("Done watchlist ticks={n}");
    Ok(())
}

async fn run_stock_bound(app: &mut AppState, secs: u64) -> anyhow::Result<()> {
    let wire = std::env::var("LONGBRIDGEA_SYMBOL").unwrap_or_else(|_| "sh600519".into());
    let period = std::env::var("LONGBRIDGEA_PERIOD")
        .ok()
        .and_then(|s| ChartPeriod::parse(&s))
        .unwrap_or(ChartPeriod::Minute);
    let sym = Symbol::parse(&wire)?;

    println!(
        "AppState→QuoteHeader+Depth+Chart | {} {} | {}s",
        sym.tencent_wire(),
        period.label_zh(),
        secs
    );

    app.focus_stock(sym, period).await;
    println!("  shell {}", app.shell.debug_layout());

    app.stock.load_all().await?;
    println!("--- bind load#0 ---");
    print!("{}", app.render_stock_bound().await);

    let mut rx = app.stock.hub.subscribe_ticks();
    app.stock.hub.start().await;
    let deadline = Instant::now() + Duration::from_secs(secs);
    let mut n = 0u32;
    let mut last_chart = Instant::now();
    while Instant::now() < deadline {
        let left = deadline.saturating_duration_since(Instant::now());
        match timeout(left, rx.recv()).await {
            Ok(Ok(tick)) => {
                n += 1;
                app.stock.on_tick(&tick).await;
                if last_chart.elapsed() >= Duration::from_secs(30) {
                    let _ = app.stock.refresh_chart().await;
                    last_chart = Instant::now();
                }
                println!("--- bind tick#{n} ---");
                print!("{}", app.render_stock_bound().await);
            }
            Ok(Err(_)) | Err(_) => break,
        }
    }
    app.stock.hub.stop().await;
    println!("Done stock ticks={n}");
    Ok(())
}
