//! Tencent public market-data client.
//!
//! See `docs/tencent-quote-api.md` for endpoint and field notes.
//! Phase 1 will implement real HTTP + GBK parsing; this skeleton only wires types.

#![forbid(unsafe_code)]

use longbridgea_core::{Quote, Symbol};

/// Client configuration (base URLs, timeout, user-agent).
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub quote_base: String,
    pub timeout_ms: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            quote_base: "https://qt.gtimg.cn".to_string(),
            timeout_ms: 10_000,
        }
    }
}

/// HTTP quote client (placeholder).
#[derive(Debug, Clone)]
pub struct TencentQuoteClient {
    config: ClientConfig,
}

impl TencentQuoteClient {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Fetch one or more snapshot quotes. Not implemented in skeleton.
    pub async fn fetch_quotes(&self, _symbols: &[Symbol]) -> anyhow::Result<Vec<Quote>> {
        Err(anyhow::anyhow!(
            "tencent-quote: fetch_quotes not implemented yet (Phase 1)"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_points_at_gtimg() {
        let c = ClientConfig::default();
        assert!(c.quote_base.contains("gtimg"));
    }
}
