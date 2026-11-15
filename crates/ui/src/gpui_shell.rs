//! gpui-kit root shell: sidebar + main panel driven by a text snapshot.
//!
//! The app process owns Sessions and periodically copies component
//! `render_text()` output into [`ShellSnapshot`]. This view only paints.

use crate::nav::Route;
use crate::theme::ColorTheme;
use gpui_kit::component::label::Label;
use gpui_kit::*;
use std::sync::{Arc, Mutex};

/// Thread-safe UI snapshot produced by app Sessions + component bind.
#[derive(Debug, Clone)]
pub struct ShellSnapshot {
    pub title: String,
    pub active_route: Route,
    pub sidebar_labels: Vec<(Route, String)>,
    pub tab_title: String,
    /// Main panel body (already formatted by components).
    pub main_text: String,
    pub status: String,
}

impl Default for ShellSnapshot {
    fn default() -> Self {
        Self {
            title: "LongbridgeA".into(),
            active_route: Route::Watchlist,
            sidebar_labels: Route::ALL
                .iter()
                .map(|r| (*r, r.label_zh().to_string()))
                .collect(),
            tab_title: Route::Watchlist.label_zh().into(),
            main_text: "等待数据…\nLONGBRIDGEA_GUI=1 启动后由 Session 填充".into(),
            status: "idle".into(),
        }
    }
}

pub type SharedSnapshot = Arc<Mutex<ShellSnapshot>>;

/// Root window content.
pub struct RootShell {
    pub snapshot: SharedSnapshot,
}

impl RootShell {
    pub fn new(snapshot: SharedSnapshot) -> Self {
        Self { snapshot }
    }
}

impl Render for RootShell {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let snap = self
            .snapshot
            .lock()
            .map(|g| g.clone())
            .unwrap_or_default();

        let theme = ColorTheme::dark(crate::theme::ChangeColorScheme::CnRedUpGreenDown);

        // Sidebar
        let mut side = div()
            .flex()
            .flex_col()
            .w(px(160.))
            .h_full()
            .gap_1()
            .p_2()
            .child(
                Label::new(snap.title.clone()).text_sm(),
            );

        for (route, label) in &snap.sidebar_labels {
            let active = *route == snap.active_route;
            let prefix = if active { "› " } else { "  " };
            side = side.child(Label::new(format!("{prefix}{label}")).text_sm());
        }

        // Main column: tab strip + body
        let main = div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_3()
                    .py_2()
                    .child(Label::new(format!("Tab: {}", snap.tab_title)).text_sm()),
            )
            .child(
                div()
                    .flex_1()
                    .p_3()
                    .child(Label::new(snap.main_text.clone()).text_sm()),
            )
            .child(
                div()
                    .px_3()
                    .py_1()
                    .child(Label::new(format!("status: {}", snap.status)).text_xs()),
            );

        div()
            .flex()
            .flex_row()
            .size_full()
            .child(side)
            .child(main)
            // Keep theme tokens referenced so linking stays honest for later color mapping.
            .when(false, |d| d.bg(rgb(parse_hex_rgb(theme.bg_page))))
    }
}

fn parse_hex_rgb(hex: &str) -> u32 {
    let h = hex.trim_start_matches('#');
    u32::from_str_radix(h, 16).unwrap_or(0x0a0e19)
}
