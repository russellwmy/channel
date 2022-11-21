use std::collections::HashMap;

use dioxus::prelude::AtomRef;

use crate::chat_room::types::ChatRoom;

pub type ChatRooms = HashMap<String, ChatRoom>;

pub static CHATROOM: AtomRef<ChatRoomState> = |_| ChatRoomState::new();

#[derive(Clone)]
pub struct ChatRoomState {
    chat_rooms: ChatRooms,
    active_id: Option<String>,
}

impl ChatRoomState {
    pub fn new() -> Self {
        Self {
            chat_rooms: ChatRooms::default(),
            active_id: None,
        }
    }

    pub fn get_all_chat_rooms(&self) -> ChatRooms {
        self.chat_rooms.clone()
    }

    pub fn add_chat_room(&mut self, chat_room: ChatRoom) {
        self.chat_rooms.insert(chat_room.id.clone(), chat_room.clone());
    }

    pub fn get_chat_room(&self, id: String) -> Option<ChatRoom> {
        return self.chat_rooms.get(&id).cloned();
    }

    pub fn get_active_chat_room(&self) -> Option<ChatRoom> {
        match self.active_id.clone() {
            Some(active_id) => {
                let chat_room = self.chat_rooms.get(&active_id);
                match chat_room {
                    Some(chat_room) => Some(chat_room.clone()),
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn set_active_id(&mut self, id: String) {
        self.active_id = Some(id);
    }
    pub fn active_id(&self) -> Option<String> {
        self.active_id.clone()
    }

    // pub fn update_chat_room_name(&mut self, String: u32, name: String) {
    //     if let Some(rec) = self.chat_rooms.get_mut(&id) {
    //         let mut new = rec.clone();
    //         new.name = name;
    //         *rec = new;
    //     };
    // }

    //     pub fn add_chat_room_user(&mut self, id: u32, user: String) {
    //         if let Some(rec) = self.chat_rooms.get_mut(&id) {
    //             let mut updated = rec.clone();
    //             updated.user.insert(
    //                 user.clone(),
    //                 ChatRoomUser {
    //                     address: user,
    //                     muted: false,
    //                 },
    //             );
    //             *rec = updated;
    //         };
    //     }

    //     pub fn remove_chat_room_user(&mut self, id: u32, user: String) {
    //         if let Some(rec) = self.chat_rooms.get_mut(&id) {
    //             let mut updated = rec.clone();
    //             updated.user.remove(&user);
    //             *rec = updated;
    //         };
    //     }

    //     fn _add_id(&mut self) {
    //         self.next_id += 1
    //     }

    //     fn _add_chat_room(&mut self, chat_room: ChatRoomCard) {
    //         self.chat_rooms.insert(self.next_id, chat_room);
    //     }
}

// fn test_add_chat_room() -> ChatRoomState {
//     let mut chat_room = ChatRoomState::new();

//     chat_room._add_chat_room({
//         ChatRoomCard {
//             id: 4,
//             name: "second_rec".to_string(),
//             user: HashMap::new(),
//         }
//     });

//     assert!(chat_room.chat_rooms.len() == 1, "{}", 0);

//     chat_room
// }

// fn test_add_chat_room_wo_id() -> ChatRoomState {
//     let mut chat_room = ChatRoomState::new();

//     chat_room.add_chat_room({
//         NewChatRoom {
//             name: "first_record".to_string(),
//             user: HashMap::new(),
//         }
//     });

//     assert!(chat_room.chat_rooms.len() == 1, "{}", 0);

//     chat_room
// }

// #[test]
// fn test_create_new_chat_room() {
//     let chat_room = ChatRoomState::new();
//     assert!(chat_room.next_id == 1, "{}", 0);
// }

// #[test]
// fn test_add_next_id() {
//     let mut chat_room = ChatRoomState::new();
//     chat_room._add_id();
//     assert!(chat_room.next_id == 2, "{}", 0);
// }

// #[test]
// fn test_add_chat_room_success() {
//     test_add_chat_room();
// }

// #[test]
// fn test_get_chat_room() {
//     let chat_room = test_add_chat_room();

//     let list = chat_room.get_all_chat_rooms();
//     assert!(list.len() == 1, "{}", 0);
// }

// #[test]
// fn test_create_chat_room_wo_id() {
//     let chat_room = test_add_chat_room_wo_id();

//     assert!(chat_room.next_id == 2, "{}", 0);

//     let list = chat_room.get_all_chat_rooms();

//     assert!(list.len() == 1, "{}", 0);
// }

// #[test]
// fn test_get_chat_room_by_id() {
//     let chat_room = test_add_chat_room_wo_id();
//     let chat_room = chat_room.get_chat_room(1);
//     match chat_room {
//         Some(p) => {
//             assert!(p.id == 1, "{}", 0);
//         }
//         None => {
//             assert!(false, "{}", 0);
//         }
//     }
// }

// #[test]
// fn test_update_chat_room_by_id() {
//     let mut chat_room = test_add_chat_room_wo_id();
//     let a = "1111".to_string();

//     chat_room.update_chat_room_name(1, a.clone());

//     let chat_room = chat_room.get_chat_room(1);

//     match chat_room {
//         Some(p) => {
//             assert!(p.name == a, "{}", 0);
//         }
//         None => {
//             assert!(false, "{}", 0);
//         }
//     }
// }

// #[test]
// fn test_add_chat_room_user_success() {
//     let mut chat_room = test_add_chat_room_wo_id();
//     let a = "user_3".to_string();

//     chat_room.add_chat_room_user(1, a.clone());

//     let chat_room = chat_room.get_chat_room(1);

//     match chat_room {
//         Some(p) => assert!(p.user[&a].address == a, "{}", 0),
//         None => assert!(false, "{}", 0),
//     }
// }

// #[test]
// fn test_remove_chat_room_user() {
//     let mut chat_rooms = test_add_chat_room_wo_id();
//     let a = "user_3".to_string();

//     chat_rooms.add_chat_room_user(1, a.clone());

//     let chat_room = chat_rooms.get_chat_room(1);

//     match chat_room {
//         Some(p) => assert!(p.user.len() == 1, "{}", 0),
//         None => assert!(false, "{}", 0),
//     }

//     chat_rooms.remove_chat_room_user(1, a.clone());

//     let chat_room = chat_rooms.get_chat_room(1);

//     match chat_room {
//         Some(p) => assert!(p.user.len() == 0, "{}", 0),
//         None => assert!(false, "{}", 0),
//     }

//     let chat_room = chat_rooms.get_active_chat_room();

//     match chat_room {
//         Some(p) => assert!(p.id == 1, "{}", 0),
//         None => assert!(false, "{}", 0),
//     }
// }
