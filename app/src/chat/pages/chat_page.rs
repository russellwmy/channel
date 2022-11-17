use dioxus::prelude::*;
use webrtc::Client;

pub static CLIENT: AtomRef<Client> = |_| Client::new();
pub static DEFAULT_ROOM_ID: &str ="7c50f4d3-30ad-4aab-886d-a27c6f5c804f";

pub fn ChatPage(cx: Scope) -> Element {
    let wallet = use_atom_ref(&cx, crate::wallet::WALLET);
    let client = use_atom_ref(&cx, CLIENT);
    // let user_id = use_state(&cx, || None);
    let cloned_client = client.clone();
    let client_fut = use_future(&cx, (), |_| async move {
        cloned_client.write().connect("ws://127.0.0.1:9000").await;
    });

    let handle_join = move |_| {
        let cloned_client = client.clone();
        cx.spawn({
            async move {
                cloned_client.write().join_room(DEFAULT_ROOM_ID);
            }
        });
    };

    let handle_new_channel_click = move |_| {
        let cloned_client = client.clone();
        cx.spawn({
            async move {
                cloned_client.write().create_room();
            }
        });
    };

    if !wallet.read().checked {
        wallet.write().check();
    }

    cx.render(rsx! (
        div {
            class: "w-screen h-screen flex items-center justify-center",
            div {
                div {
                    // "{user_id:?}"
                }
                div {
                    button {
                        class: "btn",
                        onclick: handle_new_channel_click,
                        "New Channel"
                    }
                }
                div {
                    button {
                        class: "btn",
                        onclick: handle_join,
                        "Join"
                    }
                }
            }
        }
    ))
}
