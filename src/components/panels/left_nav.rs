use super::{avatar, windows::windows_button};
use crate::resources::{NotificationTheme, ThemeMode, UiState}; // 添加 ThemeMode 导入
use bevy::prelude::{Entity, Query, ResMut};
use bevy::window::Window;
use bevy::winit::WinitWindows;
use bevy_egui::egui::{self};

#[derive(Clone)]
struct NavItem {
    icon: &'static str,
    lable: &'static str,
    has_notification: bool,
}

pub fn left_nav_ui(
    ctx: &egui::Context,
    ui_state: &mut ResMut<UiState>,
    theme: &mut ResMut<NotificationTheme>,
    window_query: &Query<(Entity, &Window)>,
    winit_windows: &WinitWindows,
) -> egui::InnerResponse<()> {
    let style = ctx.style();
    let colors = theme.current_colors();

    egui::SidePanel::left("left_nav_ui")
        .default_width(50.0)
        .width_range(50.0..=120.0)
        .max_width(120.)
        .frame(egui::Frame {
            rounding: theme.style.nav_rounding,
            shadow: style.visuals.window_shadow,
            inner_margin: theme.style.sidebar_margin,
            fill: colors.background,
            ..Default::default()
        })
        .resizable(true)
        .show(ctx, |ui| {
            ui_state.nav_width = ui.available_width();
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                #[cfg(target_os = "macos")]
                if let Ok((window_entity, _window)) = window_query.get_single() {
                    windows_button(ui, window_entity, winit_windows);
                }
            });

            avatar(ui, ui_state);
            ui.add_space(10.0);

            let nav_items = [
                NavItem {
                    icon: "\u{e71a}",
                    lable: "",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{ebb4}",
                    lable: "消 息",
                    has_notification: true,
                },
                NavItem {
                    icon: "\u{eb2b}",
                    lable: "日 历",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{ebb6}",
                    lable: "文档",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{e80c}",
                    lable: "会议",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{e6a8}",
                    lable: "表格",
                    has_notification: false,
                },
                NavItem {
                    icon: "\u{ebb3}",
                    lable: "联系人",
                    has_notification: false,
                },
            ];
            for (index, item) in nav_items.iter().enumerate() {
                let resp = if index == 0 {
                    render_search_nav_item(
                        ctx,
                        ui,
                        ui_state.selected_nav_index == index,
                        ui_state.nav_width > 60.,
                        &theme,
                        ui_state,
                    )
                } else {
                    render_nav_item(
                        ctx,
                        ui,
                        item,
                        ui_state.selected_nav_index == index,
                        ui_state.nav_width > 60.,
                        &theme,
                    )
                };

                if resp.clicked() {
                    ui_state.selected_nav_index = index
                }
                ui.add_space(10.);
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);

                let theme_icon = if matches!(theme.mode, ThemeMode::Light) {
                    "\u{e6ed}"
                } else {
                    "\u{ec86}"
                };

                let theme_nav_item = NavItem {
                    icon: theme_icon,
                    lable: "",
                    has_notification: false,
                };

                let theme_resp = render_nav_item(
                    ctx,
                    ui,
                    &theme_nav_item,
                    false,
                    ui_state.nav_width > 60.,
                    &theme,
                );

                if theme_resp.clicked() {
                    theme.toggle_mode();
                }
            });
        })
}

struct NavItemStyle<'a> {
    ctx: &'a egui::Context,
    theme: &'a NotificationTheme,
    is_selected: bool,
}

impl<'a> NavItemStyle<'a> {
    fn new(ctx: &'a egui::Context, theme: &'a NotificationTheme, is_selected: bool) -> Self {
        Self {
            ctx,
            theme,
            is_selected,
        }
    }

    fn get_icon_text(&self, icon: &str) -> egui::RichText {
        let icon_style = &self.theme.text_styles.nav_icon;
        let icon_color = if self.is_selected {
            icon_style.selected_color
        } else {
            icon_style.color
        };

        egui::RichText::new(icon)
            .font(self.theme.fonts.nav_icon.clone())
            .size(25.)
            .color(icon_color)
            .strong()
    }

    fn get_label_text(&self, label: &str) -> egui::RichText {
        let label_style = &self.theme.text_styles.nav_label;
        let label_color = if self.is_selected {
            label_style.selected_color
        } else {
            label_style.color
        };

        egui::RichText::new(label)
            .font(self.theme.fonts.nav_label.clone())
            .size(13.)
            .color(label_color)
    }

    fn paint_hover_effect(&self, ui: &mut egui::Ui, response: &egui::Response) {
        if response.hovered() {
            let icon_style = &self.theme.text_styles.nav_icon;
            ui.painter().rect_filled(
                response.rect,
                self.theme.style.rounding,
                if self.is_selected {
                    icon_style.selected_color.linear_multiply(0.1)
                } else {
                    icon_style.hover_color.linear_multiply(0.1)
                },
            );
        }
    }
}

fn render_nav_item(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    item: &NavItem,
    is_selected: bool,
    is_expanded: bool,
    theme: &NotificationTheme,
) -> egui::Response {
    ctx.set_cursor_icon(egui::CursorIcon::PointingHand);

    let style = NavItemStyle::new(ctx, theme, is_selected);
    let icon_text = style.get_icon_text(item.icon);
    let label_text = style.get_label_text(item.lable);

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

    style.paint_hover_effect(ui, &response);

    if item.has_notification {
        ui.painter().circle_filled(
            response.rect.right_top() - egui::vec2(4., -4.),
            4.,
            theme.text_styles.nav_notification.color,
        );
    }

    response
}

fn render_search_nav_item(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    is_selected: bool,
    is_expanded: bool,
    theme: &NotificationTheme,
    ui_state: &mut ResMut<UiState>,
) -> egui::Response {
    ctx.set_cursor_icon(egui::CursorIcon::PointingHand);

    let style = NavItemStyle::new(ctx, theme, is_selected);
    let icon_text = style.get_icon_text("\u{e71a}");

    let response = if is_expanded {
        ui.horizontal(|ui| {
            let icon_response = ui.label(icon_text);
            ui.add_space(8.);
            let search_response = ui.add(
                egui::TextEdit::singleline(&mut ui_state.search_text)
                    .desired_width(ui.available_width() - 30.0)
                    .hint_text("搜索..."),
            );
            icon_response.union(search_response)
        })
        .inner
    } else {
        ui.vertical_centered(|ui| ui.label(icon_text)).inner
    };

    style.paint_hover_effect(ui, &response);

    response
}
