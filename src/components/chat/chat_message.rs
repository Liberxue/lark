use bevy::prelude::ResMut;
use bevy_egui::egui::{
    self, popup_below_widget, vec2, Align, Button, Color32, Context, Frame, Id, InnerResponse, Key,
    Label, Layout, Margin, PopupCloseBehavior, Response, RichText, Rounding, ScrollArea, Sense,
    SidePanel, TextEdit, Ui, Vec2,
};
use chrono::Local;
use std::collections::HashMap;

use crate::resources::{ChatTab, NotificationTheme, UiState};

use super::ChatMainView;
use super::{
    Chat, ChatMainStyle, ChatMessage, ChatType, CodeMessageRenderer, EmojiMessageRenderer,
    FileMessageRenderer, ImageMessageRenderer, MessageRenderer, MessageType, TextMessageRenderer,
    ToolBarButton, ToolbarAction,
};

impl ChatMainView {
    pub fn render_message_content(
        &self,
        ui: &mut Ui,
        ui_state: &mut ResMut<UiState>,
        theme: &mut ResMut<NotificationTheme>,
    ) {
        self.render_messages(ui, ui_state, theme);
        ui.separator();
        self.render_input_area(ui, ui_state, theme);
    }

    fn render_messages(&self, ui: &mut Ui, ui_state: &mut UiState, theme: &NotificationTheme) {
        let available_height = ui.available_height();
        let chat_area_height = available_height - 100.;
        let mut last_date: Option<String> = None;
        let mut last_sender: Option<(String, String)> = None;
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .max_height(chat_area_height)
            .show(ui, |ui| {
                for (_idx, message) in ui_state.messages.iter().enumerate() {
                    let date = message
                        .timestamp
                        .split(' ')
                        .next()
                        .unwrap_or("")
                        .to_string();

                    if last_date.as_ref().map_or(true, |last| last != &date) {
                        ui.vertical_centered(|ui| {
                            ui.add_space(5.0);
                            ui.add(Label::new(
                                RichText::new(&date)
                                    .color(theme.text_styles.chat_time.color)
                                    .font(theme.fonts.timestamp.clone())
                                    .size(12.0),
                            ));
                            ui.add_space(5.0);
                        });
                        last_date = Some(date);
                    }

                    let minute = message
                        .timestamp
                        .split(' ')
                        .nth(1)
                        .and_then(|t| t.rsplitn(2, ':').last())
                        .unwrap_or("")
                        .to_string();

                    let show_avatar = last_sender.as_ref().map_or(true, |(sender, min)| {
                        &message.sender != sender || &minute != min
                    });

                    if show_avatar {
                        last_sender = Some((message.sender.clone(), minute.clone()));
                    }

                    self.render_message(ui, message, show_avatar, theme);
                }
            });
    }

    fn render_message(
        &self,
        ui: &mut Ui,
        message: &ChatMessage,
        show_avatar: bool,
        theme: &NotificationTheme,
    ) {
        let parts: Vec<&str> = message.timestamp.split(' ').collect();
        let (date, time) = if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            (message.timestamp.as_str(), "xxx")
        };

