use std::collections::HashMap;

pub type ChatRoomUsers = HashMap<String, ChatRoomUser>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChatRoomCard {
    pub id: u32,
    pub name: String,
    pub user: ChatRoomUsers,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChatRoom {
    pub id: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChatRoomUser {
    pub address: String,
    pub muted: bool,
}
