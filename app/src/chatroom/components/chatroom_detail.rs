use dioxus::prelude::*;

use crate::{
    chatroom::CHATROOM,
    chatroom::types::Group,
    components::Icon,
    
};

pub fn ChatroomDetail(cx: Scope) -> Element {
    let chatroom_state = use_atom_ref(&cx, CHATROOM);
    let active_chatroom: Option <Group> = None;

    // let active_chatroom = chatroom_state.read().get_active_chatroom();
    // let Some(active_user) = active_chatroom.user.len();
    cx.render(
        match active_chatroom {
            Some(chatroom) => {rsx! (
                div {
                    class: "flex flex-1 flex-col items-stretch relative",
                    div {
                        class: " sticky top-0 items-center justify-between border-b-2 border-gray-100 p-5 md:space-x-10",
                        h2 {"{chatroom.name}"}
                    }
                    // div {
                    //     class: "flex h-full flex-col items-center pt-10 bg-orange-50",
                    //     chatroom.user.clone().into_iter().map(|(_, user)| rsx!(
                    //         ChatroomUserCard {
                    //             address: user.address.to_owned(),
                    //             key: "{user.address}",
                    //             muted: user.muted,
                    //         }
                    //     )) 
                    // }
                    div {
                        class: "fixed bottom-0 flex self-center p-4",
                        div {
                            class : "flex flex-row justify-between",
                            button {
                                // onclick: move |_| {
                                //     // chatroom_state.write().set_active_id(item.id);
                                // }, 
                                class: "pr-20",
                                Icon{
                                    name: if true {"fa-solid fa-microphone-slash"} else {"fa-solid fa-microphone"},
                                    color: "text-black".to_owned(),
                                    size: "w-16 h-16".to_owned(),
                                    bg_color: "bg-neutral-100".to_owned(),
                                }
                            }
                            button {
                                // onclick: 
                                class: "pr-20",
                                Icon{
                                    name: "fa-solid fa-phone",
                                    color: "text-white".to_owned(),
                                    size: "w-16 h-16".to_owned(),
                                    bg_color: "bg-red-800".to_owned(),
                                }
                            }
                            button {
                                onclick: move |_| {
                                    // chatroom_state.write().add_chatroom_user(chatroom.uuid, "new user".to_owned() + &chatroom.user.len().to_string());
                                },
                                Icon{
                                    name: "fa-solid fa-plus",
                                    color: "text-white".to_owned(),
                                    size: "w-16 h-16".to_owned(),
                                    bg_color: "bg-blue-500".to_owned(),
                                }
                            }
                        }
                    }
                }
            )}
            None => {rsx! (
                div {
                    class: "bg-red-50",
                    h2 {}
                }
            )}
        }
    )
}
