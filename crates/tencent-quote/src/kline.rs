//! Daily/weekly/monthly K-line parser (`fqkline/get`).
//!
//! Tencent qfqday row layout: `[date, open, close, high, low, volume]`.

use longbridgea_core::{Adjust, Bar, KlinePeriod, Symbol};
use serde_json::Value;

use crate::error::{QuoteError, Result};

fn parse_f64_str(v: &Value) -> f64 {
    match v {
        Value::String(s) => s.parse().unwrap_or(0.0),
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        _ => 0.0,
    }
}

/// Key under stock object: `qfqday` / `hfqday` / `day` / week/month variants.
pub fn kline_data_key(period: KlinePeriod, adjust: Adjust) -> String {
    let p = period.as_tencent();
    match adjust {
        Adjust::Qfq => format!("qfq{p}"),
        Adjust::Hfq => format!("hfq{p}"),
        Adjust::None => p.to_string(),
    }
}

pub fn parse_kline_json(
    body: &str,
    symbol: &Symbol,
    period: KlinePeriod,
    adjust: Adjust,
) -> Result<Vec<Bar>> {
    let root: Value = serde_json::from_str(body)?;
    let code = root.get("code").and_then(|v| v.as_i64()).unwrap_or(-1);
    if code != 0 {
        let msg = root
            .get("msg")
            .and_then(|v| v.as_str())
            .unwrap_or("kline api error");
        return Err(QuoteError::Msg(format!("kline api code={code}: {msg}")));
    }

    let wire = symbol.tencent_wire();
    let stock = root
        .pointer(&format!("/data/{wire}"))
        .ok_or_else(|| QuoteError::Msg(format!("kline: missing data.{wire}")))?;

    let key = kline_data_key(period, adjust);
    let rows = stock
        .get(&key)
        .or_else(|| stock.get(period.as_tencent()))
        .and_then(|v| v.as_array())
        .ok_or_else(|| QuoteError::Msg(format!("kline: missing array key {key}")))?;

    let mut bars = Vec::with_capacity(rows.len());
    for row in rows {
        let Some(arr) = row.as_array() else {
            continue;
        };
        if arr.len() < 6 {
            continue;
        }
        let date = arr[0].as_str().unwrap_or("").to_string();
        // Tencent order: open, close, high, low, volume
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
    fn parse_qfq_day_sample() {
        let body = r#"{
            "code":0,
            "msg":"",
            "data":{
                "sh600519":{
                    "qfqday":[
                        ["2026-11-09","1440.060","1444.990","1449.990","1435.080","24801.000"],
                        ["2026-11-10","1438.230","1439.000","1444.500","1431.260","23038.000"]
                    ]
                }
            }
        }"#;
        let sym = Symbol::parse("sh600519").unwrap();
        let bars = parse_kline_json(body, &sym, KlinePeriod::Day, Adjust::Qfq).unwrap();
        assert_eq!(bars.len(), 2);
        assert_eq!(bars[0].date, "2026-11-09");
        assert!((bars[0].open - 1440.06).abs() < 1e-6);
        assert!((bars[0].close - 1444.99).abs() < 1e-6);
        assert!((bars[0].high - 1449.99).abs() < 1e-6);
        assert!((bars[0].low - 1435.08).abs() < 1e-6);
    }
}
