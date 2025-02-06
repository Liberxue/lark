use bevy::prelude::ResMut;
use bevy_egui::egui::{self};

use crate::{
    resources::{NotificationTheme, UiState},
    ChatMainView,
};

pub fn chat_main_ui(
    ctx: &egui::Context,
    ui_state: &mut ResMut<UiState>,
    theme: &mut ResMut<NotificationTheme>,
) -> egui::InnerResponse<()> {
    let view = ChatMainView::new();
    view.render(ctx, ui_state, theme)
}
