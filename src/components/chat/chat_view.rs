use bevy::prelude::ResMut;
use bevy_egui::egui::{
    self, popup_below_widget, vec2, Align, Button, Color32, Context, Frame, Id, InnerResponse, Key,
    Label, Layout, Margin, PopupCloseBehavior, Response, RichText, Rounding, ScrollArea, Sense,
    SidePanel, TextEdit, Ui, Vec2,
};
use chrono::Local;
use std::collections::HashMap;

use crate::resources::{ChatTab, NotificationTheme, UiState};

use super::{
    Chat, ChatMainStyle, ChatMessage, ChatType, CodeMessageRenderer, EmojiMessageRenderer,
    FileMessageRenderer, ImageMessageRenderer, MessageRenderer, MessageType, TextMessageRenderer,
    ToolBarButton, ToolbarAction,
};

pub struct ChatMainView {
    pub style: ChatMainStyle,
    pub message_renderers: HashMap<MessageType, Box<dyn MessageRenderer>>,
    pub toolbar_buttons: Vec<ToolBarButton>,
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
                icon: "\u{e6a2}",
                tooltip: "表情",
                action: ToolbarAction::ToggleEmoji,
            },
            ToolBarButton {
                icon: "\u{e81e}",
                tooltip: "提及",
                action: ToolbarAction::None,
            },
            ToolBarButton {
                icon: "\u{e6a3}",
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
    fn create_frame(&self, ctx: &Context, theme: &NotificationTheme) -> Frame {
        let colors = theme.current_colors();
        Frame {
            rounding: theme.style.rounding,
            inner_margin: theme.style.margin,
            shadow: ctx.style().visuals.window_shadow,
            fill: colors.background,
            ..Default::default()
        }
    }
    pub fn render(
        &self,
        ctx: &Context,
        ui_state: &mut ResMut<UiState>,
        theme: &mut ResMut<NotificationTheme>,
    ) -> InnerResponse<()> {
        SidePanel::left("chat_view_ui")
            .frame(self.create_frame(ctx, &theme))
            .resizable(false)
            .min_width(600.)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.render_header(ui, ui_state, &theme);
                    ui.separator();
                    Frame::none().show(ui, |ui| match ui_state.current_tab {
                        ChatTab::Message => self.render_message_content(ui, ui_state, theme),
                        ChatTab::Document => self.render_document_content(ui, ui_state, theme),
                        ChatTab::Announcement => {
                            self.render_announcement_content(ui, ui_state, theme)
                        }
                        ChatTab::Pin => self.render_pin_content(ui, ui_state, theme),
                        ChatTab::File => self.render_file_content(ui, ui_state, theme),
                        _ => {}
                    });
                });
            })
    }
    fn render_header(
        &self,
        ui: &mut Ui,
        ui_state: &mut ResMut<UiState>,
        theme: &NotificationTheme,
    ) {
        let _current_tab = ui_state.current_tab.clone();
        if let Some(chat) = ui_state
            .chats
            .iter()
            .find(|c| c.id == ui_state.select_chat_id)
        {
            let chat = chat.clone();

            ui.horizontal(|ui| {
                // ui.set_height(12.0);
                self.render_left_section(ui, &chat, ui_state, &theme);
                self.render_right_toolbar(ui, ui_state);
            });
        }
    }
    fn render_left_section(
        &self,
        ui: &mut Ui,
        chat: &Chat,
        ui_state: &mut ResMut<UiState>,
        theme: &NotificationTheme,
    ) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add_space(10.0);
                self.render_avatar(ui, chat, &theme);
                ui.add_space(1.);
                self.render_chat_info(ui, chat, ui_state, &theme);
            });
        });
    }

    fn render_avatar(&self, ui: &mut Ui, chat: &Chat, theme: &NotificationTheme) {
        ui.add(
            Button::new(RichText::new(&chat.avatar).color(Color32::WHITE).strong())
                .rounding(25.)
                .fill(self.get_avatar_color(&chat.avatar))
                .min_size(Vec2::new(40., 40.)),
        );
    }

    fn render_chat_info(
        &self,
        ui: &mut Ui,
        chat: &Chat,
        ui_state: &mut ResMut<UiState>,
        theme: &NotificationTheme,
    ) {
        ui.vertical(|ui| {
            self.render_chat_header(ui, chat, theme);
            ui.add_space(8.);
            self.render_tabs(ui, ui_state, theme);
        });
    }
    fn render_chat_header(&self, ui: &mut Ui, chat: &Chat, theme: &NotificationTheme) {
        ui.horizontal(|ui| {
            ui.heading(
                RichText::new(&chat.name)
                    .strong()
                    .font(theme.fonts.title.clone())
                    .color(theme.text_styles.chat_title.color),
            );
            self.render_chat_type_indicator(ui, chat, theme);
        });
    }
    fn render_chat_type_indicator(&self, ui: &mut Ui, chat: &Chat, theme: &NotificationTheme) {
        match chat.chat_type {
            ChatType::Group => {
                ui.add_space(5.);
                ui.small(
                    RichText::new("\u{e748}")
                        .size(14.)
                        .color(theme.text_styles.chat_title.color),
                );
                ui.label(
                    RichText::new(format!("{}", &chat.member_count))
                        .size(12.)
                        .color(theme.text_styles.chat_title.color),
                );
            }
            ChatType::Bot => {
                ui.add_space(5.);
                ui.small(
                    RichText::new("\u{e8bb}")
                        .size(14.)
                        .color(theme.text_styles.chat_title.color),
                );
            }
            _ => {}
        }
    }

    fn render_tabs(&self, ui: &mut Ui, ui_state: &mut ResMut<UiState>, theme: &NotificationTheme) {
        let current_tab = ui_state.current_tab.clone();
        ui.horizontal(|ui| {
            self.render_tab_button(ui, current_tab, ui_state, theme);
            self.render_add_tab_button(ui, theme);
        });
    }

    fn render_add_tab_button(&self, ui: &mut Ui, theme: &NotificationTheme) {
        let text = RichText::new("➕").color(theme.text_styles.chat_message.color);
        let add_btn = Button::new(text).frame(false).fill(Color32::TRANSPARENT);
        let add_btn_response = ui.add(add_btn);
        let popup_id = ui.make_persistent_id("add_tab_menu");
        if add_btn_response.clicked() {
            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
        }

        self.render_add_tab_button_popup(ui, popup_id, &add_btn_response, theme);
    }

    fn render_add_tab_button_popup(
        &self,
        ui: &mut Ui,
        popup_id: Id,
        add_btn_response: &Response,
        theme: &NotificationTheme,
    ) {
        let colors = theme.current_colors();
        let mut visuals = ui.style().visuals.clone();
        visuals.window_fill = colors.background;
        visuals.window_stroke = egui::Stroke::new(1.0, colors.border);
        visuals.widgets.inactive.weak_bg_fill = colors.background;
        visuals.widgets.hovered.weak_bg_fill = colors.hover;
        visuals.widgets.active.weak_bg_fill = colors.selected;
        ui.ctx().set_visuals(visuals);
        let mut should_close = false;
        popup_below_widget(
            ui,
            popup_id,
            add_btn_response,
            PopupCloseBehavior::CloseOnClick,
            |ui| {
                ui.set_min_width(80.);
                ui.style_mut().wrap = Some(false);

                let text_style = theme.text_styles.chat_message.clone();
                let font = theme.fonts.content.clone();

                let add_text = RichText::new("添加标签")
                    .font(font.clone())
                    .color(text_style.color);

                let manage_text = RichText::new("管理标签").font(font).color(text_style.color);

                if ui.button(add_text).clicked() {
                    should_close = true;
                }
                if ui.button(manage_text).clicked() {
                    should_close = true;
                }
            },
        );

        if should_close {
            ui.memory_mut(|mem| mem.close_popup());
        }
    }
    fn render_tab_button(
        &self,
        ui: &mut Ui,
        current_tab: ChatTab,
        ui_state: &mut ResMut<UiState>,
        theme: &NotificationTheme,
    ) {
        let tabs = vec![
            (ChatTab::Message, "\u{ebb4} 消息"),
            (ChatTab::Document, "\u{ebb5} 云文档"),
            (ChatTab::Announcement, "\u{e69a} 群公告"),
            (ChatTab::Pin, "\u{e9f2} Pin"),
            (ChatTab::File, "\u{e6fc} 文件"),
        ];

        ui.horizontal(|ui| {
            for (tab, label) in tabs {
                ui.add_space(5.);
                self.render_single_tab(ui, tab, label, current_tab.clone(), ui_state, theme);
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
        theme: &NotificationTheme,
    ) {
        let colors = theme.current_colors();
        let is_selected = current_tab == tab;

        let text = RichText::new(label)
            .size(12.)
            .font(theme.fonts.content.clone())
            .color(if is_selected {
                theme.text_styles.chat_title.selected_color
            } else {
                theme.text_styles.chat_title.color
            });

        let btn = Button::new(text)
            .frame(false)
            .fill(if is_selected {
                colors.selected_background
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

    fn render_document_content(
        &self,
        ui: &mut Ui,
        ui_state: &mut ResMut<UiState>,
        theme: &mut ResMut<NotificationTheme>,
    ) {
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

    fn render_announcement_content(
        &self,
        ui: &mut Ui,
        ui_state: &mut ResMut<UiState>,
        theme: &mut ResMut<NotificationTheme>,
    ) {
        ui.add_space(10.);
        ui.heading("群公告");
        ui.add_space(8.0);

        if ui.button("发布新公告").clicked() {
            //todo
        }
    }

    fn render_pin_content(
        &self,
        ui: &mut Ui,
        ui_state: &mut ResMut<UiState>,
        theme: &mut ResMut<NotificationTheme>,
    ) {
        ui.add_space(10.);
        ui.heading("置顶消息");
        ui.add_space(8.0);
    }

    fn render_file_content(
        &self,
        ui: &mut Ui,
        ui_state: &mut ResMut<UiState>,
        theme: &mut ResMut<NotificationTheme>,
    ) {
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
            ("\u{eb2b}", "日历"),
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
    pub fn get_avatar_color(&self, text: &str) -> Color32 {
        let index = text
            .bytes()
            .fold(0usize, |acc, b| acc.wrapping_add(b as usize))
            % self.style.colors.avatar_colors.len();
        self.style.colors.avatar_colors[index]
    }
}
