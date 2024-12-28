use super::{avatar, windows::windows_button};
use crate::resources::UiState;
use bevy::prelude::ResMut;
use bevy_egui::egui::{self};

#[derive(Clone)]
struct NavItem {
    icon: &'static str,
    lable: &'static str,
    has_notification: bool,
}
pub fn left_nav_ui(ctx: &egui::Context, ui_state: &mut ResMut<UiState>) -> egui::InnerResponse<()> {
    let style = ctx.style();
    egui::SidePanel::left("left_nav_ui")
        .default_width(50.0)
        .width_range(50.0..=100.0)
        .max_width(100.)
        .frame(egui::Frame {
            rounding: egui::Rounding {
                nw: 12.0, // 左上
                ne: 0.0,  // 右上
                sw: 12.0, // 左下
                se: 0.0,  // 右下
            },
            shadow: style.visuals.window_shadow,
            inner_margin: egui::Margin {
                left: 12.,
                right: 8.,
                top: 8.,
                bottom: 8.,
            },
            fill: egui::Color32::from_rgba_premultiplied(0, 0, 0, 220),
            ..Default::default()
        })
        .resizable(true)
        .show(ctx, |ui| {
            // update width
            ui_state.nav_width = ui.available_width();
            // Top icon
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                #[cfg(target_os = "macos")]
                windows_button(ui);
            });

            avatar(ui, ui_state);
            ui.add_space(10.0);
            let nav_items = [
                //search lark
                NavItem {
                    icon: "\u{e71a}",
                    lable: "",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{ebb4}",
                    lable: "消 息",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{ebb5}",
                    lable: "日 历", // icon 好像不合适， 人家lark 日历 icon 好像展示日期TODO。。
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{ebb6}",
                    lable: "云文档",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{e662}",
                    lable: "视频会议",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{e6a8}",
                    lable: "多维表格",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{ebb3}",
                    lable: "联系人",
                    has_notification: false,
                },
            ];
            // render icon ..
            for (index, item) in nav_items.iter().enumerate() {
                let resp = render_nav_item(
                    ctx,
                    ui,
                    item,
                    ui_state.selected_nav_index == index,
                    ui_state.nav_width > 60.0,
                );
                if resp.clicked() {
                    ui_state.selected_nav_index = index
                }
                ui.add_space(10.0);
            }
        })
}

/// render nav item
fn render_nav_item(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    item: &NavItem,
    is_selected: bool,
    is_expanded: bool,
) -> egui::Response {
    ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
    const ICON_SIZE: f32 = 20.0;
    const TEXT_SIZE: f32 = 10.0;
    let color = if is_selected {
        egui::Color32::from_rgba_premultiplied(22, 119, 255, 1)
    } else {
        egui::Color32::WHITE
    };
    let icon_text = egui::RichText::new(item.icon)
        .font(egui::FontId::proportional(ICON_SIZE))
        .color(color)
        .strong();
    let label_text = egui::RichText::new(item.lable)
        .font(egui::FontId::proportional(TEXT_SIZE))
        .color(color);

    let response = if is_expanded {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.horizontal(|ui| {
                let icon_response = ui.label(icon_text);
                if !item.lable.is_empty() {
                    ui.add_space(8.);
                    let label_response = ui.label(label_text);
                    icon_response.union(label_response)
                } else {
                    icon_response
                }
            })
            .inner
        })
        .inner
    } else {
        ui.vertical_centered(|ui| {
            let icon_response = ui.label(icon_text);
            if !item.lable.is_empty() {
                ui.add_space(8.);
                let label_response = ui.label(label_text);
                icon_response.union(label_response)
            } else {
                icon_response
            }
        })
        .inner
    };

    if item.has_notification {
        ui.painter().circle_filled(
            response.rect.right_top() - egui::vec2(4., -4.),
            4.,
            egui::Color32::DARK_RED,
        );
    }
    response
}
