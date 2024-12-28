use bevy::prelude::ResMut;
use bevy_egui::egui::{self, FontFamily, FontId, Frame, RichText};
use chrono::Local;

use crate::{resources::UiState, ChatMainView, ChatMessage, MessageType};

pub fn chat_main_ui(
    ctx: &egui::Context,
    ui_state: &mut ResMut<UiState>,
) -> egui::InnerResponse<()> {
    let view = ChatMainView::new();
    view.render(ctx, ui_state)
}
fn get_avatar_color(text: &str) -> egui::Color32 {
    const COLORS: &[egui::Color32] = &[
        egui::Color32::from_rgb(103, 58, 183), // Deep Purple
        egui::Color32::from_rgb(63, 81, 181),  // Indigo
        egui::Color32::from_rgb(33, 150, 243), // Blue
        egui::Color32::from_rgb(3, 169, 244),  // Light Blue
        egui::Color32::from_rgb(0, 188, 212),  // Cyan
        egui::Color32::from_rgb(0, 150, 136),  // Teal
        egui::Color32::from_rgb(76, 175, 80),  // Green
        egui::Color32::from_rgb(244, 67, 54),  // Red
        egui::Color32::from_rgb(233, 30, 99),  // Pink
        egui::Color32::from_rgb(156, 39, 176), // Purple
    ];

    let index = text
        .bytes()
        .fold(0usize, |acc, b| acc.wrapping_add(b as usize))
        % COLORS.len();
    COLORS[index]
}
