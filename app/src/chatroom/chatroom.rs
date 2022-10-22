use std::collections::HashMap;

use dioxus::prelude::AtomRef;

use crate::chatroom::types::{ChatroomCard, ChatroomUser, NewChatroom};

pub type Chatrooms = HashMap<u32, ChatroomCard>;

pub static CHATROOM: AtomRef<ChatroomState> = |_| ChatroomState::new();

pub struct ChatroomState {
    chatrooms: Chatrooms,
    next_id: u32,
}

impl ChatroomState {
    pub fn new() -> Self {
        Self {
            chatrooms: Chatrooms::default(),
            next_id: 1,
        }
    }

    pub fn get_all_chatrooms(&self) -> Chatrooms {
        self.chatrooms.clone()
    }

    pub fn add_chatroom(&mut self, chatroom: NewChatroom) {
        let new_chatroom = ChatroomCard {
            id: self.next_id,
            name: chatroom.name,
            user: chatroom.user,
        };

        self._add_chatroom(new_chatroom);
        self._add_id();
    }

    pub fn get_chatroom(&self, id: u32) -> Option<&ChatroomCard> {
        return self.chatrooms.get(&id);
    }

    pub fn update_chatroom_name(&mut self, id: u32, name: String) {
        if let Some(rec) = self.chatrooms.get_mut(&id) {
            let mut new = rec.clone();
            new.name = name;
            *rec = new;
        };
    }

    pub fn add_chatroom_user(&mut self, id: u32, user: String) {
        if let Some(rec) = self.chatrooms.get_mut(&id) {
            let mut updated = rec.clone();
            updated.user.insert(
                user.clone(),
                ChatroomUser {
                    address: user,
                    muted: false,
                },
            );
            *rec = updated;
        };
    }

    pub fn remove_chatroom_user(&mut self, id: u32, user: String) {
        if let Some(rec) = self.chatrooms.get_mut(&id) {
            let mut updated = rec.clone();
            updated.user.remove(&user);
            *rec = updated;
        };
    }

    fn _add_id(&mut self) {
        self.next_id += 1
    }

    fn _add_chatroom(&mut self, chatroom: ChatroomCard) {
        self.chatrooms.insert(self.next_id, chatroom);
    }
}

fn test_add_chatroom() -> ChatroomState {
    let mut chatroom = ChatroomState::new();

    chatroom._add_chatroom({
        ChatroomCard {
            id: 4,
            name: "second_rec".to_string(),
            user: HashMap::new(),
        }
    });

    assert!(chatroom.chatrooms.len() == 1, "{}", 0);

    chatroom
}

fn test_add_chatroom_wo_id() -> ChatroomState {
    let mut chatroom = ChatroomState::new();

    chatroom.add_chatroom({
        NewChatroom {
            name: "first_record".to_string(),
            user: HashMap::new(),
        }
    });

    assert!(chatroom.chatrooms.len() == 1, "{}", 0);

    chatroom
}

#[test]
fn test_create_new_chatroom() {
    let chatroom = ChatroomState::new();
    assert!(chatroom.next_id == 1, "{}", 0);
}

#[test]
fn test_add_next_id() {
    let mut chatroom = ChatroomState::new();
    chatroom._add_id();
    assert!(chatroom.next_id == 2, "{}", 0);
}

#[test]
fn test_add_chatroom_success() {
    test_add_chatroom();
}

#[test]
fn test_get_chatroom() {
    let chatroom = test_add_chatroom();

    let list = chatroom.get_all_chatrooms();
    assert!(list.len() == 1, "{}", 0);
}

#[test]
fn test_create_chatroom_wo_id() {
    let chatroom = test_add_chatroom_wo_id();

    assert!(chatroom.next_id == 2, "{}", 0);

    let list = chatroom.get_all_chatrooms();

    assert!(list.len() == 1, "{}", 0);
}

#[test]
fn test_get_chatroom_by_id() {
    let chatroom = test_add_chatroom_wo_id();
    let chatroom = chatroom.get_chatroom(1);
    match chatroom {
        Some(p) => {
            assert!(p.id == 1, "{}", 0);
        }
        None => {
            assert!(false, "{}", 0);
        }
    }
}

#[test]
fn test_update_chatroom_by_id() {
    let mut chatroom = test_add_chatroom_wo_id();
    let a = "1111".to_string();

    chatroom.update_chatroom_name(1, a.clone());

    let chatroom = chatroom.get_chatroom(1);

    match chatroom {
        Some(p) => {
            assert!(p.name == a, "{}", 0);
        }
        None => {
            assert!(false, "{}", 0);
        }
    }
}

#[test]
fn test_add_chatroom_user_success() {
    let mut chatroom = test_add_chatroom_wo_id();
    let a = "user_3".to_string();

    chatroom.add_chatroom_user(1, a.clone());

    let chatroom = chatroom.get_chatroom(1);

    match chatroom {
        Some(p) => assert!(p.user[&a].address == a, "{}", 0),
        None => assert!(false, "{}", 0),
    }
}

#[test]
fn test_remove_chatroom_user() {
    let mut chatrooms = test_add_chatroom_wo_id();
    let a = "user_3".to_string();

    chatrooms.add_chatroom_user(1, a.clone());

    let chatroom = chatrooms.get_chatroom(1);

    match chatroom {
        Some(p) => assert!(p.user.len() == 1, "{}", 0),
        None => assert!(false, "{}", 0),
    }

    chatrooms.remove_chatroom_user(1, a.clone());

    let chatroom = chatrooms.get_chatroom(1);

    match chatroom {
        Some(p) => assert!(p.user.len() == 0, "{}", 0),
        None => assert!(false, "{}", 0),
    }
}
