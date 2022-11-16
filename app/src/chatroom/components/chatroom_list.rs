use std::collections::HashMap;

use dioxus::prelude::*;
use log::info;

use crate::{
    chatroom::{
        components::{ChatroomListCard, NewChatroomModal}, 
        types::NewChatroom, 
        functions::get_groups_debug,
        CHATROOM
    },
    temp::components::TempCreateForm,
    user::{
        components::CreateUserButton,
        types::{GetUserInfoInput, NewUserInput},
        functions::get_user_info,
        USER,
    },
    wallet::{components::NearConnectButton, WALLET},
};
#[derive(Debug)]
enum Version {
    Version1,
    Version2,
}

pub fn ChatroomList(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();
    let wallet_clone = wallet.clone();
    let account_id = wallet_state.read().account_id();
    let account_clone = account_id.clone();

    let chatroom_state = use_atom_ref(&cx, CHATROOM);
    let chatrooms = chatroom_state.read().get_all_chatrooms().clone();
    let active_id = chatroom_state.read().active_id;

    let user_state = use_atom_ref(&cx, USER);
    let user = user_state.read().info();
    let username = use_state(&cx, || "Hello!".to_string());

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

    use_effect(&cx, (), |_| async move {
        // cx.spawn({
        //     async move {
                if let Some(account_id) = account_clone {

                    let result = get_user_info(
                        wallet_clone,
                        GetUserInfoInput {
                            account_id: account_id,
                        },
                    ).await;

                    match result {
                        Ok(v) => {
                            user_state.write().set_info(v);
                            info!("result detail:");
                        },
                        Err(e) => info!("error "),
                    }
                }

                // let groups = get_groups_debug(wallet.clone()).await;

                // match groups {
                //     Ok(v) => {
                //         info!("sucess groups");
                //     },
                //     Err(e) => info!("error {}", e),
                // }
        //     }
        // });
    });

    cx.render(rsx! (
        div {
            class: "relative items-stretch",
            div {
                class: "flex flex-row items-center justify-between border-b-2 border-gray-100 py-2 px-4 md:space-x-10 sticky top-0",
                cx.render( match account_id {
                    Some(account) => {
                        rsx! {
                            div {
                                h1 {"{account}"}
                                {
                                    match user {
                                        Some(v) => {
                                            rsx! { h2 {"{v.name}"} }
                                        }
                                        None => { rsx! {CreateUserButton{}}}
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        rsx! { div {} }
                    }
                })
                NearConnectButton{}

            }
            div {
                name_list
            }
            
            div {
                // class: "fixed bottom-40 right-0 p-2 items-center",
                class: "p-2 flex justify-around",
                NewChatroomModal()
                TempCreateForm{
                    onclick:  move |_| {
                        chatroom_state.write().add_chatroom(sample_data.clone());
                    }
                }
            }
        }
    ))
}
