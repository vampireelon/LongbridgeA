//! Tencent public market-data client.
//!
//! See `docs/tencent-quote-api.md` for endpoint and field notes.

#![forbid(unsafe_code)]

mod error;
mod parse;

pub use error::{QuoteError, Result};
pub use parse::{decode_gbk, parse_quote_body, parse_quote_line};

use longbridgea_core::{Quote, Symbol};
use std::time::Duration;

/// Client configuration.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub quote_base: String,
    pub timeout_ms: u64,
    pub user_agent: String,
    pub referer: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            quote_base: "https://qt.gtimg.cn".to_string(),
            timeout_ms: 10_000,
            user_agent: "Mozilla/5.0 (compatible; LongbridgeA/0.1)".to_string(),
            referer: "https://gu.qq.com/".to_string(),
        }
    }
}

/// HTTP quote client for `qt.gtimg.cn`.
#[derive(Debug, Clone)]
pub struct TencentQuoteClient {
    config: ClientConfig,
    http: reqwest::Client,
}

impl TencentQuoteClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .user_agent(config.user_agent.clone())
            .build()?;
        Ok(Self { config, http })
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Fetch one or more full snapshot quotes (batch via comma-separated `q=`).
    pub async fn fetch_quotes(&self, symbols: &[Symbol]) -> Result<Vec<Quote>> {
        if symbols.is_empty() {
            return Ok(Vec::new());
        }
        let wires: Vec<String> = symbols.iter().map(|s| s.tencent_wire()).collect();
        let url = format!("{}/q={}", self.config.quote_base.trim_end_matches('/'), wires.join(","));

        let bytes = self
            .http
            .get(&url)
            .header("Referer", &self.config.referer)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;

        let text = decode_gbk(&bytes);
        parse_quote_body(&text)
    }

    /// Convenience: parse a single symbol string then fetch.
    pub async fn fetch_quote_str(&self, symbol: &str) -> Result<Quote> {
        let sym = Symbol::parse(symbol)?;
        let mut list = self.fetch_quotes(&[sym]).await?;
        list.pop().ok_or(QuoteError::EmptyResponse)
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

    #[test]
    fn client_builds() {
        let c = TencentQuoteClient::new(ClientConfig::default()).unwrap();
        assert!(c.config().timeout_ms > 0);
    }
}
