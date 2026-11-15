//! Application shell: sidebar + tabs + content route state.

use crate::nav::{NavItem, Route};
use crate::theme::{ChangeColorScheme, ThemeMode};
use longbridgea_core::Symbol;
use serde::{Deserialize, Serialize};

/// What a content tab shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TabKind {
    /// Primary page for a sidebar route (single system tab).
    Route(Route),
    /// Per-symbol analysis workspace.
    StockDetail { symbol_wire: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub title: String,
    pub kind: TabKind,
    pub closable: bool,
}

impl Tab {
    pub fn for_route(route: Route) -> Self {
        Self {
            id: format!("route:{}", route.id()),
            title: route.label_zh().to_string(),
            kind: TabKind::Route(route),
            closable: false,
        }
    }

    pub fn for_stock(symbol: &Symbol, name: &str) -> Self {
        let wire = symbol.tencent_wire();
        Self {
            id: format!("stock:{wire}"),
            title: if name.is_empty() {
                wire.clone()
            } else {
                format!("{name} {wire}")
            },
            kind: TabKind::StockDetail { symbol_wire: wire },
            closable: true,
        }
    }
}

/// Static layout metrics for the system chrome.
#[derive(Debug, Clone, Copy)]
pub struct ShellLayout {
    pub sidebar_width: f32,
    pub sidebar_collapsed_width: f32,
    pub titlebar_height: f32,
    pub tabbar_height: f32,
    pub min_window_width: f32,
    pub min_window_height: f32,
}

impl Default for ShellLayout {
    fn default() -> Self {
        Self {
            sidebar_width: 200.0,
            sidebar_collapsed_width: 56.0,
            titlebar_height: 40.0,
            tabbar_height: 36.0,
            min_window_width: 1200.0,
            min_window_height: 720.0,
        }
    }
}

/// Mutable shell state (no GUI types — safe for tests and future gpui binding).
#[derive(Debug, Clone)]
pub struct AppShell {
    pub theme_mode: ThemeMode,
    pub change_scheme: ChangeColorScheme,
    pub sidebar_collapsed: bool,
    pub active_route: Route,
    pub nav_items: Vec<NavItem>,
    pub tabs: Vec<Tab>,
    pub active_tab_id: String,
    pub layout: ShellLayout,
    /// Global search query draft (Ctrl/Cmd+K).
    pub search_query: String,
    pub search_open: bool,
}

impl Default for AppShell {
    fn default() -> Self {
        let active_route = Route::Watchlist;
        let tab = Tab::for_route(active_route);
        let active_tab_id = tab.id.clone();
        Self {
            theme_mode: ThemeMode::Dark,
            change_scheme: ChangeColorScheme::CnRedUpGreenDown,
            sidebar_collapsed: false,
            active_route,
            nav_items: NavItem::defaults_zh(),
            tabs: vec![tab],
            active_tab_id,
            layout: ShellLayout::default(),
            search_query: String::new(),
            search_open: false,
        }
    }
}

impl AppShell {
    pub fn navigate(&mut self, route: Route) {
        self.active_route = route;
        let id = format!("route:{}", route.id());
        if !self.tabs.iter().any(|t| t.id == id) {
            self.tabs.push(Tab::for_route(route));
        }
        self.active_tab_id = id;
    }

    pub fn open_stock(&mut self, symbol: &Symbol, name: &str) {
        let tab = Tab::for_stock(symbol, name);
        let id = tab.id.clone();
        if let Some(existing) = self.tabs.iter().find(|t| t.id == id) {
            self.active_tab_id = existing.id.clone();
        } else {
            self.tabs.push(tab);
            self.active_tab_id = id;
        }
        self.active_route = Route::Analysis;
    }

    pub fn close_tab(&mut self, tab_id: &str) {
        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id && t.closable) {
            self.tabs.remove(pos);
            if self.active_tab_id == tab_id {
                self.active_tab_id = self
                    .tabs
                    .last()
                    .map(|t| t.id.clone())
                    .unwrap_or_else(|| Tab::for_route(Route::Watchlist).id);
            }
        }
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        self.tabs.iter().find(|t| t.id == self.active_tab_id)
    }

    pub fn toggle_sidebar(&mut self) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
    }

    pub fn open_search(&mut self) {
        self.search_open = true;
    }

    pub fn close_search(&mut self) {
        self.search_open = false;
        self.search_query.clear();
    }

    /// ASCII wireframe for debugging / CLI.
    pub fn debug_layout(&self) -> String {
        let side = if self.sidebar_collapsed { "icons" } else { "full" };
        let nav: Vec<_> = self
            .nav_items
            .iter()
            .map(|n| {
                let mark = if n.route == self.active_route {
                    "*"
                } else {
                    " "
                };
                format!("{mark}{}", n.label)
            })
            .collect();
        let tabs: Vec<_> = self
            .tabs
            .iter()
            .map(|t| {
                let mark = if t.id == self.active_tab_id { "[" } else { " " };
                let end = if t.id == self.active_tab_id { "]" } else { " " };
                format!("{mark}{}{end}", t.title)
            })
            .collect();
        format!(
            "shell sidebar={side} route={} tabs={} nav={{{}}}",
            self.active_route.id(),
            tabs.join(" "),
            nav.join(" ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use longbridgea_core::Symbol;

    #[test]
    fn navigate_and_open_stock() {
        let mut shell = AppShell::default();
        shell.navigate(Route::Screener);
        assert_eq!(shell.active_route, Route::Screener);

        let sym = Symbol::parse("sh600519").unwrap();
        shell.open_stock(&sym, "贵州茅台");
        assert!(shell.active_tab_id.contains("600519"));
        assert_eq!(shell.tabs.iter().filter(|t| t.closable).count(), 1);

        shell.close_tab(&shell.active_tab_id.clone());
        assert!(!shell.tabs.iter().any(|t| t.closable));
    }
}
