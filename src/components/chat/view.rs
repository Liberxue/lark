use super::{controller::ChatListController, ChatEvent, ChatListItem, AVATAR_COLORS};
use crate::resources::NotificationTheme;
use bevy_egui::egui::{self, Color32, Margin};

pub struct ChatListView<'a> {
    controller: &'a mut ChatListController,
}

struct BadgePosition {
    offset_x: f32,
    offset_y: f32,
}

impl BadgePosition {
    const DEFAULT: Self = Self {
        offset_x: 0.9,
        offset_y: 0.1,
    };

    fn calculate_pos(&self, avatar_rect: egui::Rect) -> egui::Pos2 {
        egui::pos2(
            avatar_rect.right() - (avatar_rect.width() * (1.0 - self.offset_x)),
            avatar_rect.top() + (avatar_rect.height() * self.offset_y),
        )
    }
}

impl<'a> ChatListView<'a> {
    pub fn new(controller: &'a mut ChatListController) -> Self {
        Self { controller }
    }

    fn prepare_chat_items(&self) -> Vec<ChatListItem> {
        self.controller
            .view()
            .filtered()
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn render(&mut self, ui: &mut egui::Ui, theme: &NotificationTheme) -> ChatEvent {
        let mut event = ChatEvent::None;
        let colors = theme.current_colors();
        ui.add_space(4.0);
        ui.separator();
        ui.painter().hline(
            ui.max_rect().x_range(),
            ui.cursor().max.y,
            egui::Stroke::new(1.0, colors.divider),
        );
        ui.add_space(4.0);
        let chat_items = self.prepare_chat_items();
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for item in &chat_items {
                    if let Some(new_event) = self.render_chat_item(ui, item, theme) {
                        event = new_event;
                    }
                }
            });
        event
    }

    pub fn render_chat_item(
        &mut self,
        ui: &mut egui::Ui,
        item: &ChatListItem,
        theme: &NotificationTheme,
    ) -> Option<ChatEvent> {
        let mut event = None;
        let colors = theme.current_colors();
        egui::Frame::none()
            .fill(if item.is_selected {
                colors.selected_background
            } else {
                colors.background
            })
            .inner_margin(Margin::symmetric(8., 1.))
            .show(ui, |ui| {
                ui.add_space(15.);
                let response = ui.add(
                    egui::Button::new("")
                        .frame(false)
                        .fill(Color32::TRANSPARENT)
                        .min_size(egui::vec2(ui.available_width(), 60.)),
                );

                let content_rect = response.rect;
                ui.allocate_ui_at_rect(content_rect, |ui| {
                    self.render_chat_item_content(ui, item, &response, theme);
                });

                // 右键菜单
                response.context_menu(|ui| {
                    let colors = theme.current_colors();

                    let mut visuals = ui.style().visuals.clone();
                    visuals.window_fill = colors.background;
                    visuals.window_stroke = egui::Stroke::new(1.0, colors.border);
                    visuals.widgets.inactive.weak_bg_fill = colors.background;
                    visuals.widgets.hovered.weak_bg_fill = colors.hover;
                    visuals.widgets.active.weak_bg_fill = colors.selected;

                    ui.ctx().set_visuals(visuals);

                    let menu_items = [
                        "置顶",
                        "标为未读",
                        "标记",
                        "新建标签",
                        "消息免打扰",
                        "完成",
                        "在导航栏打开",
                        "在独立窗口打开",
                    ];

                    for menu_item in menu_items {
                        let text = egui::RichText::new(menu_item)
                            .font(theme.fonts.content.clone())
                            .color(theme.text_styles.chat_message.color);

                        if ui
                            .add(
                                egui::Button::new(text)
                                    .frame(false)
                                    .fill(egui::Color32::TRANSPARENT),
                            )
                            .clicked()
                        {
                            ui.close_menu();
                        }
                    }
                });

                if response.clicked() {
                    event = Some(self.controller.handle_click(item.id.clone()));
                };
            });
        event
    }

    fn get_avatar_color(&self, avatar: &str) -> Color32 {
        let index = avatar.bytes().fold(0usize, |acc, b| {
            acc.wrapping_add(b as usize) % AVATAR_COLORS.len()
        });
        AVATAR_COLORS[index]
    }

    pub fn render_chat_item_content(
        &mut self,
        ui: &mut egui::Ui,
        item: &ChatListItem,
        response: &egui::Response,
        theme: &NotificationTheme,
    ) {
        ui.horizontal(|ui| {
            let avatar_size = egui::vec2(40., 40.);
            let avatar_response = ui.add(
                egui::Button::new(
                    egui::RichText::new(&item.avatar)
                        .color(Color32::WHITE)
                        .strong(),
                )
                .frame(true)
                .rounding(20.)
                .fill(self.get_avatar_color(&item.avatar))
                .min_size(avatar_size),
            );

            // 渲染未读计数标记
            if let Some(count) = item.unread_count {
                if count > 0 {
                    let badge_size = 16.;
                    let badge_pos = BadgePosition::DEFAULT.calculate_pos(avatar_response.rect);
                    ui.painter().circle_filled(
                        badge_pos,
                        badge_size / 2.,
                        theme.text_styles.chat_unread.color,
                    );
                    ui.painter().text(
                        badge_pos,
                        egui::Align2::CENTER_CENTER,
                        count.to_string(),
                        theme.fonts.label.clone(),
                        theme.text_styles.count.color,
                    );
                }
            }

            ui.add_space(6.);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 2.;
                self.render_chat_item_detail(ui, item, response, theme);
            })
        });
    }

    pub fn render_chat_item_detail(
        &self,
        ui: &mut egui::Ui,
        item: &ChatListItem,
        response: &egui::Response,
        theme: &NotificationTheme,
    ) {
        ui.horizontal(|ui| {
            // 聊天名称
            ui.label(
                egui::RichText::new(&item.name)
                    .font(theme.fonts.title.clone())
                    .color(theme.text_styles.chat_title.color)
                    .strong(),
            );

            // 时间戳
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("12:33")
                        .font(theme.fonts.timestamp.clone())
                        .color(theme.text_styles.chat_time.color),
                );
            });
        });

        // 最后一条消息
        if let Some(msg) = &item.last_message {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(msg)
                        .font(theme.fonts.content.clone())
                        .color(theme.text_styles.chat_message.color),
                );

                if response.hovered() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // 已读标记
                        ui.label(
                            egui::RichText::new("✓")
                                .font(theme.fonts.icon.clone())
                                .color(theme.text_styles.chat_message.color),
                        );

                        // 置顶标记
                        if item.is_pinned {
                            ui.label(
                                egui::RichText::new("📌")
                                    .font(theme.fonts.icon.clone())
                                    .color(theme.text_styles.chat_message.color),
                            );
                        }
                    });
                }
            });
        }
    }
}
