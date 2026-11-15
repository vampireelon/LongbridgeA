//! Intraday minute series parser (`web.ifzq.gtimg.cn/.../minute/query`).

use longbridgea_core::{MinutePoint, MinuteSeries, Symbol};
use serde_json::Value;

use crate::error::{QuoteError, Result};

/// Parse JSON body from minute/query.
pub fn parse_minute_json(body: &str, symbol: &Symbol) -> Result<MinuteSeries> {
    let root: Value = serde_json::from_str(body)?;
    let code = root.get("code").and_then(|v| v.as_i64()).unwrap_or(-1);
    if code != 0 {
        let msg = root
            .get("msg")
            .and_then(|v| v.as_str())
            .unwrap_or("minute api error");
        return Err(QuoteError::Msg(format!("minute api code={code}: {msg}")));
    }

    let wire = symbol.tencent_wire();
    let stock = root
        .pointer(&format!("/data/{wire}"))
        .ok_or_else(|| QuoteError::Msg(format!("minute: missing data.{wire}")))?;

    let date = stock
        .pointer("/data/date")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let rows = stock
        .pointer("/data/data")
        .and_then(|v| v.as_array())
        .ok_or_else(|| QuoteError::Msg("minute: missing data.data array".into()))?;

    let mut points = Vec::with_capacity(rows.len());
    for row in rows {
        let Some(s) = row.as_str() else {
            continue;
        };
        // "0930 1404.00 150 21060000.00" — time price vol_cum [amount_cum]
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() < 3 {
            continue;
        }
        let time = parts[0].to_string();
        let price: f64 = parts[1].parse().unwrap_or(0.0);
        let volume_cum: f64 = parts[2].parse().unwrap_or(0.0);
        let amount_cum: f64 = parts.get(3).and_then(|p| p.parse().ok()).unwrap_or(0.0);
        points.push(MinutePoint {
            time,
            price,
            volume_cum,
            amount_cum,
        });
    }

    if points.is_empty() {
        return Err(QuoteError::EmptyResponse);
    }

    Ok(MinuteSeries {
        symbol: symbol.clone(),
        date,
        points,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use longbridgea_core::Symbol;

    #[test]
    fn parse_sample_minute() {
        let body = r#"{
            "code":0,
            "msg":"",
            "data":{
                "sh600519":{
                    "data":{
                        "data":["0930 1404.00 150 21060000.00","0931 1397.51 864 120847636.00"],
                        "date":"20261113"
                    }
                }
            }
        }"#;
        let sym = Symbol::parse("sh600519").unwrap();
        let series = parse_minute_json(body, &sym).unwrap();
        assert_eq!(series.date.as_deref(), Some("20261113"));
        assert_eq!(series.points.len(), 2);
        assert_eq!(series.points[0].time, "0930");
        assert!((series.points[0].price - 1404.0).abs() < 1e-6);
        assert!((series.points[1].volume_cum - 864.0).abs() < 1e-6);
    }
}
