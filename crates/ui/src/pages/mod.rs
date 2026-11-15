//! Page-level view models (no gpui types).

pub mod market;
pub mod screener;
pub mod settings;
pub mod stock;
pub mod watchlist;

pub use market::MarketPage;
pub use screener::ScreenerPage;
pub use settings::SettingsPage;
pub use stock::StockDetailPage;
pub use watchlist::WatchlistPage;

/// Legacy placeholders kept for app smoke output.
pub fn placeholder_watchlist() -> &'static str {
    "Watchlist page model ready (gpui view pending)"
}

pub fn placeholder_screener() -> &'static str {
    "Screener page model ready (gpui view pending)"
}
