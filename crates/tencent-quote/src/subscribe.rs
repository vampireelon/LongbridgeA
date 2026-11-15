//! Polling subscription hub for watchlist-style quote updates.
//!
//! UI / app holds [`QuoteHub`], calls [`QuoteHub::subscribe`] / `unsubscribe`,
//! and listens on a [`tokio::sync::broadcast`] channel for [`QuoteTick`] events.

use crate::cache::QuoteCache;
use crate::{QuoteError, Result, TencentQuoteClient};
use longbridgea_core::{Quote, Symbol};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::task::JoinHandle;
use tokio::time::{interval, MissedTickBehavior};

/// One poll cycle result pushed to subscribers.
#[derive(Debug, Clone)]
pub struct QuoteTick {
    /// Successfully fetched (and cached) quotes this cycle.
    pub quotes: Vec<Quote>,
    /// Human-readable errors (network / parse); empty on full success.
    pub errors: Vec<String>,
    /// Symbols that were requested this cycle.
    pub requested: Vec<String>,
}

/// Configuration for the background poller.
#[derive(Debug, Clone)]
pub struct PollConfig {
    /// Interval between full-batch fetches.
    pub interval: Duration,
    /// broadcast channel capacity (oldest dropped when full).
    pub broadcast_capacity: usize,
}

impl Default for PollConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(3),
            broadcast_capacity: 64,
        }
    }
}

/// Shared hub: symbol set + cache + poll loop + broadcast.
#[derive(Debug)]
pub struct QuoteHub {
    client: TencentQuoteClient,
    cache: Arc<QuoteCache>,
    symbols: Arc<RwLock<HashSet<String>>>,
    tx: broadcast::Sender<QuoteTick>,
    poll: PollConfig,
    /// Background task handle (if started).
    worker: RwLock<Option<JoinHandle<()>>>,
}

impl QuoteHub {
    pub fn new(client: TencentQuoteClient, cache: Arc<QuoteCache>, poll: PollConfig) -> Self {
        let (tx, _) = broadcast::channel(poll.broadcast_capacity.max(1));
        Self {
            client,
            cache,
            symbols: Arc::new(RwLock::new(HashSet::new())),
            tx,
            poll,
            worker: RwLock::new(None),
        }
    }

    pub fn with_defaults(client: TencentQuoteClient) -> Self {
        Self::new(
            client,
            Arc::new(QuoteCache::with_default_ttl()),
            PollConfig::default(),
        )
    }

    pub fn cache(&self) -> &Arc<QuoteCache> {
        &self.cache
    }

    /// Subscribe to tick stream (cloneable receiver).
    pub fn subscribe_ticks(&self) -> broadcast::Receiver<QuoteTick> {
        self.tx.subscribe()
    }

    /// Add symbols to the poll set (idempotent).
    pub async fn watch(&self, symbols: &[Symbol]) {
        let mut g = self.symbols.write().await;
        for s in symbols {
            g.insert(s.tencent_wire());
        }
    }

    /// Remove symbols from the poll set.
    pub async fn unwatch(&self, symbols: &[Symbol]) {
        let mut g = self.symbols.write().await;
        for s in symbols {
            g.remove(&s.tencent_wire());
        }
    }

    pub async fn unwatch_all(&self) {
        self.symbols.write().await.clear();
    }

    pub async fn watched(&self) -> Vec<String> {
        let g = self.symbols.read().await;
        let mut v: Vec<_> = g.iter().cloned().collect();
        v.sort();
        v
    }

    /// One-shot: fetch batch, update cache, return quotes (does not require watch set).
    pub async fn fetch_and_cache(&self, symbols: &[Symbol]) -> Result<Vec<Quote>> {
        if symbols.is_empty() {
            return Ok(Vec::new());
        }
        let quotes = self.client.fetch_quotes(symbols).await?;
        self.cache.put_many(quotes.clone());
        Ok(quotes)
    }

