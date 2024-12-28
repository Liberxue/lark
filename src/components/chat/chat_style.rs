use super::chat_model::{ChatColors, ChatFonts, ChatMainStyle};
use bevy_egui::egui::{self, Color32, FontFamily, FontId, Margin, Rounding};
impl Default for ChatMainStyle {
    fn default() -> Self {
        Self {
            rounding: Rounding {
                nw: 0.0,
                ne: -12.0,
                sw: 0.0,
                se: 12.0,
            },
            margin: Margin {
                left: 0.,
                right: 12.,
                top: 12.,
                bottom: 12.,
            },
            colors: ChatColors::default(),
            fonts: ChatFonts::default(),
        }
    }
}

impl Default for ChatColors {
    fn default() -> Self {
        Self {
            background: egui::Color32::from_rgba_premultiplied(0, 0, 0, 200),
            text: egui::Color32::from_rgb(255, 255, 255),
            avatar_colors: vec![
                Color32::from_rgb(103, 58, 183), // Deep Purple
                Color32::from_rgb(63, 81, 181),  // Indigo
                Color32::from_rgb(33, 150, 243), // Blue
                Color32::from_rgb(3, 169, 244),  // Light Blue
                Color32::from_rgb(0, 188, 212),  // Cyan
                Color32::from_rgb(0, 150, 136),  // Teal
                Color32::from_rgb(76, 175, 80),  // Green
                Color32::from_rgb(244, 67, 54),  // Red
                Color32::from_rgb(233, 30, 99),  // Pink
                Color32::from_rgb(156, 39, 176), // Purple
            ],
        }
    }
}
impl Default for ChatFonts {
    fn default() -> Self {
        Self {
            title: FontId::new(16., FontFamily::default()),
            content: FontId::new(14.0, FontFamily::default()),
            timestamp: FontId::new(12.0, FontFamily::default()),
        }
    }
}
