use dioxus::prelude::*;

use crate::{
    chat_room::CHATROOM,
    // chat_room::{components::ChatRoomUserCard, CHATROOM},
    components::Icon,
    
};

pub fn ChatRoomDetail(cx: Scope) -> Element {
    let chat_room_state = use_atom_ref(&cx, CHATROOM);
    let active_chat_room = chat_room_state.read().get_active_chat_room();
    // let Some(active_user) = active_chat_room.user.len();
    cx.render(
        match active_chat_room {
            Some(chat_room) => {rsx! (
                div {
                    class: "flex flex-1 flex-col items-stretch relative",
                    div {
                        class: " sticky top-0 items-center justify-between border-b-2 border-gray-100 p-5 md:space-x-10",
                        h2 {"{chat_room.name}"}
                    }
                    // div {
                    //     class: "flex h-full flex-col items-center pt-10 bg-orange-50",
                    //     chat_room.user.clone().into_iter().map(|(_, user)| rsx!(
                    //         ChatRoomUserCard {
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
                                //     // chat_room_state.write().set_active_id(item.id);
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
                                    // chat_room_state.write().add_chat_room_user(chat_room.id, "new user".to_owned() + &chat_room.user.len().to_string());
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
