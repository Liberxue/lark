use std::collections::HashMap;

use bevy::{prelude::ResMut, utils::warn};
use bevy_egui::egui::{
    self, popup_below_widget, vec2, Align, Button, Color32, Context, Frame, Id, InnerResponse, Key,
    Label, Layout, Margin, PopupCloseBehavior, Response, RichText, Rounding, ScrollArea, Sense,
    SidePanel, TextEdit, Ui, Vec2,
};
use chrono::Local;

use crate::resources::{ChatTab, UiState};

use super::{
    Chat, ChatMainStyle, ChatMessage, ChatType, CodeMessageRenderer, EmojiMessageRenderer,
    FileMessageRenderer, ImageMessageRenderer, MessageRenderer, MessageType, TextMessageRenderer,
    ToolBarButton, ToolbarAction,
};

pub struct ChatMainView {
    style: ChatMainStyle,
    message_renderers: HashMap<MessageType, Box<dyn MessageRenderer>>,
    toolbar_buttons: Vec<ToolBarButton>,
}
impl ChatMainView {
    pub fn new() -> Self {
        let mut message_renderers: HashMap<MessageType, Box<dyn MessageRenderer>> = HashMap::new();

        message_renderers.insert(
            MessageType::Text,
            Box::new(TextMessageRenderer) as Box<dyn MessageRenderer>,
        );
        message_renderers.insert(
            MessageType::Code,
            Box::new(CodeMessageRenderer) as Box<dyn MessageRenderer>,
        );
        message_renderers.insert(
            MessageType::File,
            Box::new(FileMessageRenderer) as Box<dyn MessageRenderer>,
        );
        message_renderers.insert(
            MessageType::Images,
            Box::new(ImageMessageRenderer) as Box<dyn MessageRenderer>,
        );
        //  message_renderers.insert(
        //      MessageType::Emoji,
        //      Box::new(EmojiMessageRenderer) as Box<dyn MessageRenderer>,
        //  );
        let toolbar_buttons = vec![
            ToolBarButton {
                icon: "\u{e601}",
                tooltip: "表情",
                action: ToolbarAction::ToggleEmoji,
            },
            ToolBarButton {
                icon: "\u{e81e}",
                tooltip: "提及",
                action: ToolbarAction::None,
            },
            ToolBarButton {
                icon: "\u{e600}",
                tooltip: "附件",
                action: ToolbarAction::SetMessageType(MessageType::File),
            },
            ToolBarButton {
                icon: "\u{e6af}",
                tooltip: "代码块",
                action: ToolbarAction::SetMessageType(MessageType::Code),
            },
            ToolBarButton {
                icon: "\u{e854}",
                tooltip: "文本",
                action: ToolbarAction::SetMessageType(MessageType::Text),
            },
        ];
        Self {
            style: ChatMainStyle::default(),
            message_renderers,
            toolbar_buttons,
        }
    }
    fn create_frame(&self, ctx: &Context) -> Frame {
        Frame {
            rounding: self.style.rounding.clone(),
            inner_margin: self.style.margin.clone(),
            shadow: ctx.style().visuals.window_shadow,
            fill: self.style.colors.background,
            ..Default::default()
        }
    }
    pub fn render(&self, ctx: &Context, ui_state: &mut ResMut<UiState>) -> InnerResponse<()> {
        SidePanel::left("chat_main_ui")
            .frame(self.create_frame(ctx))
            .resizable(false)
            .default_width(1000.0)
            //.width_range(1000.0..=1280.0)
            .min_width(600.)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.render_header(ui, ui_state);
                    ui.separator();
                    Frame::none().show(ui, |ui| match ui_state.current_tab {
                        ChatTab::Message => self.render_message_content(ui, ui_state),
                        ChatTab::Document => self.render_document_content(ui, ui_state),
                        ChatTab::Announcement => self.render_announcement_content(ui, ui_state),
                        ChatTab::Pin => self.render_pin_content(ui, ui_state),
                        ChatTab::File => self.render_file_content(ui, ui_state),
                        _ => {}
                    });
                });
            })
    }
    fn render_header(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
        let _current_tab = ui_state.current_tab.clone();
        if let Some(chat) = ui_state
            .chats
            .iter()
            .find(|c| c.id == ui_state.select_chat_id)
        {
            let chat = chat.clone();

            ui.horizontal(|ui| {
                ui.set_height(14.0);
                self.render_left_section(ui, &chat, ui_state);
                self.render_right_toolbar(ui, ui_state);
            });
        }
    }
    fn render_left_section(&self, ui: &mut Ui, chat: &Chat, ui_state: &mut ResMut<UiState>) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                self.render_avatar(ui, chat);
                ui.add_space(1.);
                self.render_chat_info(ui, chat, ui_state);
            });
        });
    }
    fn render_avatar(&self, ui: &mut Ui, chat: &Chat) {
        ui.add(
            Button::new(RichText::new(&chat.avatar).color(Color32::WHITE).strong())
                .rounding(25.)
                .fill(self.get_avatar_color(&chat.avatar))
                .min_size(Vec2::new(40., 40.)),
        );
    }
    fn render_chat_info(&self, ui: &mut Ui, chat: &Chat, ui_state: &mut ResMut<UiState>) {
        ui.vertical(|ui| {
            self.render_chat_header(ui, chat);
            ui.add_space(5.);
            self.render_tabs(ui, ui_state);
        });
    }
    fn render_chat_header(&self, ui: &mut Ui, chat: &Chat) {
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new(&chat.name)
                    .strong()
                    .font(self.style.fonts.title.clone()),
            );
            self.render_chat_type_indicator(ui, chat);
        });
    }
    fn render_chat_type_indicator(&self, ui: &mut Ui, chat: &Chat) {
        match chat.chat_type {
            ChatType::Group => {
                ui.add_space(5.);
                ui.small("\u{e748}");
                ui.label(RichText::new(format!("{}", &chat.member_count)).size(12.));
            }
            ChatType::Bot => {
                ui.add_space(5.);
                ui.small("\u{e8bb}");
            }
            _ => {}
        }
    }
    fn render_tabs(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
        let current_tab = ui_state.current_tab.clone();
        ui.horizontal(|ui| {
            self.render_tab_button(ui, current_tab, ui_state);
            self.render_add_tab_button(ui);
        });
    }
    fn render_add_tab_button(&self, ui: &mut Ui) {
        let add_btn = Button::new("➕").frame(false).fill(Color32::TRANSPARENT);
        let add_btn_response = ui.add(add_btn);
        let popup_id = ui.make_persistent_id("add_tab_menu");
        if add_btn_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id))
        }
    }
    fn render_add_tab_button_popup(&self, ui: &mut Ui, popup_id: Id, add_btn_response: &Response) {
        let mut shound_close = false;
        popup_below_widget(
            ui,
            popup_id,
            add_btn_response,
            PopupCloseBehavior::CloseOnClick,
            |ui| {
                ui.set_min_width(80.);
                ui.style_mut().wrap = Some(false);
                if ui.button("添加标签").clicked() {
                    shound_close = true;
                }
                if ui.button("管理标签").clicked() {
                    shound_close = true;
                }
            },
        );
        if shound_close {
            ui.memory_mut(|mem| mem.close_popup());
        }
    }
    fn render_tab_button(&self, ui: &mut Ui, current_tab: ChatTab, ui_state: &mut ResMut<UiState>) {
        let tabs = vec![
            (ChatTab::Message, "\u{ebb4} 消息"),
            (ChatTab::Document, "\u{e630} 云文档"),
            (ChatTab::Announcement, "\u{e69a} 群公告"),
            (ChatTab::Pin, "\u{e9f2} Pin"),
            (ChatTab::File, "\u{e6fc} 文件"),
        ];

        ui.horizontal(|ui| {
            for (tab, label) in tabs {
                self.render_single_tab(ui, tab, label, current_tab.clone(), ui_state);
                ui.add_space(5.);
            }
        });
    }

    fn render_single_tab(
        &self,
        ui: &mut Ui,
        tab: ChatTab,
        label: &str,
        current_tab: ChatTab,
        ui_state: &mut ResMut<UiState>,
    ) {
        let is_selected = current_tab == tab;
        let btn = Button::new(RichText::new(label).size(12.))
            .frame(false)
            .fill(if is_selected {
                Color32::from_rgb(0x35, 0x44, 0x66)
            } else {
                Color32::TRANSPARENT
            })
            .rounding(if is_selected { 5. } else { 0. });

        if ui.add(btn).clicked() {
            ui_state.current_tab = tab;
        }
    }
    fn render_right_toolbar(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_space(10.);
            self.render_more_menu(ui);
            self.render_tool_button(ui);
        });
    }

    fn render_message_content(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
        self.render_messages(ui, ui_state);
        ui.separator();
        self.render_input_area(ui, ui_state);
    }

    fn render_document_content(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
        ui.vertical(|ui| {
            ui.add_space(10.);
            ui.horizontal(|ui| {
                ui.add_space(10.);
                ui.heading("云文档");
                ui.add_space(ui.available_width() - 150.0);
                if ui.button("新建文档").clicked() {
                    //TODO
                }
            });

            ui.add_space(8.0);

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for i in 0..15 {
                        ui.horizontal(|ui| {
                            ui.add_space(20.);
                            ui.label(format!("文档 {}", i));
                            ui.add_space(ui.available_width() - 100.0);
                            if ui.button("查看").clicked() {
                                //TODO
                            }
                        });
                        ui.add_space(4.0);
                    }
                });
        });
    }

    fn render_announcement_content(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
        ui.add_space(10.);
        ui.heading("群公告");
        ui.add_space(8.0);

        if ui.button("发布新公告").clicked() {
            //todo
        }
    }

    fn render_pin_content(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
        ui.add_space(10.);
        ui.heading("置顶消息");
        ui.add_space(8.0);
    }

    fn render_file_content(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>) {
        ui.vertical(|ui| {
            ui.add_space(10.);
            ui.horizontal(|ui| {
                ui.add_space(20.);
                ui.heading("文件管理");
                ui.add_space(ui.available_width() - 150.0);
                if ui.button("上传文件").clicked() {
                    //TODO
                }
            });

            ui.add_space(8.0);

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for i in 0..10 {
                        ui.horizontal(|ui| {
                            ui.add_space(10.);
                            ui.label(format!("文件 {}", i));
                            ui.add_space(ui.available_width() - 100.0);
                            if ui.button("下载").clicked() {
                                //todo
                            }
                        });
                        ui.add_space(4.0);
                    }
                });
        });
    }

    fn render_more_menu(&self, ui: &mut Ui) {
        let more_btn = Button::new("...").frame(false);
        let more_btn_response = ui.add(more_btn);
        let popup_id = ui.make_persistent_id("more_menu");

        if more_btn_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }
        let mut shound_close = false;
        popup_below_widget(
            ui,
            popup_id,
            &more_btn_response,
            PopupCloseBehavior::CloseOnClick,
            |ui| {
                ui.set_min_width(80.);
                ui.style_mut().wrap = Some(false);
                if ui.button("查看任务").clicked() {
                    shound_close = true;
                }
                ui.separator();
                if ui.button("设置").clicked() {
                    ui.memory_mut(|mem| mem.close_popup());
                }
                ui.separator();
            },
        );
        if shound_close {
            ui.memory_mut(|mem| mem.close_popup());
        }
    }
    fn render_tool_button(&self, ui: &mut Ui) {
        for (icon, tooltip) in [
            ("\u{e71a}", "搜索会话记录"),
            ("\u{e662}", "视频会议"),
            ("\u{e777}", "添加新成员"),
            ("\u{ebb5}", "日历"),
            ("\u{e748}", "群成员"),
        ] {
            let btn = ui.add(Button::new(icon).frame(false));
            if btn.clicked() {
                btn.on_hover_ui(|ui| {
                    ui.label(tooltip);
                });
            }
            ui.add_space(5.);
        }
    }

    fn render_messages(&self, ui: &mut Ui, ui_state: &mut UiState) {
        let available_height = ui.available_height();
        let chat_area_height = available_height - 100.;

        let mut last_date: Option<String> = None;
        let mut last_sender: Option<(String, String)> = None;

        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .max_height(chat_area_height)
            .show(ui, |ui| {
                for (idx, message) in ui_state.messages.iter().enumerate() {
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
                                RichText::new(&date).color(Color32::GRAY).size(12.0),
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

                    self.render_message(ui, message, show_avatar);
                }
            });
    }

    fn render_message(&self, ui: &mut Ui, message: &ChatMessage, show_avatar: bool) {
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
                                        Color32::GRAY
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .size(13.)
                                    .strong(),
                            )
                            .fill(Color32::TRANSPARENT),
                        );
                    }
                });

                ui.vertical(|ui| {
                    if show_avatar {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&message.sender).strong());
                            if is_hovered {
                                let display_time = format!(
                                    "{} {}",
                                    date.split('.').skip(1).collect::<Vec<_>>().join("."),
                                    time
                                );
                                ui.label(
                                    RichText::new(&display_time).color(Color32::GRAY).size(13.),
                                );
                            }
                        });
                    }

                    Frame::none()
                        .fill(Color32::from_rgb(0x24, 0x24, 0x24))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::same(8.0))
                        .show(ui, |ui| {
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::LEFT).with_main_wrap(true),
                                |ui| {
                                    if let Some(renderer) =
                                        self.message_renderers.get(&message.message_type)
                                    {
                                        renderer.render(ui, message, &self.style);
                                    }
                                },
                            );
                        });
                });
            });
        });
    }
    fn render_input_area(&self, ui: &mut Ui, ui_state: &mut UiState) {
        Frame::none().outer_margin(vec2(1.0, 1.0)).show(ui, |ui| {
            ui.vertical(|ui| {
                self.render_toolbar(ui, ui_state);
                if ui_state.show_emoji_picker {
                    // TODO: Implement emoji picker
                }
                self.render_input(ui, ui_state);
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

    fn render_input(&self, ui: &mut Ui, ui_state: &mut UiState) {
        let frame = Frame::none()
            .outer_margin(vec2(4.0, 4.0))
            .inner_margin(vec2(4.0, 4.0));
        frame.show(ui, |ui| {
            let text_edit = TextEdit::multiline(&mut ui_state.input_text)
                .desired_width(ui.available_width())
                .desired_rows(1)
                .min_size(vec2(0.0, 30.0))
                .hint_text("输入消息...");

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
    fn get_avatar_color(&self, text: &str) -> Color32 {
        let index = text
            .bytes()
            .fold(0usize, |acc, b| acc.wrapping_add(b as usize))
            % self.style.colors.avatar_colors.len();
        self.style.colors.avatar_colors[index]
    }
}
