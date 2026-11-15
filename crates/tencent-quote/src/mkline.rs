//! Minute K-line parser (`ifzq.gtimg.cn/.../kline/mkline`).
//!
//! Row layout: `[YYYYMMDDHHMM, open, close, high, low, volume, …]`.

use longbridgea_core::{Bar, MinuteInterval, Symbol};
use serde_json::Value;

use crate::error::{QuoteError, Result};

fn parse_f64_str(v: &Value) -> f64 {
    match v {
        Value::String(s) => s.parse().unwrap_or(0.0),
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        _ => 0.0,
    }
}

/// Parse JSON body from mkline endpoint.
pub fn parse_mkline_json(
    body: &str,
    symbol: &Symbol,
    interval: MinuteInterval,
) -> Result<Vec<Bar>> {
    let root: Value = serde_json::from_str(body)?;
    let code = root.get("code").and_then(|v| v.as_i64()).unwrap_or(-1);
    if code != 0 {
        let msg = root
            .get("msg")
            .and_then(|v| v.as_str())
            .unwrap_or("mkline api error");
        return Err(QuoteError::Msg(format!("mkline api code={code}: {msg}")));
    }

    let wire = symbol.tencent_wire();
    let stock = root
        .pointer(&format!("/data/{wire}"))
        .ok_or_else(|| QuoteError::Msg(format!("mkline: missing data.{wire}")))?;

    let key = interval.as_tencent();
    let rows = stock
        .get(key)
        .and_then(|v| v.as_array())
        .ok_or_else(|| QuoteError::Msg(format!("mkline: missing array key {key}")))?;

    let mut bars = Vec::with_capacity(rows.len());
    for row in rows {
        let Some(arr) = row.as_array() else {
            continue;
        };
        if arr.len() < 6 {
            continue;
        }
        let date = arr[0].as_str().unwrap_or("").to_string();
        // Same OHLC order as fqkline: open, close, high, low, volume
        let open = parse_f64_str(&arr[1]);
        let close = parse_f64_str(&arr[2]);
        let high = parse_f64_str(&arr[3]);
        let low = parse_f64_str(&arr[4]);
        let volume = parse_f64_str(&arr[5]);
        bars.push(Bar {
            ts: 0,
            date,
            open,
            high,
            low,
            close,
            volume,
        });
    }

    if bars.is_empty() {
        return Err(QuoteError::EmptyResponse);
    }
    Ok(bars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_m5_sample() {
        let body = r#"{
            "code":0,
            "msg":"",
            "data":{
                "sh600519":{
                    "m5":[
                        ["202611131325","1428.17","1424.89","1429.00","1423.46","520.00",{},"0.42"],
                        ["202611131330","1424.89","1431.01","1431.09","1424.07","1020.00",{},"0.82"]
                    ]
                }
            }
        }"#;
        let sym = Symbol::parse("sh600519").unwrap();
        let bars = parse_mkline_json(body, &sym, MinuteInterval::M5).unwrap();
        assert_eq!(bars.len(), 2);
        assert_eq!(bars[0].date, "202611131325");
        assert!((bars[0].open - 1428.17).abs() < 1e-6);
        assert!((bars[0].close - 1424.89).abs() < 1e-6);
        assert!((bars[0].high - 1429.0).abs() < 1e-6);
        assert!((bars[0].low - 1423.46).abs() < 1e-6);
        assert!((bars[0].volume - 520.0).abs() < 1e-6);
    }
}
