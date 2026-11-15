//! LongbridgeA UI layer.
//!
//! Page models + presentation components. Optional `gpui` feature adds a
//! desktop shell view that consumes bound text snapshots from the app.

#![forbid(unsafe_code)]

pub mod components;
pub mod nav;
pub mod pages;
pub mod shell;
pub mod theme;
pub mod widgets;

#[cfg(feature = "gpui")]
pub mod gpui_shell;

pub use nav::{NavItem, Route};
pub use shell::{AppShell, ShellLayout, Tab, TabKind};
pub use theme::{ChangeColorScheme, ColorTheme, Spacing, ThemeMode};

pub fn skeleton_banner() -> String {
    #[cfg(feature = "gpui")]
    {
        format!(
            "LongbridgeA UI | routes={} | gpui=on",
            Route::ALL.len()
        )
    }
    #[cfg(not(feature = "gpui"))]
    {
        format!(
            "LongbridgeA UI | routes={} | gpui=off (cargo run -p longbridgea-app --features gpui)",
            Route::ALL.len()
        )
    }
}
