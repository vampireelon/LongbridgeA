//! Desktop entry: gpui window + background Session polling.
//!
//! Enabled with `--features gpui` and `LONGBRIDGEA_GUI=1`.

use crate::state::AppState;
use longbridgea_core::Symbol;
use longbridgea_ui::gpui_shell::{RootShell, SharedSnapshot, ShellSnapshot};
use longbridgea_ui::pages::stock::ChartPeriod;
use longbridgea_ui::Route;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::runtime::Runtime;

/// Block on the GUI event loop. Spawns a multi-thread tokio runtime for market data.
pub fn run() -> anyhow::Result<()> {
    let mode = std::env::var("LONGBRIDGEA_MODE").unwrap_or_else(|_| "watchlist".into());
    let snapshot: SharedSnapshot = Arc::new(Mutex::new(ShellSnapshot::default()));

    let rt = Runtime::new()?;
    let snap_bg = Arc::clone(&snapshot);
    let mode_bg = mode.clone();

    rt.spawn(async move {
        if let Err(e) = data_loop(snap_bg, mode_bg).await {
            eprintln!("data loop error: {e:#}");
        }
    });

    // Keep runtime alive for the process lifetime.
    std::mem::forget(rt);

    use gpui_kit::*;

    application()
        .with_assets(assets::Assets)
        .run(move |cx| {
            init(cx);
            open_window(WindowOptions::default(), cx, {
                let snapshot = Arc::clone(&snapshot);
                move |_, cx| cx.new(|_| RootShell::new(snapshot))
            })
            .expect("Failed to open LongbridgeA window");
        });

    Ok(())
}

async fn data_loop(snapshot: SharedSnapshot, mode: String) -> anyhow::Result<()> {
    let mut app = AppState::with_defaults()?;
    let secs: u64 = std::env::var("LONGBRIDGEA_WATCH_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3600); // GUI stays up; poll until process exit

    let is_stock = matches!(
        mode.to_ascii_lowercase().as_str(),
        "stock" | "detail" | "quote"
    );

    if is_stock {
        let wire = std::env::var("LONGBRIDGEA_SYMBOL").unwrap_or_else(|_| "sh600519".into());
        let period = std::env::var("LONGBRIDGEA_PERIOD")
            .ok()
            .and_then(|s| ChartPeriod::parse(&s))
            .unwrap_or(ChartPeriod::Minute);
        let sym = Symbol::parse(&wire)?;
        app.focus_stock(sym, period).await;
        app.stock.load_all().await?;
        push_stock(&snapshot, &app).await;

        let mut rx = app.stock.hub.subscribe_ticks();
        app.stock.hub.start().await;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(secs);
        while tokio::time::Instant::now() < deadline {
            match tokio::time::timeout(Duration::from_secs(5), rx.recv()).await {
                Ok(Ok(tick)) => {
                    app.stock.on_tick(&tick).await;
                    push_stock(&snapshot, &app).await;
                }
                Ok(Err(_)) => break,
                Err(_) => {
                    // idle refresh chart occasionally
                    let _ = app.stock.refresh_chart().await;
                    push_stock(&snapshot, &app).await;
                }
            }
        }
        app.stock.hub.stop().await;
    } else {
        let symbols = default_symbols()?;
        app.focus_watchlist();
        app.watchlist.set_symbols(&symbols).await?;
        app.watchlist.refresh_once().await;
        push_watchlist(&snapshot, &app).await;

        let mut rx = app.watchlist.hub.subscribe_ticks();
        app.watchlist.hub.start().await;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(secs);
        while tokio::time::Instant::now() < deadline {
            match tokio::time::timeout(Duration::from_secs(5), rx.recv()).await {
                Ok(Ok(tick)) => {
                    app.watchlist.on_tick(&tick).await;
                    push_watchlist(&snapshot, &app).await;
                }
                Ok(Err(_)) => break,
                Err(_) => push_watchlist(&snapshot, &app).await,
            }
        }
        app.watchlist.hub.stop().await;
    }

    Ok(())
}

fn default_symbols() -> anyhow::Result<Vec<Symbol>> {
    if let Ok(raw) = std::env::var("LONGBRIDGEA_SYMBOLS") {
        let mut out = Vec::new();
        for part in raw.split(|c| c == ',' || c == ' ' || c == ';') {
            let p = part.trim();
            if !p.is_empty() {
                out.push(Symbol::parse(p)?);
            }
        }
        return Ok(out);
    }
    Ok(vec![
        Symbol::parse("sh600519")?,
        Symbol::parse("sz000001")?,
        Symbol::parse("sh000001")?,
    ])
}

async fn push_watchlist(snapshot: &SharedSnapshot, app: &AppState) {
    let text = app.render_watchlist_bound().await;
    if let Ok(mut g) = snapshot.lock() {
        g.active_route = Route::Watchlist;
        g.tab_title = Route::Watchlist.label_zh().into();
        g.main_text = text;
        g.status = format!("watchlist @ {}", chrono_like_now());
    }
}

async fn push_stock(snapshot: &SharedSnapshot, app: &AppState) {
    let text = app.render_stock_bound().await;
    if let Ok(mut g) = snapshot.lock() {
        g.active_route = Route::Analysis;
        g.tab_title = Route::Analysis.label_zh().into();
        g.main_text = text;
        g.status = format!("stock @ {}", chrono_like_now());
    }
}

fn chrono_like_now() -> String {
    // Avoid extra dep; plain monotonic-ish label.
    format!("{:?}", std::time::SystemTime::now())
}
