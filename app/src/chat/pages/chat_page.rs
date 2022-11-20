use dioxus::prelude::*;
use webrtc::Client;

pub static CLIENT: AtomRef<Client> = |_| Client::new();
pub static DEFAULT_ROOM_ID: &str = "7c50f4d3-30ad-4aab-886d-a27c6f5c804f";

pub fn ChatPage(cx: Scope) -> Element {
    let wallet = use_atom_ref(&cx, crate::wallet::WALLET);
    let client = use_atom_ref(&cx, CLIENT);
    // let user_id = use_state(&cx, || None);
    let cloned_client = client.clone();
    let client_fut = use_future(&cx, (), |_| async move {
        let host = web_sys::window().unwrap().location().host().unwrap();

        cloned_client
            .write()
            .connect(format!("ws://{}/ws", host).as_str())
            .await;
    });

    let handle_join = move |_| {
        let cloned_client = client.clone();
        cx.spawn({
            async move {
                cloned_client.write().join_room(DEFAULT_ROOM_ID);
            }
        });
    };

    let handle_start_stream = move |_| {
        let cloned_client = client.clone();
        cx.spawn({
            async move {
                // let mut client = cloned_client.write();
                // let room = client.room();
                // let mut local_participant = room.local_participant().unwrap();

                // local_participant.borrow_mut().start_stream().await;
                // let stream = local_participant.stream().unwrap();
                // for participant in room.participants() {
                //     participant.publish(stream.clone());
                // }
                // client.set_local_participant(local_participant);
            }
        });
    };

    let handle_stop_stream = move |_| {
        let cloned_client = client.clone();
        cx.spawn({
            async move {
                let mut client = cloned_client.write();
                let mut local_participant = client.room().local_participant().unwrap();

                local_participant.stop_stream().await;
                client.set_local_participant(local_participant);
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

                div {
                    button {
                        class: "btn",
                        onclick: handle_start_stream,
                        "Start Stream"
                    }
                    button {
                        class: "btn",
                        onclick: handle_stop_stream,
                        "Stop Stream"
                    }
                }
            }
        }
    ))
}
