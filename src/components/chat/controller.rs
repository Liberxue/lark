use super::{Chat, ChatEvent, ChatFilter, ChatListModel};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ChatListController {
    model: ChatListModel,
}

impl ChatListController {
    pub fn new(chats: &[Chat], selected_id: &str, unread_count: &HashMap<String, i32>) -> Self {
        Self {
            model: ChatListModel::new(chats, selected_id, unread_count),
        }
    }
    pub fn handle_click(&mut self, id: String) -> ChatEvent {
        ChatEvent::Selected { id }
    }
    pub fn handle_filter_changer(&mut self, show_pinned: bool) {
        self.model.set_filter(if show_pinned {
            ChatFilter::Pinned
        } else {
            ChatFilter::All
        });
    }

    pub fn view(&self) -> &ChatListModel {
        &self.model
    }
}