    /// Prefer cache if fresh; otherwise network fetch for a single symbol.
    pub async fn get_quote(&self, symbol: &Symbol) -> Result<Quote> {
        if let Some(q) = self.cache.get(symbol) {
            return Ok(q);
        }
        let mut list = self.fetch_and_cache(std::slice::from_ref(symbol)).await?;
        list.pop().ok_or(QuoteError::EmptyResponse)
    }

    /// Run one poll cycle against the current watch set.
    pub async fn poll_once(&self) -> QuoteTick {
        let wires: Vec<String> = {
            let g = self.symbols.read().await;
            g.iter().cloned().collect()
        };
        if wires.is_empty() {
            return QuoteTick {
                quotes: vec![],
                errors: vec![],
                requested: vec![],
            };
        }

        let mut symbols = Vec::with_capacity(wires.len());
        let mut errors = Vec::new();
        for w in &wires {
            match Symbol::parse(w) {
                Ok(s) => symbols.push(s),
                Err(e) => errors.push(format!("bad symbol {w}: {e}")),
            }
        }

        match self.client.fetch_quotes(&symbols).await {
            Ok(quotes) => {
                self.cache.put_many(quotes.clone());
                QuoteTick {
                    quotes,
                    errors,
                    requested: wires,
                }
            }
            Err(e) => {
                errors.push(e.to_string());
                QuoteTick {
                    quotes: vec![],
                    errors,
                    requested: wires,
                }
            }
        }
    }

    /// Start background polling. Safe to call once; subsequent calls are no-ops
    /// while a worker is already running.
    pub async fn start(&self) {
        {
            let g = self.worker.read().await;
            if g.is_some() {
                return;
            }
        }

        let client = self.client.clone();
        let cache = Arc::clone(&self.cache);
        let symbols = Arc::clone(&self.symbols);
        let tx = self.tx.clone();
        let every = self.poll.interval;

        let handle = tokio::spawn(async move {
            let mut tick = interval(every);
            tick.set_missed_tick_behavior(MissedTickBehavior::Skip);
            // First tick completes immediately; skip so we wait one full interval
            // or poll right away — we poll immediately for snappier UI.
            loop {
                let wires: Vec<String> = {
                    let g = symbols.read().await;
                    g.iter().cloned().collect()
                };

                if !wires.is_empty() {
                    let mut syms = Vec::new();
                    let mut errors = Vec::new();
                    for w in &wires {
                        match Symbol::parse(w) {
                            Ok(s) => syms.push(s),
                            Err(e) => errors.push(format!("bad symbol {w}: {e}")),
                        }
                    }
                    let event = match client.fetch_quotes(&syms).await {
                        Ok(quotes) => {
                            cache.put_many(quotes.clone());
                            QuoteTick {
                                quotes,
                                errors,
                                requested: wires,
                            }
                        }
                        Err(e) => {
                            errors.push(e.to_string());
                            QuoteTick {
                                quotes: vec![],
                                errors,
                                requested: wires,
                            }
                        }
                    };
                    let _ = tx.send(event);
                }

                tick.tick().await;
            }
        });

        *self.worker.write().await = Some(handle);
    }

    /// Abort background worker if running.
    pub async fn stop(&self) {
        if let Some(h) = self.worker.write().await.take() {
            h.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;

    #[tokio::test]
    async fn watch_set_and_poll_empty() {
        let client = TencentQuoteClient::new(ClientConfig::default()).unwrap();
        let hub = QuoteHub::with_defaults(client);
        let tick = hub.poll_once().await;
        assert!(tick.requested.is_empty());

        let sym = Symbol::parse("sh600519").unwrap();
        hub.watch(&[sym.clone()]).await;
        assert_eq!(hub.watched().await, vec!["sh600519".to_string()]);
        hub.unwatch(&[sym]).await;
        assert!(hub.watched().await.is_empty());
    }
}
