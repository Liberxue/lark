use crate::{Chat, ChatMessage, ChatType, MessageType, Notification};
use std::collections::HashMap;

use bevy::prelude::{Component, Resource};
use bevy_egui::egui::Vec2;

#[derive(Debug, Clone, PartialEq)]
pub enum ChatTab {
    Message,
    Document,
    Announcement,
    Pin,
    File,
    Add,
}

impl Default for ChatTab {
    fn default() -> Self {
        Self::Message
    }
}

#[derive(Resource)]
pub struct UiState {
    pub nav_width: f32,
    pub selected_nav_index: usize,
    pub show_avatar_menu: bool,
    pub show_status_menu: bool,
    pub show_siderbar: bool,
    pub selected_siderbar_button: String,
    pub current_tab: ChatTab,
    pub search_text: String,

    //chat main ui
    pub current_message_type: MessageType,
    pub input_text: String,
    pub select_chat_id: String,
    pub show_emoji_picker: bool,
    pub show_pin_message: bool,
    // chat content
    pub messages: Vec<ChatMessage>,
    pub chats: Vec<Chat>,
    pub notifications: Vec<Notification>,
    pub unread_counts: HashMap<String, i32>,
}

impl Default for UiState {
    fn default() -> Self {
        let chat_data = ChatData::create_default_chat_rooms();
        let mut chats = Vec::new();
        let mut messages = Vec::new();
        let mut unread_counts = HashMap::new();

        for (_, room_data) in chat_data.iter() {
            chats.push(room_data.chat.clone());
            messages.extend(room_data.messages.clone());
            unread_counts.insert(room_data.chat.id.clone(), room_data.unread_count);
        }

        Self {
            nav_width: 50.0,
            selected_nav_index: 1,
            show_avatar_menu: false,
            show_status_menu: false,
            show_siderbar: false,
            select_chat_id: "1".to_string(),
            current_tab: ChatTab::Message,
            search_text: "搜索联系/文档".to_string(),
            selected_siderbar_button: String::new(),
            current_message_type: MessageType::Text,
            input_text: String::new(),
            show_emoji_picker: false,
            show_pin_message: false,
            messages,
            chats,
            notifications: vec![],
            unread_counts,
        }
    }
}

impl UiState {
    pub fn current_chat_name(&self) -> String {
        self.chats
            .iter()
            .find(|c| c.id == self.select_chat_id)
            .map(|c| c.name.clone())
            .unwrap_or_default()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NavPage {
    Message,
    VideoMeeting,
    Calendar,
    Doc,
    Contact,
    Table,
    Search,
}
impl UiState {
    pub fn current_page(&self) -> NavPage {
        match self.selected_nav_index {
            0 => NavPage::Search,
            1 => NavPage::Message,
            2 => NavPage::Calendar,
            3 => NavPage::Doc,
            4 => NavPage::VideoMeeting,
            5 => NavPage::Table,
            6 => NavPage::Contact,
            _ => NavPage::Message,
        }
    }
}

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Resource, Default, Debug, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    SplashStart,
    UiSetup,
    SplashAnimate,
    Running,
}
#[derive(Component)]
pub struct SplashCamera;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct SplashScreen {
    pub is_animating: bool,
}

#[derive(Component)]
pub struct SplashAnimation {
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub progress: f32,
    pub duration: f32,
}

#[derive(Clone)]
pub struct ChatRoomData {
    pub chat: Chat,
    pub messages: Vec<ChatMessage>,
    pub unread_count: i32,
}

impl ChatRoomData {
    pub fn new(chat: Chat, messages: Vec<ChatMessage>, unread_count: i32) -> Self {
        Self {
            chat,
            messages,
            unread_count,
        }
    }
}

pub struct ChatData {
    pub chats: Vec<Chat>,
    pub messages: Vec<ChatMessage>,
    pub notifications: Vec<Notification>,
    pub unread_counts: HashMap<String, i32>,
}

impl ChatData {
    pub fn new() -> Self {
        let mut data = Self::default();
        data.init_default_data();
        data
    }

    pub fn create_default_chat_rooms() -> HashMap<String, ChatRoomData> {
        let mut rooms = HashMap::new();
        rooms.insert(
            "1".to_string(),
            ChatRoomData::new(
                Chat {
                    id: "1".to_string(),
                    name: "Rust Stack 交流群".to_string(),
                    avatar: "R".to_string(),
                    member_count: 850,
                    last_message: Some("欢迎加入群聊".to_string()),
                    chat_type: ChatType::Group,
                    pin: true,
                },
                vec![
                    ChatMessage {
                        id: "1-1".to_string(),
                        is_bot: false,
                        chat_id: "1".to_string(),
                        sender: "张鸣".to_string(),
                        avatar: "鸣".to_string(),
                        content: "我在尝试用Rust写一个飞书客户端".to_string(),
                        timestamp: "2014.10.22 23:50:52".to_string(),
                        message_type: MessageType::Text,
                        reactions: Vec::new(),
                    },
                    // ... other messages
                ],
                3,
            ),
        );

        rooms.insert(
            "2".to_string(),
            ChatRoomData::new(
                Chat {
                    id: "2".to_string(),
                    name: "技术讨论群".to_string(),
                    avatar: "技".to_string(),
                    member_count: 225,
                    last_message: Some("大家好".to_string()),
                    chat_type: ChatType::Group,
                    pin: false,
                },
                vec![ChatMessage {
                    is_bot: false,
                    id: "2-1".to_string(),
                    chat_id: "2".to_string(),
                    sender: "技术人员".to_string(),
                    avatar: "技".to_string(),
                    content: "技术讨论群的消息".to_string(),
                    timestamp: "2024.03.09 10:00:00".to_string(),
                    message_type: MessageType::Text,
                    reactions: Vec::new(),
                }],
                1,
            ),
        );

        rooms
    }

    fn init_default_data(&mut self) {
        let rooms = Self::create_default_chat_rooms();
        for (_, room_data) in rooms.iter() {
            self.chats.push(room_data.chat.clone());
            self.unread_counts
                .insert(room_data.chat.id.clone(), room_data.unread_count);
            self.messages.extend(room_data.messages.clone());
        }
    }

    pub fn get_messages_for_chat(&self, chat_id: &str) -> Vec<ChatMessage> {
        self.messages
            .iter()
            .filter(|msg| msg.chat_id == chat_id)
            .cloned()
            .collect()
    }

    pub fn add_message(&mut self, chat_id: &str, message: ChatMessage) {
        self.messages.push(message);
        if let Some(count) = self.unread_counts.get_mut(chat_id) {
            *count += 1;
        }
    }

    pub fn mark_as_read(&mut self, chat_id: &str) {
        if let Some(count) = self.unread_counts.get_mut(chat_id) {
            *count = 0;
        }
    }
}

impl Default for ChatData {
    fn default() -> Self {
        Self {
            chats: Vec::new(),
            messages: Vec::new(),
            notifications: Vec::new(),
            unread_counts: HashMap::new(),
        }
    }
}

mod setup;
mod theme;

pub use setup::*;
pub use theme::*;
