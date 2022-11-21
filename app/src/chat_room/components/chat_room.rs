use dioxus::prelude::*;

use crate::{
    chat_room::{
        components::{ChatRoomUserCard, InviteUserModal},
        functions::get_group_users,
        types::GetGroupUserInput,
    },
    user::USER,
    wallet::WALLET,
};
#[derive(Debug)]
enum Version {
    Version1,
    Version2,
}

#[derive(PartialEq, Props)]
pub struct ChatRoomUserListProps {
    room_id: String,
}

pub fn ChatRoom(cx: Scope<ChatRoomUserListProps>) -> Element {
    let room_id = cx.props.room_id.clone();
    let router = use_router(&cx);
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();
    let wallet_clone = wallet.clone();
    let account_id = wallet_state.read().account_id();
    let account_id_clone = account_id.clone();
    let user_state = use_atom_ref(&cx, USER);
    let user = user_state.read().user();
    let username = use_state(&cx, || "".to_string());

    let group_users_fut = use_future(&cx, (), |_| async move {
        match account_id {
            Some(account_id) => {
                let wallet_clone = wallet.clone();
                get_group_users(
                    wallet_clone,
                    GetGroupUserInput {
                        group_id: room_id.clone(),
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
                InviteUserModal{}
                button {
                    class: "btn ml-1",
                    onclick: move |_| {
                        group_users_fut.restart();
                    },
                    i { class: "fa-solid fa-rotate" },
                }
            }
            div {
                match group_users_fut.value() {
                    Some(Ok(groups)) => {
                        rsx!(
                            groups.iter().map(|item| {
                                rsx!(ChatRoomUserCard {
                                    key: "{item.account_id}",
                                    account_id: item.account_id.to_string(),
                                    muted: false,
                                    is_admin: false,
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