        Frame::none().show(ui, |ui| {
            let response =
                ui.allocate_response(Vec2::new(ui.available_width(), 10.0), Sense::hover());
            let is_hovered = response.hovered();

            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    if show_avatar {
                        ui.add_space(25.0);
                        ui.add(
                            Button::new(
                                RichText::new(&message.avatar)
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .rounding(35.0)
                            .fill(self.get_avatar_color(&message.avatar))
                            .min_size(Vec2::new(35.0, 35.0)),
                        );
                    } else {
                        ui.add_space(5.0);
                        ui.add(
                            Button::new(
                                RichText::new(time)
                                    .color(if is_hovered && !show_avatar {
                                        theme.text_styles.chat_time.color
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .font(theme.fonts.timestamp.clone())
                                    .size(12.5)
                                    .strong(),
                            )
                            .fill(Color32::TRANSPARENT),
                        );
                    }
                });

                ui.vertical(|ui| {
                    if show_avatar {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(&message.sender)
                                    .font(theme.fonts.title.clone())
                                    .color(theme.text_styles.chat_title.color)
                                    .strong(),
                            );
                            if is_hovered {
                                let display_time = format!(
                                    "{} {}",
                                    date.split('.').skip(1).collect::<Vec<_>>().join("."),
                                    time
                                );
                                ui.label(
                                    RichText::new(&display_time)
                                        .color(theme.text_styles.chat_time.color)
                                        .font(theme.fonts.timestamp.clone())
                                        .size(13.),
                                );
                            }
                        });
                    }

                    // 消息框
                    Frame::none()
                        .fill(Color32::from_rgba_unmultiplied(0x24, 0x24, 0x24, 245))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::LEFT).with_main_wrap(true),
                                |ui| {
                                    if let Some(renderer) =
                                        self.message_renderers.get(&message.message_type)
                                    {
                                        renderer.render(ui, message, &self.style, theme);
                                    }
                                },
                            );
                        });
                });
            });
        });
    }
    fn render_input_area(
        &self,
        ui: &mut Ui,
        ui_state: &mut UiState,
        theme: &mut ResMut<NotificationTheme>,
    ) {
        Frame::none().outer_margin(vec2(1.0, 1.0)).show(ui, |ui| {
            ui.vertical(|ui| {
                self.render_toolbar(ui, ui_state);
                if ui_state.show_emoji_picker {
                    // TODO: Implement emoji picker
                }
                self.render_input(ui, ui_state, theme);
            });
        });
    }

    fn render_toolbar(&self, ui: &mut Ui, ui_state: &mut UiState) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for button in &self.toolbar_buttons {
                let btn = ui.add(
                    Button::new(button.icon)
                        .frame(false)
                        .min_size(vec2(24.0, 24.0)),
                );
                if btn.clicked() {
                    match &button.action {
                        ToolbarAction::ToggleEmoji => {
                            ui_state.show_emoji_picker = !ui_state.show_emoji_picker;
                        }
                        ToolbarAction::SetMessageType(msg_type) => {
                            ui_state.current_message_type = msg_type.clone();
                        }
                        ToolbarAction::None => {}
                    }
                }
                if btn.hovered() {
                    btn.on_hover_ui(|ui| {
                        ui.label(button.tooltip);
                    });
                }
            }
        });
    }

    fn render_input(&self, ui: &mut Ui, ui_state: &mut UiState, theme: &NotificationTheme) {
        let mut visuals = ui.ctx().style().visuals.clone();
        let colors = theme.current_colors();

        visuals.extreme_bg_color = colors.background;
        visuals.widgets.inactive.bg_fill = colors.background;
        visuals.widgets.active.bg_fill = colors.background;
        visuals.widgets.hovered.bg_fill = colors.hover;

        ui.ctx().set_visuals(visuals);

        let frame = Frame::none()
            .outer_margin(vec2(4.0, 4.0))
            .inner_margin(vec2(4.0, 4.0));

        frame.show(ui, |ui| {
            let text_edit = TextEdit::multiline(&mut ui_state.input_text)
                .desired_width(ui.available_width())
                .desired_rows(1)
                .min_size(vec2(0.0, 30.0))
                .hint_text(RichText::new("输入消息...").color(theme.text_styles.chat_message.color))
                .text_color(theme.text_styles.chat_message.color)
                .frame(true);

            let _response = ui.add(text_edit);
            let enter_pressed = ui.input(|i| i.key_pressed(Key::Enter) && !i.modifiers.shift);

            if enter_pressed && !ui_state.input_text.is_empty() {
                self.send_message(ui_state);
            }
        });
    }

    fn send_message(&self, ui_state: &mut UiState) {
        let now = Local::now().format("%Y.%m.%d %H:%M:%S").to_string();

        let trimmed_text = ui_state.input_text.trim().to_string();

        if !trimmed_text.is_empty() {
            ui_state.messages.push(ChatMessage {
                id: format!("msg_{}", now),
                is_bot: false,
                chat_id: ui_state.select_chat_id.clone(),
                sender: "You".to_string(),
                avatar: "Y".to_string(),
                content: trimmed_text,
                timestamp: now,
                message_type: ui_state.current_message_type.clone(),
                reactions: Vec::new(),
            });
        }
        ui_state.input_text.clear();
    }
}
