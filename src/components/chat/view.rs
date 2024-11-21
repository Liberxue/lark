#[derive(Debug)]
pub struct ChatListModel {
    pub items: Vec<ChatListItem>,
    pub filter: ChatFilter,
    pub selected_id: String,
}

#[derive(Debug)]
pub struct ChatListItem {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub last_message: String,
    pub is_selected: bool,
    pub unread_count: i32,
    pub is_pinned: bool,
}
#[derive(Clone, Copy, PartialEq)]
enum ChatFilter {
    All,
    Pinned,
}

#[derive(Clone, Debug)]
pub struct Chat {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub member_count: i32,
    pub last_message: Option<String>,
    pub chat_type: i32,
    pub pin: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatType {
    Group,
    DirectMessage,
    Bot,
    // More
}
#[derive(Debug)]
struct ChatMessage {
    id: String,
    sender: String,
    avatar: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
    reactions: Vec<Reaction>,
}

#[derive(Debug)]
enum MessageType {
    Text,
    Images,
    File,
    Code,
    // TODO::
}

#[derive(Debug)]
pub struct Reaction {
    pub emoji: String,
    pub count: i32,
    pub users: Vec<String>,
}

#[derive(Debug)]
struct Notification {
    pub title: String,
    pub content: String,
    pub timestamp: String,
}
impl ChatListModel {
    // add code here
    fn new(
        chats: Vec<ChatListItem>,
        selected_id: String,
        unread_count: &HashMap<String, i32>,
    ) -> Self {
    }
}
