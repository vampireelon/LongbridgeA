//! LongbridgeA UI layer.
//!
//! Phase 2 skeleton: design tokens, navigation shell state, page models and
//! pure widgets. Optional `gpui` feature will host gpui-kit views later.
//!
//! See `docs/DESIGN.md` and `docs/UI_SKELETON.md`.

#![forbid(unsafe_code)]

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
        "LongbridgeA UI skeleton | routes={} | theme=dark cn-red-up",
        Route::ALL.len()
    )
}
