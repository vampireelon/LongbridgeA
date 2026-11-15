//! Application UI layer.
//!
//! Phase 2 will depend on `gpui-kit` and implement shell + pages described in `docs/DESIGN.md`.
//! This skeleton only reserves module layout.

#![forbid(unsafe_code)]

pub mod pages {
    //! Top-level routes: watchlist, stock detail, screener, market, settings.

    pub fn placeholder_watchlist() -> &'static str {
        "Watchlist (UI not wired yet)"
    }

    pub fn placeholder_screener() -> &'static str {
        "Screener (UI not wired yet)"
    }
}

pub mod theme {
    //! Design tokens bridge; see `docs/DESIGN.md`.

    /// A-share convention: red up / green down.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub enum ChangeColorScheme {
        #[default]
        CnRedUpGreenDown,
        GlobalGreenUpRedDown,
    }
}
