use dioxus::{prelude::*, events::FormEvent};
use uuid::Uuid;

use crate::{chat_room::{functions::set_group, types::SetGroupInput}, wallet::WALLET};

pub fn NewChatRoomModal(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();
    let is_hidden = use_state(&cx, || true);
    let name = use_state(&cx, || "".to_string());

    let handle_submit = move |e: FormEvent| {
        let name = e.values["name"].clone();
        let wallet_clone = wallet.clone();
        cx.spawn({
            async move {
                set_group(wallet_clone, SetGroupInput {
                    id: Uuid::new_v4().to_string(),
                    name: name.clone()
                } ).await;
            }
        });
        is_hidden.set(!is_hidden);
    };
    let bg = match *is_hidden.clone() {
        true => "",
        false => "bg-black"
    };

    cx.render(rsx! {
        div {
            class: "relative",
            match *is_hidden.clone() {
                true => rsx!(
                    button {
                        onclick: move |_| { is_hidden.set(!is_hidden) },
                        class: " btn primary normal-case",
                        i { class: "fa-solid fa-plus"},
                    }),
                false => rsx!("")

            }
            div {
                class: "fixed top-0 left-0 w-screen h-screen pointer-events-none flex items-center justify-center {bg}",
                div {
                    class: "relative pointer-events-auto",
                    id:"my-modal",
                    hidden: "{is_hidden}",
                    form {
                        prevent_default: "onsubmit",
                        onsubmit: handle_submit,
                        div {
                            class:"modal-box p-3 m-3",
                            h3 {
                                class: "p-2",
                                "Name"
                            }
                            input {
                                name: "name",
                                value: "{name}",
                                class:"input input-bordered w-full p-2",
                                placeholder:"Type name here"  ,
                                oninput: move |e| name.set(e.value.clone()),
                            }
                            button {
                                r#type: "reset",
                                class: " btn primary normal-case m-3",
                                onclick: move |_| { is_hidden.set(!is_hidden) },
        
                                h1 {"Close"}
                            }
        
                            button {
                                class: " btn primary normal-case m-3",
                                h1 {"create"}
                            }
                        }
                    }
                }
            }
        }
    })
}
