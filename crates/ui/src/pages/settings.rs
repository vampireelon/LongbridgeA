//! Settings page model.

use crate::theme::{ChangeColorScheme, ThemeMode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsPage {
    pub theme_mode: ThemeMode,
    pub change_scheme: ChangeColorScheme,
    /// Watchlist poll interval seconds.
    pub watchlist_poll_secs: u64,
    pub detail_poll_secs: u64,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::Dark,
            change_scheme: ChangeColorScheme::CnRedUpGreenDown,
            watchlist_poll_secs: 3,
            detail_poll_secs: 3,
        }
    }
}

impl SettingsPage {
    pub fn watchlist_poll(&self) -> Duration {
        Duration::from_secs(self.watchlist_poll_secs.max(1))
    }
}
