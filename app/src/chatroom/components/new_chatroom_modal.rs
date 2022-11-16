use dioxus::{events::MouseEvent, prelude::*};
use log::info;
use crate::{
    wallet::WALLET,
    chatroom::{
        functions::set_group
    },
};


pub fn NewChatroomModal(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();


    let is_modal_visible = use_state(&cx, || false);
    // let name = use_state(&cx, || "".to_string());

    let view = rsx! {
        div {
            class: "absolute",
            button {
                onclick: move |_| { is_modal_visible.set(!is_modal_visible) },
                class: " btn primary normal-case", 
                i {
                   class: "fa-solid fa-plus"
                }
            }
            div {
                id:"my-modal",
                hidden: "{is_modal_visible}",
                div {
                    class:"modal-box p-3 m-3",
                    h3 {"Name"}
                    // <input type="text" class="input w-full max-w-xs" />
                    input {
                        // name: "name",
                        class:"input w-full m-3",
                        // placeholder:"Type here"  ,
            
                        // oninput: move |evt| name.set(evt.value.clone()),
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
                                    set_group(wallet_clone, "chatroom".to_string()).await;
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
    };

    cx.render(view)
}