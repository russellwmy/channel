use std::collections::HashMap;
use std::vec::Vec;

use dioxus::prelude::*;

use crate::{
    chatroom::{
        components::{ChatroomListCard, NewChatroomModal},
        functions::{get_groups_debug, get_joined_groups},
        types::{Group, NewChatroom},
        CHATROOM,
    },
    errors::ContractCallError,
    temp::components::TempCreateForm,
    user::{components::CreateUserButton, functions::get_user, types::GetUserInput, USER},
    wallet::{components::NearConnectButton, WALLET},
};

pub type Groups = HashMap<String, Group>;

pub fn ChatroomList(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();
    let wallet_clone = wallet.clone();
    let account_id = wallet_state.read().account_id();
    let account_clone = account_id.clone();

    let chatroom_state = use_atom_ref(&cx, CHATROOM);

    let active_id = chatroom_state.read().active_id.clone();
    log::info!("active id :: {}", active_id);
    let active_id_clone = &*active_id;

    let user_state = use_atom_ref(&cx, USER);
    let user = user_state.read().user();

    let user_fut = use_future(&cx, (), |_| async move {
        match account_clone {
            Some(account_id) => {
                let result = get_user(wallet_clone, GetUserInput { account_id }).await;
                result.map_err(|e| ContractCallError::CallFail(e.to_string()))
            }
            _ => Err(ContractCallError::InputError(
                "missing account id".to_string(),
            )),
        }
    });

    match user.clone() {
        Some(val) => {}
        None => {
            match user_fut.value() {
                Some(Ok(val)) => {
                    user_state.write().set_info(val.clone());
                }
                Some(Err(err)) => log::error!("{}", err),
                None => log::info!("loading"),
            };
        }
    };


    let empty_group: Vec<Group> = Vec::new();
    let group = use_state(&cx, || empty_group);

    let group_fut = use_future(&cx, (), |_| async move {
        let wallet_clone = wallet.clone();
        let groups: Result<Vec<Group>, serde_json::Error>  = get_groups_debug(wallet_clone).await;
        return groups
    });

    if (group.clone().len() == 0){

        match group_fut.value() {
            Some(Ok(val)) => { group.set(val.clone()) }
            Some(Err(err)) => log::error!(" get group debug{}", err),
            None => log::info!("loading"),
        };
    };

    cx.render(rsx! (
        div {
            class: "items-stretch",
            div {
                class: "flex flex-row items-center justify-between border-b-2 border-gray-100 py-2 px-4 md:space-x-10 sticky top-0",
                cx.render( match account_id.clone() {
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
                group.iter().map(|item| {
                    rsx!(ChatroomListCard {
                        id: "{item.uuid}",
                        // key: "{item.uuid}",
                        title: item.name.to_owned(),
                        active: active_id_clone == item.uuid,
                        onclick: move |_| {
                            chatroom_state.write().set_active_id(item.clone().uuid);
                        }
                    })
                })
            }
            NewChatroomModal()
            // match account_id.clone() {
            //     Some(account) => {
            //         rsx! {
            //             button {
            //                 class: "btn primary normal-case m-3",
            //                 onclick: move |_| { 
            //                     // let wallet_clone_2 = wallet_clone.clone();
            //                     // let account_clone = account.clone();
            //                     // cx.spawn({
            //                     //     async move {
            //                     //         get_joined_groups(wallet_clone_2.clone(), account_clone).await;
            //                     //     }
            //                     // });
            //                 },
            //                 h2 {
            //                     "Joined group only"
            //                 }
            //             }
            //         }
            //     }
            //     None => { rsx! { div {} }}
            // }

        }
    ))
}
