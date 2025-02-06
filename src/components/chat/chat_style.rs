use super::{
    chat_model::{ChatColors, ChatFonts, ChatMainStyle},
    AVATAR_COLORS,
};
use bevy_egui::egui::{self, FontFamily, FontId, Margin, Rounding};
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
            avatar_colors: AVATAR_COLORS.to_vec(),
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
