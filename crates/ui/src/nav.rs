//! Primary navigation routes (sidebar).

use serde::{Deserialize, Serialize};

/// Top-level application route (sidebar items).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Route {
    Watchlist,
    Market,
    Screener,
    Analysis,
    Settings,
}

impl Route {
    pub const ALL: &'static [Route] = &[
        Route::Watchlist,
        Route::Market,
        Route::Screener,
        Route::Analysis,
        Route::Settings,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Route::Watchlist => "watchlist",
            Route::Market => "market",
            Route::Screener => "screener",
            Route::Analysis => "analysis",
            Route::Settings => "settings",
        }
    }

    pub fn label_zh(self) -> &'static str {
        match self {
            Route::Watchlist => "自选",
            Route::Market => "行情",
            Route::Screener => "选股",
            Route::Analysis => "分析",
            Route::Settings => "设置",
        }
    }

    pub fn label_en(self) -> &'static str {
        match self {
            Route::Watchlist => "Watchlist",
            Route::Market => "Market",
            Route::Screener => "Screener",
            Route::Analysis => "Analysis",
            Route::Settings => "Settings",
        }
    }

    /// Keyboard chord hint (Ctrl/Cmd + digit), if any.
    pub fn shortcut_digit(self) -> Option<u8> {
        match self {
            Route::Watchlist => Some(1),
            Route::Market => Some(2),
            Route::Screener => Some(3),
            Route::Settings => Some(4),
            Route::Analysis => None,
        }
    }
}

/// Sidebar row descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavItem {
    pub route: Route,
    pub label: String,
    pub icon: &'static str,
}

impl NavItem {
    pub fn defaults_zh() -> Vec<Self> {
        Route::ALL
            .iter()
            .copied()
            .map(|route| NavItem {
                route,
                label: route.label_zh().to_string(),
                icon: match route {
                    Route::Watchlist => "star",
                    Route::Market => "chart",
                    Route::Screener => "filter",
                    Route::Analysis => "search",
                    Route::Settings => "gear",
                },
            })
            .collect()
    }
}
