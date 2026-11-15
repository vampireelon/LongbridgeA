//! Presentation components bound to page view-models.
//!
//! These are GUI-agnostic: they only read `pages::*` state and produce
//! structured view data or plain text. gpui views will map the same structs
//! to elements later; headless CLI already uses `render_*`.

pub mod chart_panel;
pub mod depth_book;
pub mod quote_header;
pub mod watchlist_table;

pub use chart_panel::{ChartPanelView, ChartPointView};
pub use depth_book::DepthBookView;
pub use quote_header::QuoteHeaderView;
pub use watchlist_table::{WatchlistRowView, WatchlistTableView};
