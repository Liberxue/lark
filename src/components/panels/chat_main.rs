use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Frame};

use crate::resources::UiState;

pub fn chat_main_ui(
    ctx: &egui::Context,
    ui_state: &mut ResMut<UiState>,
) -> egui::InnerResponse<()> {
    let style = ctx.style();
    egui::SidePanel::left("chat_main_ui")
        .frame(Frame {
            rounding: egui::Rounding {
                nw: 0.0,  // 左上
                ne: 12.0, // 右上
                sw: 0.0,  // 左下
                se: 12.0, // 右下
            },
            inner_margin: egui::Margin {
                left: 0.,
                right: 12.,
                top: 12.,
                bottom: 12.,
            },
            shadow: style.visuals.window_shadow,
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 255),
            ..Default::default()
        })
        .resizable(true)
        .default_width(1000.0)
        .width_range(1000.0..=1280.0)
        .min_width(600.)
        .show(ctx, |ui| {
            ui.label("Main resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
}
