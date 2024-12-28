use bevy_egui::egui::{self, Color32, FontId, Margin, Rounding};

use crate::{ChatMessage, MessageType};

#[derive(Clone)]
pub struct ChatMainStyle {
    pub rounding: Rounding,
    pub margin: Margin,
    pub colors: ChatColors,
    pub fonts: ChatFonts,
}

#[derive(Clone)]
pub struct ChatColors {
    pub background: Color32,
    pub text: Color32,
    pub avatar_colors: Vec<Color32>,
}
#[derive(Clone)]
pub struct ChatFonts {
    pub title: FontId,
    pub content: FontId,
    pub timestamp: FontId,
}

pub trait MessageRenderer {
    fn render(&self, ui: &mut egui::Ui, message: &ChatMessage, style: &ChatMainStyle);
}

#[derive(Clone)]
pub struct ToolBarButton {
    pub icon: &'static str,
    pub tooltip: &'static str,
    pub action: ToolbarAction,
}

#[derive(Clone)]
pub enum ToolbarAction {
    ToggleEmoji,
    SetMessageType(MessageType),
    None,
}
