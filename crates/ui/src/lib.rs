//! LongbridgeA UI layer.
//!
//! Page models + presentation components. Sessions live in `longbridgea-app`
//! and write into page state; components **bind** by reading that state.
//!
//! See `docs/DESIGN.md` and `docs/UI_SKELETON.md`.

#![forbid(unsafe_code)]

pub mod components;
pub mod nav;
pub mod pages;
pub mod shell;
pub mod theme;
pub mod widgets;

pub use nav::{NavItem, Route};
pub use shell::{AppShell, ShellLayout, Tab, TabKind};
pub use theme::{ChangeColorScheme, ColorTheme, Spacing, ThemeMode};

/// Human-readable skeleton banner for CLI smoke tests.
pub fn skeleton_banner() -> String {
    format!(
        "LongbridgeA UI | routes={} | components=watchlist/quote/depth/chart",
        Route::ALL.len()
    )
}
