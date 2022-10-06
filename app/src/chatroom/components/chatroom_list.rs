use dioxus::prelude::*;

use crate::{
    chatroom::components::{ChatroomCard, ChatroomListCard},
    home::pages::components::Header,
    temp::components::TempCreateForm,
};

pub type Chatrooms = im_rc::HashMap<u32, ChatroomCard>;

pub fn ChatroomList(cx: Scope) -> Element {
    let chatroom_list = use_state(&cx, Chatrooms::default);
    let active_id = use_state(&cx, || 0);
    let chatroom_id = use_state(&cx, || 2);

    let name_list = chatroom_list.iter().map(|(id, item)| {
        rsx!(ChatroomListCard {
            id: "{id}",
            key: "{id}",
            title: item.name.to_owned(),
            active: *active_id == item.id,
            onclick: move |_| { *active_id.make_mut() = item.id }
        })
    });

    // add sample data
    chatroom_list.make_mut().insert(
        0,
        ChatroomCard {
            id: 0,
            name: "I like a dog".to_string(),
        },
    );

    chatroom_list.make_mut().insert(
        1,
        ChatroomCard {
            id: 1,
            name: "I like a cat".to_string(),
        },
    );

    cx.render(rsx! (
        div {
            div {
                class: "sm:w-full lg:w-1/4 ",
                name_list
            }
            div {
                class: "p-2 flex justify-around",
                TempCreateForm{
                    // chatroom_list: chatroom_list,
                    // chatroom_id: chatroom_id,
                    onclick:  move |_| {
                        chatroom_list.make_mut().insert(
                            **chatroom_id,
                            ChatroomCard {
                                id: **chatroom_id,
                                name: "_".to_string(),
                            },
                        );
                        *chatroom_id.make_mut() += 1;
                    }
                }
            }
        }
    ))
}
