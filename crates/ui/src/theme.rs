//! Design tokens from `docs/DESIGN.md`.

use serde::{Deserialize, Serialize};

/// A-share default: red up / green down.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeColorScheme {
    #[default]
    CnRedUpGreenDown,
    GlobalGreenUpRedDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
    System,
}

/// Resolved color palette (hex strings for now; gpui Color mapping later).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColorTheme {
    pub bg_page: &'static str,
    pub bg_surface: &'static str,
    pub bg_elevated: &'static str,
    pub bg_sidebar: &'static str,
    pub border_default: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub text_muted: &'static str,
    pub accent_brand: &'static str,
    pub accent_brand_soft: &'static str,
    pub up: &'static str,
    pub down: &'static str,
    pub flat: &'static str,
    pub warning: &'static str,
    pub danger: &'static str,
}

impl ColorTheme {
    pub fn dark(scheme: ChangeColorScheme) -> Self {
        let (up, down) = match scheme {
            ChangeColorScheme::CnRedUpGreenDown => ("#EF5350", "#26A69A"),
            ChangeColorScheme::GlobalGreenUpRedDown => ("#26A69A", "#EF5350"),
        };
        Self {
            bg_page: "#0A0E19",
            bg_surface: "#161A26",
            bg_elevated: "#202127",
            bg_sidebar: "#0A0E19",
            border_default: "#3C3F44",
            text_primary: "#DFDFD6",
            text_secondary: "#98989F",
            text_muted: "#6B6E76",
            accent_brand: "#00F0C4",
            accent_brand_soft: "rgba(0,240,196,0.08)",
            up,
            down,
            flat: "#98989F",
            warning: "#F0B90B",
            danger: "#F6465D",
        }
    }

    pub fn light(scheme: ChangeColorScheme) -> Self {
        let (up, down) = match scheme {
            ChangeColorScheme::CnRedUpGreenDown => ("#EF5350", "#26A69A"),
            ChangeColorScheme::GlobalGreenUpRedDown => ("#26A69A", "#EF5350"),
        };
        Self {
            bg_page: "#F5F6F8",
            bg_surface: "#FFFFFF",
            bg_elevated: "#F0F1F3",
            bg_sidebar: "#FFFFFF",
            border_default: "#E5E7EB",
            text_primary: "#1A1D26",
            text_secondary: "#6B7280",
            text_muted: "#9CA3AF",
            accent_brand: "#00ADA2",
            accent_brand_soft: "rgba(0,173,162,0.08)",
            up,
            down,
            flat: "#6B7280",
            warning: "#F0B90B",
            danger: "#F6465D",
        }
    }

    pub fn resolve(mode: ThemeMode, scheme: ChangeColorScheme) -> Self {
        match mode {
            ThemeMode::Dark | ThemeMode::System => Self::dark(scheme),
            ThemeMode::Light => Self::light(scheme),
        }
    }

    /// Color for a signed change (pct or amount).
    pub fn change_color(&self, delta: f64) -> &'static str {
        if delta > 0.0 {
            self.up
        } else if delta < 0.0 {
            self.down
        } else {
            self.flat
        }
    }
}

/// Spacing scale (px).
#[derive(Debug, Clone, Copy)]
pub struct Spacing;

impl Spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;
}

/// Corner radii (px).
#[derive(Debug, Clone, Copy)]
pub struct Radius;

impl Radius {
    pub const SM: f32 = 4.0;
    pub const MD: f32 = 8.0;
    pub const LG: f32 = 12.0;
    pub const XL: f32 = 16.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cn_scheme_red_up() {
        let t = ColorTheme::dark(ChangeColorScheme::CnRedUpGreenDown);
        assert_eq!(t.change_color(1.0), t.up);
        assert_eq!(t.change_color(-1.0), t.down);
        assert_eq!(t.up, "#EF5350");
    }
}
