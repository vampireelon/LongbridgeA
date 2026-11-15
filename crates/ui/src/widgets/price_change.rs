//! Price / percent change display model.

use crate::theme::{ChangeColorScheme, ColorTheme, ThemeMode};

/// View-model for a signed change cell.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceChangeView {
    pub text: String,
    pub color_hex: &'static str,
    pub delta: f64,
}

impl PriceChangeView {
    pub fn from_pct(pct: f64, theme: &ColorTheme) -> Self {
        let sign = if pct > 0.0 {
            "+"
        } else {
            ""
        };
        Self {
            text: format!("{sign}{pct:.2}%"),
            color_hex: theme.change_color(pct),
            delta: pct,
        }
    }

    pub fn from_amount(amount: f64, theme: &ColorTheme) -> Self {
        let sign = if amount > 0.0 {
            "+"
        } else {
            ""
        };
        Self {
            text: format!("{sign}{amount:.2}"),
            color_hex: theme.change_color(amount),
            delta: amount,
        }
    }

    pub fn with_default_dark_cn(pct: f64) -> Self {
        let theme = ColorTheme::resolve(ThemeMode::Dark, ChangeColorScheme::CnRedUpGreenDown);
        Self::from_pct(pct, &theme)
    }
}

pub fn format_price(price: f64) -> String {
    if price >= 1000.0 {
        format!("{price:.2}")
    } else if price >= 100.0 {
        format!("{price:.2}")
    } else {
        format!("{price:.3}")
    }
}

pub fn format_pct(pct: f64) -> String {
    PriceChangeView::with_default_dark_cn(pct).text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pct_formatting() {
        let v = PriceChangeView::with_default_dark_cn(1.23);
        assert_eq!(v.text, "+1.23%");
        assert_eq!(v.color_hex, "#EF5350");
        let d = PriceChangeView::with_default_dark_cn(-0.5);
        assert_eq!(d.text, "-0.50%");
        assert_eq!(d.color_hex, "#26A69A");
    }
}
