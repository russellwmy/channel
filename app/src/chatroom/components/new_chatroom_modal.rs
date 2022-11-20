use dioxus::{events::MouseEvent, prelude::*};
use log::info;

use crate::{chatroom::functions::set_group, wallet::WALLET};

pub fn NewChatroomModal(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();

    let is_modal_visible = use_state(&cx, || true);
    let name = use_state(&cx, || "".to_string());

    let view = rsx! {
        div {
            button {
                class: "btn primary normal-case m-3",
                onclick: move |_| { is_modal_visible.set(!is_modal_visible) },
                i {
                   class: "fa-solid fa-plus"
                }
            }
            div {
                class: "static",
                div {
                    class: "absolute bottom-0 left-0 top-0 right-0",
                    hidden: "{is_modal_visible}",
                    div {
                        class: "grid place-content-center h-full",
                        id:"my-modal",
                        hidden: "{is_modal_visible}",
                        div {
                            class:"modal-box p-3 m-3",
                            h3 {"Name"}
                            input {
                                class:"input w-3/4 m-3",
                                oninput: move |evt| name.set(evt.value.clone()),
                            }
                            button {
                                class: " btn primary normal-case m-3",
                                onclick: move |_| { is_modal_visible.set(!is_modal_visible) },
                                h1 {"Close"}
                            }

                            button {
                                class: " btn primary normal-case m-3",
                                onclick: move |_| {
                                    let wallet_clone = wallet.clone();
                                    cx.spawn({
                                        async move {
                                            set_group(wallet_clone, "chatroom 4".to_string()).await;
                                        }
                                    });
                                    info!("after set up");
                                    is_modal_visible.set(!is_modal_visible)
                                },
                                h1 {"create"}
                            }
                        }
                    }
                }
            }

        }
    };

    cx.render(view)
}
