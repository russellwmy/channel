use dioxus::prelude::*;
use webrtc::Client;

use crate::{
    chat_room::{
        components::{ChatRoomUserCard, InviteUserModal},
        functions::get_group,
        types::GetGroupInput,
    },
    wallet::WALLET,
};

pub static CLIENT: AtomRef<Client> = |_| Client::new();

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
    let is_muted = use_state(&cx, || true);
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();
    let wallet_clone = wallet.clone();

    let account_id = wallet_state.read().account_id();
    let account_id_clone = account_id.clone();

    let client = use_atom_ref(&cx, CLIENT);
    let cloned_client = client.clone();

    let room_id_clone = room_id.clone();
    let client_fut = use_future(&cx, (), |_| async move {
        let host = web_sys::window().unwrap().location().host().unwrap();

        cloned_client
            .write()
            .connect(format!("wss://{}/ws", host).as_str())
            .await;
        cloned_client.write().join_room(&room_id_clone.clone());
    });

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

    let handle_microphone_click = move |_| {
        let cloned_client = client.clone();
        let cloned_is_muted = is_muted.clone();
        cx.spawn({
            async move {
                let mut client = cloned_client.write();
                let mut local_participant = client.room().local_participant().unwrap();
                if *(cloned_is_muted).clone() {
                    local_participant.start_streaming().await;
                    cloned_is_muted.set(false);
                } else {
                    local_participant.stop_streaming().await;
                    cloned_is_muted.set(true);
                }

                client.set_local_participant(local_participant);
            }
        });
    };
    let microphone_button_class = match *(is_muted).clone() {
        true => "btn-error",
        false => "btn-success",
    };
    cx.render(rsx! (
        div {
            class: "relative items-stretch",
            div {
                class: "relative flex items-center justify-between h-[50px] mb-2",
                div {
                    class: "flex",
                    InviteUserModal{}
                    button {
                        class: "btn w-[48px] h-[48px] ml-1",
                        onclick: move |_| {
                            group_fut.restart();
                        },
                        i { class: "fa-solid fa-rotate" },
                    }

                }
                button {
                    class: "btn w-[48px] h-[48px] text-white {microphone_button_class} ml-1",
                    onclick: handle_microphone_click,
                    match  *(is_muted).clone() {
                        true => rsx!(i { class: "fa-solid fa-microphone-slash" }),
                        false => rsx!(i { class: "fa-solid fa-microphone" }),
                    }

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
