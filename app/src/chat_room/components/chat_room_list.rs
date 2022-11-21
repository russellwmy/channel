use dioxus::prelude::*;

use crate::{
    chat_room::{

        components::{ChatRoomListCard, NewChatRoomModal},
        functions::get_owned_groups,
        types::{ GetOwnedGroupsInput},
    },
    user::USER,
    wallet::WALLET,
};
#[derive(Debug)]
enum Version {
    Version1,
    Version2,
}

pub fn ChatRoomList(cx: Scope) -> Element {
    let router = use_router(&cx);
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();
    let wallet_clone = wallet.clone();
    let account_id = wallet_state.read().account_id();
    let account_id_clone = account_id.clone();
    let user_state = use_atom_ref(&cx, USER);
    let user = user_state.read().user();
    let username = use_state(&cx, || "".to_string());

    let chat_rooms_fut = use_future(&cx, (), |_| async move {
        match account_id {
            Some(account_id) => {
                let wallet_clone = wallet.clone();
                get_owned_groups(
                    wallet_clone,
                    GetOwnedGroupsInput {
                        account_id: account_id.clone(),
                    },
                )
                .await
            }
            _ => Ok(vec![]),
        }
    });

    cx.render(rsx! (
        div {
            class: "relative items-stretch",
            div {
                class: "relative flex justify-center h-[50px] mb-2",
                NewChatRoomModal{}
                button {
                    class: "btn ml-1",
                    onclick: move |_| {
                        chat_rooms_fut.restart();
                    },
                    i { class: "fa-solid fa-rotate" },
                }
            }
            div {
                match chat_rooms_fut.value() {
                    Some(Ok(groups)) => {
                        rsx!(
                            groups.iter().map(|item| {
                                rsx!(ChatRoomListCard {
                                    id: "{item.id}",
                                    key: "{item.id}",
                                    name: item.name.clone(),
                                    creator: item.creator.clone().to_string(),
                                    active:  false,
                                    onclick: move |_| {
                                        let view_url = format!("/room/{}", item.id);
                                        router.push_route(&view_url, None, None);
                                    }
                                })
                            })
                        )
                    }
                    Some(Err(err)) => rsx!(""),
                    None => rsx!("loading"),
                }
            }
        }
    ))
}
