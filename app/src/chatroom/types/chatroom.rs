use std::collections::HashMap;

pub type ChatroomUsers = HashMap<String, ChatroomUser>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChatroomCard {
    pub id: u32,
    pub name: String,
    pub user: ChatroomUsers,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NewChatroom {
    pub name: String,
    pub user: ChatroomUsers,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChatroomUser {
    pub address: String,
    pub muted: bool,
}
