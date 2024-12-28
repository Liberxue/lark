use bevy::prelude::ResMut;
use bevy_egui::egui;

use crate::resources::UiState;

struct WindowConf {
    width: f32,
    height: f32,
    position: egui::Pos2,
}
pub fn create_custom_window<'a>(
    ctx: &egui::Context,
    title: &'a str,
    config: WindowConf,
) -> egui::Window<'a> {
    egui::Window::new(title)
        .fixed_pos(config.position)
        .fixed_size(egui::vec2(config.width, config.height))
        .frame(
            egui::Frame::window(&ctx.style())
                .rounding(8.0)
                .shadow(egui::epaint::Shadow {
                    color: egui::Color32::from_black_alpha(60),
                    ..Default::default()
                })
                .fill(egui::Color32::from_rgb(32, 33, 36)) // 设置背景色
                .stroke(egui::Stroke::NONE) // 移除边框
                .outer_margin(0.0) // 移除外边距
                .inner_margin(8.0),
        )
        .title_bar(false)
        .resizable(false)
}

fn create_avatar_button(text: &str, size: f32, button_size: f32) -> impl egui::Widget + '_ {
    egui::Button::new(
        egui::RichText::new(text)
            .color(egui::Color32::WHITE)
            .size(size),
    )
    .fill(egui::Color32::from_rgb(255, 148, 0))
    .rounding(25.0)
    .min_size(egui::vec2(button_size, button_size))
}
// lark avatar ui
pub fn avatar(ui: &mut egui::Ui, ui_state: &mut ResMut<UiState>) {
    ui.add_space(5.);
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let avatar_response = ui.add(
            egui::Button::new(
                egui::RichText::new("鸣")
                    .color(egui::Color32::WHITE)
                    .size(16.),
            )
            .fill(egui::Color32::from_rgb(255, 148, 0))
            .rounding(30.0)
            .min_size(egui::vec2(32.0, 32.0)),
        );
        if avatar_response.clicked() {
            ui_state.show_avatar_menu = !ui_state.show_avatar_menu;
        }
        if ui_state.show_avatar_menu {
            show_avatar_menu(ui.ctx(), avatar_response.rect, ui_state);
        }
    });
}

