use super::{controller::ChatListController, ChatEvent, ChatListItem};
use bevy_egui::egui::{self, Margin};

pub struct ChatListView<'a> {
    controller: &'a mut ChatListController,
}

struct BadgePostion {
    offset_x: f32,
    offset_y: f32,
}

impl BadgePostion {
    const DEFAULT: Self = Self {
        offset_x: 0.9,
        offset_y: 0.1,
    };

    fn calculate_post(&self, avatar_rect: egui::Rect) -> egui::Pos2 {
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

    pub fn render(&mut self, ui: &mut egui::Ui) -> ChatEvent {
        let mut event = ChatEvent::None;
        ui.separator();
        let chat_items = self.prepare_chat_items();
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for item in &chat_items {
                    if let Some(new_event) = self.render_chat_item(ui, item) {
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
    ) -> Option<ChatEvent> {
        let mut event = None;
        egui::Frame::none()
            .fill(if item.is_selected {
                egui::Color32::from_rgb(0x33, 0x33, 0x33)
            } else {
                egui::Color32::TRANSPARENT
            })
            .inner_margin(Margin::symmetric(8., 1.))
            .show(ui, |ui| {
                ui.add_space(15.);
                let response = ui.add(
                    egui::Button::new("")
                        .frame(false)
                        .fill(egui::Color32::TRANSPARENT)
                        .min_size(egui::vec2(ui.available_width(), 60.)),
                );

                let content_rect = response.rect;
                ui.allocate_ui_at_rect(content_rect, |ui| {
                    self.render_chat_item_content(ui, item, &response);
                });

                response.context_menu(|ui| {
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
                    for item in menu_items {
                        if ui.button(item).clicked() {
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
    fn get_avatar_color(&self, avatar: &str) -> egui::Color32 {
        const COLORS: &[egui::Color32] = &[
            egui::Color32::from_rgb(103, 58, 183),
            egui::Color32::from_rgb(63, 81, 181),
            egui::Color32::from_rgb(33, 150, 243),
            egui::Color32::from_rgb(3, 169, 244),
            egui::Color32::from_rgb(0, 188, 212),
            egui::Color32::from_rgb(0, 150, 136),
            egui::Color32::from_rgb(76, 175, 80),
            egui::Color32::from_rgb(244, 67, 54),
            egui::Color32::from_rgb(233, 30, 99),
            egui::Color32::from_rgb(156, 39, 176),
        ];
        let index = avatar
            .bytes()
            .fold(0usize, |acc, b| acc.wrapping_add(b as usize) % COLORS.len());
        COLORS[index]
    }
    pub fn render_chat_item_content(
        &mut self,
        ui: &mut egui::Ui,
        item: &ChatListItem,
        response: &egui::Response,
    ) {
        ui.horizontal(|ui| {
            let avatar_size = egui::vec2(40., 40.);
            let avatar_response = ui.add(
                egui::Button::new(
                    egui::RichText::new(&item.avatar)
                        .color(egui::Color32::WHITE)
                        .size(20.)
                        .strong(),
                )
                .frame(true)
                .rounding(20.)
                .fill(self.get_avatar_color(&item.avatar))
                .min_size(avatar_size),
            );
            if let Some(count) = item.unread_count {
                if count > 0 {
                    let badge_size = 16.;
                    let badge_pos = BadgePostion::DEFAULT.calculate_post(avatar_response.rect);
                    ui.painter().circle_filled(
                        badge_pos,
                        badge_size / 2.,
                        egui::Color32::from_rgb(255, 92, 92),
                    );
                    ui.painter().text(
                        badge_pos,
                        egui::Align2::CENTER_CENTER,
                        count.to_string(),
                        egui::FontId::proportional(10.),
                        egui::Color32::WHITE,
                    );
                }
            }
            ui.add_space(6.);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 2.;
                self.render_chat_item_detail(ui, item, &response);
            })
        });
    }

    pub fn render_chat_item_detail(
        &self,
        ui: &mut egui::Ui,
        item: &ChatListItem,
        response: &egui::Response,
    ) {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(&item.name)
                    .color(egui::Color32::WHITE)
                    .size(14.)
                    .strong(),
            ); // mock error...
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("12:33")
                        .size(13.)
                        .color(egui::Color32::GRAY),
                );
            });
        });
        if let Some(msg) = &item.last_message {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(msg)
                        .size(12.)
                        .color(egui::Color32::GRAY),
                );
                if response.hovered() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("✓")
                                .size(16.)
                                .color(egui::Color32::WHITE),
                        );
                        if item.is_pinned {
                            ui.label("📌");
                        }
                    });
                };
            });
        };
    }
}
