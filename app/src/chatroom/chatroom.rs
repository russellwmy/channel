use std::collections::HashMap;
use std::vec::Vec;
use dioxus::prelude::AtomRef;

use crate::chatroom::types::{Group, ChatroomUser, NewChatroom};

pub type Groups = HashMap<String, Group>;

pub static CHATROOM: AtomRef<ChatroomState> = |_| ChatroomState::new();

pub struct ChatroomState {
    // chatrooms: Chatrooms,
    pub active_id: String,
}

impl ChatroomState {
    pub fn new() -> Self {
        Self {
            active_id: "3".to_string(),
        }
    }

    pub fn set_active_id(&mut self, uuid: String) {
        self.active_id = uuid;
    }

    // pub fn get_all_chatrooms(&self) -> Chatrooms {
    //     self.chatrooms.clone()
    // }

    // pub fn add_chatroom(&mut self, chatroom: NewChatroom) {
    //     let new_chatroom = Group {
    //         creator:"stat".to_string(),
    //         uuid: "Fds".to_string(),
    //         name: chatroom.name,
    //         // user: chatroom.user,
    //     };

    //     // self._add_chatroom(new_chatroom);
    //     // self._add_id();
    // }

    // pub fn get_chatroom(&self, uuid: String) -> Option<Group> {
    //     return self.chatrooms.get(&uuid).cloned();
    // }

    // pub fn get_active_chatroom(&self) -> Option<Group> {
    //     return self.chatrooms.get(&self.active_id).cloned();
    // }

    // pub fn set_group(&mut self, group: Vec<Group>) {
    //     // let group_hm: HashMap<String, Group> = group.into_iter().collect();
    //     // self.chatrooms = group_hm;
    // }

    // pub fn update_chatroom_name(&mut self, uuid: String, name: String) {
    //     if let Some(rec) = self.chatrooms.get_mut(&uuid) {
    //         let mut new = rec.clone();
    //         new.name = name;
    //         *rec = new;
    //     };
    // }

    // pub fn add_chatroom_user(&mut self, uuid: String, user: String) {
    //     if let Some(rec) = self.chatrooms.get_mut(&uuid) {
    //         let mut updated = rec.clone();
    //         updated.user.insert(
    //             user.clone(),
    //             ChatroomUser {
    //                 address: user,
    //                 muted: false,
    //             },
    //         );
    //         *rec = updated;
    //     };
    // }

    // pub fn remove_chatroom_user(&mut self, uuid: String, user: String) {
    //     if let Some(rec) = self.chatrooms.get_mut(&uuid) {
    //         let mut updated = rec.clone();
    //         updated.user.remove(&user);
    //         *rec = updated;
    //     };
    // }

    // fn _add_id(&mut self) {
    //     self.next_id += 1
    // }

    // fn _add_chatroom(&mut self, chatroom: Group) {
    //     self.chatrooms.insert(self.next_id, chatroom);
    // }
}

// fn test_add_chatroom() -> ChatroomState {
//     let mut chatroom = ChatroomState::new();

//     chatroom._add_chatroom({
//         Group {
//             uuid: "4",
//             name: "second_rec".to_string(),
//             user: HashMap::new(),
//         }
//     });

//     assert!(chatroom.chatrooms.len() == 1, "{}", 0);

//     chatroom
// }

// fn test_add_chatroom_wo_id() -> ChatroomState {
//     let mut chatroom = ChatroomState::new();

//     chatroom.add_chatroom({
//         NewChatroom {
//             name: "first_record".to_string(),
//             user: HashMap::new(),
//         }
//     });

//     assert!(chatroom.chatrooms.len() == 1, "{}", 0);

//     chatroom
// }

// #[test]
// fn test_create_new_chatroom() {
//     let chatroom = ChatroomState::new();
//     assert!(chatroom.next_id == 1, "{}", 0);
// }

// #[test]
// fn test_add_next_id() {
//     let mut chatroom = ChatroomState::new();
//     chatroom._add_id();
//     assert!(chatroom.next_id == 2, "{}", 0);
// }

// #[test]
// fn test_add_chatroom_success() {
//     test_add_chatroom();
// }

// #[test]
// fn test_get_chatroom() {
//     let chatroom = test_add_chatroom();

//     let list = chatroom.get_all_chatrooms();
//     assert!(list.len() == 1, "{}", 0);
// }

// #[test]
// fn test_create_chatroom_wo_id() {
//     let chatroom = test_add_chatroom_wo_id();

//     assert!(chatroom.next_id == 2, "{}", 0);

//     let list = chatroom.get_all_chatrooms();

//     assert!(list.len() == 1, "{}", 0);
// }

// #[test]
// fn test_get_chatroom_by_id() {
//     let chatroom = test_add_chatroom_wo_id();
//     let chatroom = chatroom.get_chatroom(1);
//     match chatroom {
//         Some(p) => {
//             assert!(p.uuid == "1", "{}", 0);
//         }
//         None => {
//             assert!(false, "{}", 0);
//         }
//     }
// }

// #[test]
// fn test_update_chatroom_by_id() {
//     let mut chatroom = test_add_chatroom_wo_id();
//     let a = "1111".to_string();

//     chatroom.update_chatroom_name(1, a.clone());

//     let chatroom = chatroom.get_chatroom(1);

//     match chatroom {
//         Some(p) => {
//             assert!(p.name == a, "{}", 0);
//         }
//         None => {
//             assert!(false, "{}", 0);
//         }
//     }
// }

// #[test]
// fn test_add_chatroom_user_success() {
//     let mut chatroom = test_add_chatroom_wo_id();
//     let a = "user_3".to_string();

//     chatroom.add_chatroom_user(1, a.clone());

//     let chatroom = chatroom.get_chatroom(1);

//     match chatroom {
//         Some(p) => assert!(p.user[&a].address == a, "{}", 0),
//         None => assert!(false, "{}", 0),
//     }
// }

// #[test]
// fn test_remove_chatroom_user() {
//     let mut chatrooms = test_add_chatroom_wo_id();
//     let a = "user_3".to_string();

//     chatrooms.add_chatroom_user(1, a.clone());

//     let chatroom = chatrooms.get_chatroom(1);

//     match chatroom {
//         Some(p) => assert!(p.user.len() == 1, "{}", 0),
//         None => assert!(false, "{}", 0),
//     }

//     chatrooms.remove_chatroom_user(1, a.clone());

//     let chatroom = chatrooms.get_chatroom(1);

//     match chatroom {
//         Some(p) => assert!(p.user.len() == 0, "{}", 0),
//         None => assert!(false, "{}", 0),
//     }

//     let chatroom = chatrooms.get_active_chatroom();

//     match chatroom {
//         Some(p) => assert!(p.uuid == "1", "{}", 0),
//         None => assert!(false, "{}", 0),
//     }
// }
