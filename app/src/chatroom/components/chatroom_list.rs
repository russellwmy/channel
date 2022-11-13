use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
    chatroom::{components::ChatroomListCard, types::NewChatroom, CHATROOM},
    temp::components::TempCreateForm,
    wallet::components::NearConnectButton,
};

pub fn ChatroomList(cx: Scope) -> Element {
    let chatroom_state = use_atom_ref(&cx, CHATROOM);

    let chatrooms = chatroom_state.read().get_all_chatrooms().clone();
    let active_id = chatroom_state.read().active_id;

    let sample_data = NewChatroom {
        name: "Group name".to_string(),
        user: HashMap::new(),
    };

    let name_list = chatrooms.into_iter().map(|(id, item)| {
        rsx!(ChatroomListCard {
            id: "{id}",
            key: "{id}",
            title: item.name.to_owned(),
            active: active_id == item.id,
            onclick: move |_| {
                chatroom_state.write().set_active_id(item.id);
            }
        })
    });

    cx.render(rsx! (
        div {
            // class: "relative items-stretch",
            div {
                class: "flex flex-row items-center justify-between border-b-2 border-gray-100 py-2 px-4 md:space-x-10 sticky top-0",
                h1 { "Channel" }
                NearConnectButton{}
            }
            div {
                name_list
            }
            div {
                // class: "fixed bottom-40 right-0 p-2 items-center",
                TempCreateForm{
                    onclick:  move |_| {
                        chatroom_state.write().add_chatroom(sample_data.clone());
                    }
                }
            }
        }
    ))
}
