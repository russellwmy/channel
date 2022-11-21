use dioxus::prelude::*;

use crate::{
    chat_room::{
        components::{ChatRoomUserCard, InviteUserModal},
        functions::get_group,
        types::GetGroupInput,
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

    let group_fut = use_future(&cx, (), |_| async move {
        match account_id {
            Some(account_id) => {
                let wallet_clone = wallet.clone();
                let result = get_group(
                    wallet_clone,
                    GetGroupInput {
                        group_id: room_id.clone(),
                    },
                )
                .await;
                match result {
                    Ok(result) => Some(result),
                    _ => None,
                }
            }
            _ => None,
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
                        group_fut.restart();
                    },
                    i { class: "fa-solid fa-rotate" },
                }
            }
            div {
                match group_fut.value() {
                    Some(value) => {
                        match value {
                            Some(group) =>  rsx!(
                                group.users.iter().map(|item| {
                                    rsx!(ChatRoomUserCard {
                                        key: "{item.account_id}",
                                        account_id: item.account_id.to_string(),
                                        muted: false,
                                        is_admin: item.is_admin,
                                    })
                                })
                            ),
                            None => rsx!("")
                        }
                    }
                    None => rsx!(""),
                }
            }
        }
    ))
}
