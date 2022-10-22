use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{
    chatroom::{components::ChatroomListCard, types::NewChatroom, CHATROOM},
    temp::components::TempCreateForm,
};

pub fn ChatroomList(cx: Scope) -> Element {
    let chatroom_state = use_atom_ref(&cx, CHATROOM);

    let chatrooms = chatroom_state.read().get_all_chatrooms().clone();
    let active_id = use_state(&cx, || 0);

    let sample_data = NewChatroom {
        name: "I like a dog".to_string(),
        user: HashMap::new(),
    };

    let name_list = chatrooms.into_iter().map(|(id, item)| {
        rsx!(ChatroomListCard {
            id: "{id}",
            key: "{id}",
            title: item.name.to_owned(),
            active: *active_id == item.id,
            onclick: move |_| { *active_id.make_mut() = item.id }
        })
    });

    cx.render(rsx! (
        div {
            div {
                class: "sm:w-full lg:w-1/4 ",
                name_list
            }
            div {
                class: "p-2 flex justify-around",
                TempCreateForm{
                    onclick:  move |_| {
                        chatroom_state.write().add_chatroom(sample_data.clone());
                    }
                }
            }
        }
    ))
}
