//! LongbridgeA application entry.
//!
//! GUI bootstrap (gpui-kit) lands in Phase 2. For now this binary only proves
//! the workspace links.

fn main() -> anyhow::Result<()> {
    println!("LongbridgeA {}", env!("CARGO_PKG_VERSION"));
    println!("Scope: A-share selection & analysis (no trading).");
    println!("UI: {}", longbridgea_ui::pages::placeholder_watchlist());
    println!(
        "Quote client base: {}",
        tencent_quote::TencentQuoteClient::new(Default::default())
            .config()
            .quote_base
    );
    Ok(())
}
