use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Frame};

use crate::resources::UiState;

pub fn left_chat_list_ui(
    ctx: &egui::Context,
    ui_state: &mut ResMut<UiState>,
) -> egui::InnerResponse<()> {
    egui::SidePanel::left("chat_list_panel")
        .resizable(true)
        .default_width(120.0)
        .width_range(120.0..=180.0)
        .frame(Frame {
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 252),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("<<|").clicked() {
                    ui_state.show_siderbar = !ui_state.show_siderbar;
                }
                ui.label("消息");
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
}

pub fn left_sidebar_ui(
    ctx: &egui::Context,
    ui_state: &mut ResMut<UiState>,
) -> egui::InnerResponse<()> {
    egui::SidePanel::left("lef_sidebar_ui")
        .resizable(true)
        .max_width(180.)
        .default_width(150.)
        .frame(Frame {
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 252),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
            ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(200, 200, 200));
            ui.vertical(|ui| {
                ui.heading("分组");
                ui.add_space(10.);
                let menu_items = [
                    ("标记", 2),
                    ("@我", 2),
                    ("标签", 2),
                    ("单聊", 2),
                    ("群组", 2),
                    ("云文档", 2),
                    ("话题", 2),
                    ("已完成", 2),
                ];
                for (label, count) in menu_items {
                    ui.horizontal(|ui| {
                        ui.add_space(10.);
                        let is_selected = ui_state.selected_siderbar_button == label;
                        let response = ui.selectable_label(is_selected, label);
                        ui.add_space(ui.available_width() - 45.);
                        ui.label(count.to_string());

                        if response.clicked() {
                            ui_state.selected_siderbar_button = label.to_string();
                        }
                        ui.add_space(10.);
                    });
                }
            });
        })
}
