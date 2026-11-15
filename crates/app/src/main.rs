//! LongbridgeA application entry.
//!
//! GUI bootstrap (gpui-kit) lands in Phase 2.
//! This binary can optionally smoke-test the quote client.

use longbridgea_core::Symbol;
use tencent_quote::{ClientConfig, TencentQuoteClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("LongbridgeA {}", env!("CARGO_PKG_VERSION"));
    println!("Scope: A-share selection & analysis (no trading).");
    println!("UI: {}", longbridgea_ui::pages::placeholder_watchlist());

    let client = TencentQuoteClient::new(ClientConfig::default())?;
    println!("Quote client base: {}", client.config().quote_base);

    // Optional live smoke test: set LONGBRIDGEA_LIVE=1 to hit the network.
    if std::env::var("LONGBRIDGEA_LIVE").is_ok() {
        let sym = Symbol::parse("sh600519")?;
        match client.fetch_quotes(&[sym]).await {
            Ok(quotes) => {
                for q in quotes {
                    println!(
                        "LIVE {} {} price={} change_pct={}%",
                        q.symbol.tencent_wire(),
                        q.name,
                        q.price,
                        q.change_pct
                    );
                }
            }
            Err(e) => eprintln!("live quote failed: {e}"),
        }
    } else {
        println!("Tip: LONGBRIDGEA_LIVE=1 cargo run -p longbridgea-app  # live smoke test");
    }

    Ok(())
}
