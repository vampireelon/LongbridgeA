//! Pure parsers for Tencent `qt.gtimg.cn` snapshot bodies.

use longbridgea_core::{DepthLevel, Market, Quote, Symbol};

use crate::error::QuoteError;

fn parse_f64(s: &str) -> Option<f64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    t.parse().ok()
}

fn field<'a>(fields: &[&'a str], idx: usize) -> &'a str {
    fields.get(idx).copied().unwrap_or("")
}

/// Parse a single `v_sh600519="…"` line (or the quoted payload alone).
pub fn parse_quote_line(line: &str) -> Result<Quote, QuoteError> {
    let line = line.trim().trim_end_matches(';');
    if line.is_empty() {
        return Err(QuoteError::EmptyResponse);
    }

    let (wire_hint, payload) = if let Some(rest) = line.strip_prefix("v_") {
        let Some((name, after_eq)) = rest.split_once('=') else {
            return Err(QuoteError::BadFormat(line.chars().take(80).collect()));
        };
        let payload = after_eq.trim().trim_matches('"');
        (Some(name.trim()), payload)
    } else {
        (None, line.trim().trim_matches('"'))
    };

    let fields: Vec<&str> = payload.split('~').collect();
    if fields.len() < 35 {
        return Err(QuoteError::TooFewFields(fields.len()));
    }

    let code = field(&fields, 2);
    let symbol = if let Some(w) = wire_hint {
        Symbol::parse(w).or_else(|_| infer_symbol(code))?
    } else {
        infer_symbol(code)?
    };

    let mut bids = Vec::with_capacity(5);
    let mut asks = Vec::with_capacity(5);
    for i in 0..5 {
        let p = parse_f64(field(&fields, 9 + i * 2));
        let v = parse_f64(field(&fields, 10 + i * 2));
        if let (Some(price), Some(volume)) = (p, v) {
            bids.push(DepthLevel { price, volume });
        }
    }
    for i in 0..5 {
        let p = parse_f64(field(&fields, 19 + i * 2));
        let v = parse_f64(field(&fields, 20 + i * 2));
        if let (Some(price), Some(volume)) = (p, v) {
            asks.push(DepthLevel { price, volume });
        }
    }

    let update_time = {
        let t = field(&fields, 30);
        if t.is_empty() {
            None
        } else {
            Some(t.to_string())
        }
    };

    Ok(Quote {
        symbol,
        name: field(&fields, 1).to_string(),
        price: parse_f64(field(&fields, 3)).unwrap_or(0.0),
        prev_close: parse_f64(field(&fields, 4)).unwrap_or(0.0),
        open: parse_f64(field(&fields, 5)).unwrap_or(0.0),
        high: parse_f64(field(&fields, 33)).unwrap_or(0.0),
        low: parse_f64(field(&fields, 34)).unwrap_or(0.0),
        change: parse_f64(field(&fields, 31)).unwrap_or(0.0),
        change_pct: parse_f64(field(&fields, 32)).unwrap_or(0.0),
        volume: parse_f64(field(&fields, 6))
            .or_else(|| parse_f64(field(&fields, 36)))
            .unwrap_or(0.0),
        amount: parse_f64(field(&fields, 37)).unwrap_or(0.0),
        turnover_rate: parse_f64(field(&fields, 38)),
        pe: parse_f64(field(&fields, 39)),
        pb: parse_f64(field(&fields, 46)),
        amplitude: parse_f64(field(&fields, 43)),
        float_mv: parse_f64(field(&fields, 44)),
        total_mv: parse_f64(field(&fields, 45)),
        limit_up: parse_f64(field(&fields, 47)),
        limit_down: parse_f64(field(&fields, 48)),
        volume_ratio: parse_f64(field(&fields, 49)),
        update_time,
        bids,
        asks,
    })
}

fn infer_symbol(code: &str) -> Result<Symbol, QuoteError> {
    Symbol::parse(code).map_err(QuoteError::from)
}

/// Split a full HTTP body that may contain multiple `v_…="…";` records.
pub fn parse_quote_body(body: &str) -> Result<Vec<Quote>, QuoteError> {
    let mut out = Vec::new();
    for part in body.split(';') {
        let part = part.trim();
        if part.is_empty() || !part.contains('=') {
            continue;
        }
        if !part.trim_start().starts_with('v') {
            continue;
        }
        match parse_quote_line(part) {
            Ok(q) => out.push(q),
            Err(QuoteError::TooFewFields(_)) => continue,
            Err(e) => return Err(e),
        }
    }
    if out.is_empty() {
        return Err(QuoteError::EmptyResponse);
    }
    Ok(out)
}

/// Decode response bytes as GBK (Tencent default).
pub fn decode_gbk(bytes: &[u8]) -> String {
    let (cow, _, _) = encoding_rs::GBK.decode(bytes);
    cow.into_owned()
}

#[allow(dead_code)]
pub fn market_from_wire(wire: &str) -> Market {
    Symbol::parse(wire)
        .map(|s| s.market)
        .unwrap_or(Market::Unknown)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Field-aligned synthetic sample (indices match docs/tencent-quote-api.md).
    fn sample_line() -> String {
        // 0..49 with ~ separators; key indices set deliberately.
        let mut f = vec!["51"; 50];
        f[1] = "贵州茅台";
        f[2] = "600519";
        f[3] = "1460.00";
        f[4] = "1448.00";
        f[5] = "1455.00";
        f[6] = "12345";
        f[9] = "1459.00";
        f[10] = "10";
        f[19] = "1460.00";
        f[20] = "20";
        f[30] = "20261115150000";
        f[31] = "12.00";
        f[32] = "0.83";
        f[33] = "1472.00";
        f[34] = "1448.00";
        f[36] = "12345";
        f[37] = "180000";
        f[38] = "0.42";
        f[39] = "28.5";
        f[43] = "1.66";
        f[44] = "18000";
        f[45] = "18300";
        f[46] = "9.1";
        f[47] = "1592.80";
        f[48] = "1303.20";
        f[49] = "1.20";
        format!("v_sh600519=\"{}\";", f.join("~"))
    }

    #[test]
    fn parse_single_snapshot() {
        let q = parse_quote_line(&sample_line()).unwrap();
        assert_eq!(q.symbol.tencent_wire(), "sh600519");
        assert_eq!(q.name, "贵州茅台");
        assert!((q.price - 1460.0).abs() < 1e-6);
        assert!((q.change_pct - 0.83).abs() < 1e-6);
        assert!((q.amount - 180000.0).abs() < 1e-6);
        assert_eq!(q.pb, Some(9.1));
        assert_eq!(q.amplitude, Some(1.66));
        assert_eq!(q.bids.len(), 1);
        assert_eq!(q.asks.len(), 1);
        assert_eq!(q.update_time.as_deref(), Some("20261115150000"));
    }

    #[test]
    fn parse_body_multi() {
        let body = format!("{}\n{}", sample_line(), sample_line());
        let list = parse_quote_body(&body).unwrap();
        assert_eq!(list.len(), 2);
    }
}
