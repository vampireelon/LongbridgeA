//! Chart helpers: indicators and geometry inputs.
//!
//! Drawing stays in `longbridgea-ui` / gpui; this crate focuses on pure data transforms.

#![forbid(unsafe_code)]

use longbridgea_core::Bar;

/// Simple moving average of close prices. Returns `None` positions until the window is full.
pub fn sma(bars: &[Bar], period: usize) -> Vec<Option<f64>> {
    if period == 0 {
        return bars.iter().map(|_| None).collect();
    }
    let mut out = Vec::with_capacity(bars.len());
    let mut sum = 0.0;
    for i in 0..bars.len() {
        sum += bars[i].close;
        if i >= period {
            sum -= bars[i - period].close;
        }
        if i + 1 >= period {
            out.push(Some(sum / period as f64));
        } else {
            out.push(None);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use longbridgea_core::Bar;

    #[test]
    fn sma_period_2() {
        let bars: Vec<Bar> = [1.0, 2.0, 3.0, 4.0]
            .into_iter()
            .enumerate()
            .map(|(i, c)| Bar {
                ts: i as i64,
                open: c,
                high: c,
                low: c,
                close: c,
                volume: 0.0,
            })
            .collect();
        let s = sma(&bars, 2);
        assert_eq!(s[0], None);
        assert_eq!(s[1], Some(1.5));
        assert_eq!(s[2], Some(2.5));
        assert_eq!(s[3], Some(3.5));
    }
}