struct MenuItem {
    text: String,
    show_red_hot: bool,
    is_separator_after: bool,
}
fn render_menu_item(ui: &mut egui::Ui, item: &MenuItem) -> egui::Response {
    let button = ui.add(
        egui::Button::new(&item.text)
            .fill(egui::Color32::TRANSPARENT)
            .min_size(egui::vec2(ui.available_width(), 36.)),
    );
    if item.show_red_hot {
        ui.painter().circle_filled(
            button.rect.right_center() - egui::vec2(10., 10.),
            4.,
            egui::Color32::RED,
        );
    }
    if item.is_separator_after {
        ui.separator();
    }
    button
}
pub fn show_avatar_menu(
    ctx: &egui::Context,
    avatar_rect: egui::Rect,
    ui_state: &mut ResMut<UiState>,
) {
    let menu_width = 280.;
    let config = WindowConf {
        width: 280.,
        height: 400.,
        position: egui::pos2(avatar_rect.right() + 10., avatar_rect.top()),
    };
    create_custom_window(ctx, "avatar_menu", config).show(ctx, |ui| {
        ui.set_min_width(menu_width);
        ui.add_space(16.0);
        ui.horizontal(|ui| {
            // 头像
            ui.add(create_avatar_button("鸣", 24.0, 48.0));
            ui.vertical(|ui| {
                ui.heading("鸣");
                ui.label("飞书个人用户");
                ui.add_space(4.);
                let status_button = ui.button("+ 状态");
                if status_button.clicked() {
                    ui_state.show_status_menu = !ui_state.show_status_menu;
                }
                if ui_state.show_status_menu {
                    show_status_menu(ui.ctx(), status_button.rect, ui_state);
                }
            });
        });
        ui.add_space(8.0);
        let mut signature = String::new();
        ui.add(
            egui::TextEdit::singleline(&mut signature)
                .hint_text("输入你的个性签名...") //TODO:
                .margin(egui::vec2(8.0, 8.0)),
        );
        ui.add_space(8.);
        ui.separator();

        let menu_items = vec![
            MenuItem {
                text: "我的个人名片".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "我的二维码与链接".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "添加账号".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "帮助与客服".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "设置".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "退出登录".to_string(),
                show_red_hot: true,
                is_separator_after: true,
            },
            MenuItem {
                text: "管理后台".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
        ];
        for (index, item) in menu_items.iter().enumerate() {
            if render_menu_item(ui, item).clicked() {
                println!("index {}", index)
            }
        }
    });
}

struct StatusMenuItem<'a> {
    icon: &'a str,
    text: &'a str,
    duration: &'a str,
}
fn render_status_menu_item(ui: &mut egui::Ui, item: &StatusMenuItem) {
    ui.add_space(8.);
    let resp = ui.add_sized(
        egui::vec2(ui.available_width(), 50.),
        egui::Button::new("").fill(egui::Color32::TRANSPARENT),
    );
    if resp.clicked() {
        //TODO:: to change status
    }
    status_style(ui, item.icon, item.text, item.duration, resp.rect);
}
pub fn show_status_menu(
    ctx: &egui::Context,
    button_rect: egui::Rect,
    ui_state: &mut ResMut<UiState>,
) {
    let _ = ui_state;
    let menu_width = 280.0;

    let menu_pos = egui::pos2(button_rect.right() + 10.0, button_rect.top());

    egui::Window::new("status_menu")
        .fixed_pos(menu_pos)
        .fixed_size(egui::vec2(menu_width, 400.0))
        .frame(
            egui::Frame::window(&ctx.style())
                .rounding(8.0)
                .shadow(egui::epaint::Shadow {
                    color: egui::Color32::from_black_alpha(60),
                    ..Default::default()
                })
                .fill(egui::Color32::from_rgb(32, 33, 36)) // 设置背景色
                .stroke(egui::Stroke::NONE) // 移除边框
                .outer_margin(0.0) // 移除外边距
                .inner_margin(8.0),
        ) // 设置内边距
        .title_bar(false)
        .resizable(false)
        .title_bar(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_min_width(menu_width);
            ui.heading("我的状态");
            ui.add_space(8.0);
            //TODO:: replace icon
            let status_items = vec![
                StatusMenuItem {
                    icon: "🔕",
                    text: "请勿打扰",
                    duration: "1 小时 🔕静音",
                },
                StatusMenuItem {
                    icon: "📅",
                    text: "会议中",
                    duration: "1 小时",
                },
                StatusMenuItem {
                    icon: "☕",
                    text: "休息中",
                    duration: "1 小时",
                },
                StatusMenuItem {
                    icon: "💬",
                    text: "生病请假~",
                    duration: "至今晚",
                },
            ];
            for item in status_items {
                render_status_menu_item(ui, &item);
            }
            ui.add_space(8.);
            ui.separator();
            ui.add_space(8.);
            ui.horizontal(|ui| {
                if ui
                    .button(egui::RichText::new("+ 新建状态").size(14.))
                    .clicked()
                {
                    //TODO:
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(egui::RichText::new("⚙").size(16.0)).clicked() {
                        // TODO::
                    }
                })
            })
        });
}

fn status_style(ui: &mut egui::Ui, icon: &str, text: &str, duration: &str, resp: egui::Rect) {
    ui.allocate_ui_at_rect(resp, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(icon).size(17.));
            ui.add_space(15.);
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(text).size(17.)); // 12?
                ui.label(
                    egui::RichText::new(duration)
                        .size(14.)
                        .color(egui::Color32::GRAY),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("...")
                        .size(14.)
                        .color(egui::Color32::GRAY),
                )
            });
        });
    });
}
