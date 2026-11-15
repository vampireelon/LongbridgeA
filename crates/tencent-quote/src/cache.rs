//! In-memory TTL cache for quote snapshots (and optional K-line slots).

use longbridgea_core::{Bar, Quote, Symbol};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Entry<T> {
    value: T,
    fetched_at: Instant,
}

impl<T> Entry<T> {
    fn fresh(&self, ttl: Duration) -> bool {
        self.fetched_at.elapsed() < ttl
    }
}

/// Thread-safe quote snapshot cache with per-entry TTL.
#[derive(Debug)]
pub struct QuoteCache {
    inner: RwLock<HashMap<String, Entry<Quote>>>,
    ttl: Duration,
}

impl QuoteCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
            ttl,
        }
    }

    /// Default TTL: 3 seconds (watchlist-friendly).
    pub fn with_default_ttl() -> Self {
        Self::new(Duration::from_secs(3))
    }

    pub fn ttl(&self) -> Duration {
        self.ttl
    }

    pub fn len(&self) -> usize {
        self.inner.read().map(|m| m.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&self) {
        if let Ok(mut m) = self.inner.write() {
            m.clear();
        }
    }

    /// Return cached quote if present and not expired.
    pub fn get(&self, symbol: &Symbol) -> Option<Quote> {
        let key = symbol.tencent_wire();
        let guard = self.inner.read().ok()?;
        let e = guard.get(&key)?;
        if e.fresh(self.ttl) {
            Some(e.value.clone())
        } else {
            None
        }
    }

    /// Insert or replace; refreshes `fetched_at`.
    pub fn put(&self, quote: Quote) {
        let key = quote.symbol.tencent_wire();
        if let Ok(mut m) = self.inner.write() {
            m.insert(
                key,
                Entry {
                    value: quote,
                    fetched_at: Instant::now(),
                },
            );
        }
    }

    pub fn put_many(&self, quotes: impl IntoIterator<Item = Quote>) {
        if let Ok(mut m) = self.inner.write() {
            let now = Instant::now();
            for q in quotes {
                let key = q.symbol.tencent_wire();
                m.insert(
                    key,
                    Entry {
                        value: q,
                        fetched_at: now,
                    },
                );
            }
        }
    }

    /// All still-fresh quotes (order undefined).
    pub fn snapshot_fresh(&self) -> Vec<Quote> {
        let Ok(guard) = self.inner.read() else {
            return Vec::new();
        };
        guard
            .values()
            .filter(|e| e.fresh(self.ttl))
            .map(|e| e.value.clone())
            .collect()
    }
}

/// Cache key for historical K-line / minute K.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KlineCacheKey {
    pub wire: String,
    /// e.g. `day:qfq` or `m5`
    pub series: String,
}

impl KlineCacheKey {
    pub fn daily(symbol: &Symbol, period: &str, adjust: &str) -> Self {
        Self {
            wire: symbol.tencent_wire(),
            series: format!("{period}:{adjust}"),
        }
    }

    pub fn minute_k(symbol: &Symbol, interval: &str) -> Self {
        Self {
            wire: symbol.tencent_wire(),
            series: interval.to_string(),
        }
    }
}

/// Longer-lived bar series cache (default TTL 60s).
#[derive(Debug)]
pub struct KlineCache {
    inner: RwLock<HashMap<KlineCacheKey, Entry<Vec<Bar>>>>,
    ttl: Duration,
}

impl KlineCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
            ttl,
        }
    }

    pub fn with_default_ttl() -> Self {
        Self::new(Duration::from_secs(60))
    }

    pub fn get(&self, key: &KlineCacheKey) -> Option<Vec<Bar>> {
        let guard = self.inner.read().ok()?;
        let e = guard.get(key)?;
        if e.fresh(self.ttl) {
            Some(e.value.clone())
        } else {
            None
        }
    }

    pub fn put(&self, key: KlineCacheKey, bars: Vec<Bar>) {
        if let Ok(mut m) = self.inner.write() {
            m.insert(
                key,
                Entry {
                    value: bars,
                    fetched_at: Instant::now(),
                },
            );
        }
    }

    pub fn clear(&self) {
        if let Ok(mut m) = self.inner.write() {
            m.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use longbridgea_core::{Market, Symbol};
    use std::thread;

    fn sample_quote(code: &str) -> Quote {
        Quote {
            symbol: Symbol::new(Market::Sh, code),
            name: "测试".into(),
            price: 10.0,
            prev_close: 9.0,
            open: 9.5,
            high: 10.5,
            low: 9.0,
            change: 1.0,
            change_pct: 11.11,
            volume: 100.0,
            amount: 1000.0,
            turnover_rate: None,
            pe: None,
            pb: None,
            amplitude: None,
            float_mv: None,
            total_mv: None,
            limit_up: None,
            limit_down: None,
            volume_ratio: None,
            update_time: None,
            bids: vec![],
            asks: vec![],
        }
    }

    #[test]
    fn quote_ttl_expiry() {
        let cache = QuoteCache::new(Duration::from_millis(50));
        let q = sample_quote("600519");
        let sym = q.symbol.clone();
        cache.put(q);
        assert!(cache.get(&sym).is_some());
        thread::sleep(Duration::from_millis(80));
        assert!(cache.get(&sym).is_none());
    }

    #[test]
    fn kline_key_roundtrip() {
        let sym = Symbol::parse("sh600519").unwrap();
        let k = KlineCacheKey::daily(&sym, "day", "qfq");
        let cache = KlineCache::with_default_ttl();
        cache.put(
            k.clone(),
            vec![Bar {
                ts: 0,
                date: "2026-11-15".into(),
                open: 1.0,
                high: 2.0,
                low: 0.5,
                close: 1.5,
                volume: 10.0,
            }],
        );
        assert_eq!(cache.get(&k).unwrap().len(), 1);
    }
}
