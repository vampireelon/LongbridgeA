//! Screener page model (conditions + results; no execution engine yet).

use longbridgea_core::Quote;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RangeFilter {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl Default for RangeFilter {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
        }
    }
}

impl RangeFilter {
    pub fn matches(&self, value: f64) -> bool {
        if let Some(min) = self.min {
            if value < min {
                return false;
            }
        }
        if let Some(max) = self.max {
            if value > max {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScreenerCriteria {
    pub change_pct: RangeFilter,
    pub amount: RangeFilter,
    pub turnover_rate: RangeFilter,
    pub volume_ratio: RangeFilter,
    pub pe: RangeFilter,
    pub pb: RangeFilter,
    pub total_mv: RangeFilter,
}

impl ScreenerCriteria {
    pub fn matches_quote(&self, q: &Quote) -> bool {
        if !self.change_pct.matches(q.change_pct) {
            return false;
        }
        if !self.amount.matches(q.amount) {
            return false;
        }
        if let Some(t) = q.turnover_rate {
            if !self.turnover_rate.matches(t) {
                return false;
            }
        }
        if let Some(v) = q.volume_ratio {
            if !self.volume_ratio.matches(v) {
                return false;
            }
        }
        if let Some(pe) = q.pe {
            if !self.pe.matches(pe) {
                return false;
            }
        }
        if let Some(pb) = q.pb {
            if !self.pb.matches(pb) {
                return false;
            }
        }
        if let Some(mv) = q.total_mv {
            if !self.total_mv.matches(mv) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenerStatus {
    Idle,
    Running,
    Done,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct ScreenerPage {
    pub criteria: ScreenerCriteria,
    pub results: Vec<Quote>,
    pub status: ScreenerStatus,
    pub selected: Vec<String>,
    pub preset_name: Option<String>,
}

impl Default for ScreenerPage {
    fn default() -> Self {
        Self {
            criteria: ScreenerCriteria::default(),
            results: Vec::new(),
            status: ScreenerStatus::Idle,
            selected: Vec::new(),
            preset_name: None,
        }
    }
}

impl ScreenerPage {
    pub fn apply_local_filter(&mut self, universe: &[Quote]) {
        self.status = ScreenerStatus::Running;
        self.results = universe
            .iter()
            .filter(|q| self.criteria.matches_quote(q))
            .cloned()
            .collect();
        self.status = ScreenerStatus::Done;
    }
}
