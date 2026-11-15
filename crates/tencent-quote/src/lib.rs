//! Tencent public market-data client.
//!
//! See `docs/tencent-quote-api.md` for endpoint and field notes.

#![forbid(unsafe_code)]

mod error;
mod kline;
mod minute;
mod parse;

pub use error::{QuoteError, Result};
pub use kline::{kline_data_key, parse_kline_json};
pub use minute::parse_minute_json;
pub use parse::{decode_gbk, parse_quote_body, parse_quote_line};

use longbridgea_core::{Adjust, Bar, KlinePeriod, MinuteSeries, Quote, Symbol};
use std::time::Duration;

/// Client configuration.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub quote_base: String,
    /// Base host for minute + fqkline JSON APIs.
    pub ifzq_base: String,
    pub timeout_ms: u64,
    pub user_agent: String,
    pub referer: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            quote_base: "https://qt.gtimg.cn".to_string(),
            ifzq_base: "https://web.ifzq.gtimg.cn".to_string(),
            timeout_ms: 10_000,
            user_agent: "Mozilla/5.0 (compatible; LongbridgeA/0.1)".to_string(),
            referer: "https://gu.qq.com/".to_string(),
        }
    }
}

/// HTTP client for Tencent public quote endpoints.
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

    async fn get_text(&self, url: &str) -> Result<String> {
        let bytes = self
            .http
            .get(url)
            .header("Referer", &self.config.referer)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        // Snapshot is GBK; ifzq JSON is typically UTF-8. Try UTF-8 first, fall back GBK.
        match std::str::from_utf8(&bytes) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Ok(decode_gbk(&bytes)),
        }
    }

    /// Fetch one or more full snapshot quotes (batch via comma-separated `q=`).
    pub async fn fetch_quotes(&self, symbols: &[Symbol]) -> Result<Vec<Quote>> {
        if symbols.is_empty() {
            return Ok(Vec::new());
        }
        let wires: Vec<String> = symbols.iter().map(|s| s.tencent_wire()).collect();
        let url = format!(
            "{}/q={}",
            self.config.quote_base.trim_end_matches('/'),
            wires.join(",")
        );
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

    pub async fn fetch_quote_str(&self, symbol: &str) -> Result<Quote> {
        let sym = Symbol::parse(symbol)?;
        let mut list = self.fetch_quotes(&[sym]).await?;
        list.pop().ok_or(QuoteError::EmptyResponse)
    }

    /// Intraday minute series for one symbol.
    pub async fn fetch_minute(&self, symbol: &Symbol) -> Result<MinuteSeries> {
        let url = format!(
            "{}/appstock/app/minute/query?code={}",
            self.config.ifzq_base.trim_end_matches('/'),
            symbol.tencent_wire()
        );
        let text = self.get_text(&url).await?;
        parse_minute_json(&text, symbol)
    }

    /// Historical K-line bars (default: last `count` days, front-adjusted).
    pub async fn fetch_kline(
        self: &Self,
        symbol: &Symbol,
        period: KlinePeriod,
        adjust: Adjust,
        count: u32,
    ) -> Result<Vec<Bar>> {
        let count = count.max(1);
        let adj = adjust.as_tencent_suffix();
        let param = if adj.is_empty() {
            format!(
                "{},{},,,{}",
                symbol.tencent_wire(),
                period.as_tencent(),
                count
            )
        } else {
            format!(
                "{},{},,,{},{}",
                symbol.tencent_wire(),
                period.as_tencent(),
                count,
                adj
            )
        };
        let url = format!(
            "{}/appstock/app/fqkline/get?param={}",
            self.config.ifzq_base.trim_end_matches('/'),
            param
        );
        let text = self.get_text(&url).await?;
        parse_kline_json(&text, symbol, period, adjust)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_points_at_gtimg() {
        let c = ClientConfig::default();
        assert!(c.quote_base.contains("gtimg"));
        assert!(c.ifzq_base.contains("ifzq"));
    }

    #[test]
    fn client_builds() {
        let c = TencentQuoteClient::new(ClientConfig::default()).unwrap();
        assert!(c.config().timeout_ms > 0);
    }
}
