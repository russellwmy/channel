use dioxus::{events::FormEvent, prelude::*};

use crate::{
    user::{functions::set_user, types::SetUserInput},
    wallet::WALLET,
};

pub fn ProfilePage(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let name = use_state(&cx, || "".to_string());
    let wallet = wallet_state.read().wallet();
    let account_id = wallet_state.read().account_id();
    let account_id_cloned = account_id.clone();
    let wallet_clone = wallet.clone();

    let handle_submit = move |e: FormEvent| {
        let name = e.values["name"].clone();
        let wallet_clone = wallet.clone();
        if let Some(account_id) = account_id_cloned.clone() {
            cx.spawn({
                async move {
                    log::info!("name: {}", name);
                    let result = set_user(wallet_clone, SetUserInput { name, image: None }).await;
                }
            });
        }
    };
    let account_id = account_id.clone().unwrap();

    let view = rsx! {
        div {
            class: "w-screen h-screen flex items-center justify-center",
            form {
                prevent_default: "onsubmit",
                onsubmit: handle_submit,
                div {
                    class: "flex flex-col",
                    div {
                        class: "w-full max-w-xs mb-2",
                        div {
                            h3 {
                                class: "font-bold m-2",
                                "Welcome {account_id}!"
                            }

                        }
                    }
                    div {
                        class: "form-control w-full max-w-xs mb-2",
                        label {
                            class:"label",
                            "Name"
                        }
                        input {
                            class: "input input-bordered w-full max-w-xs",
                            name: "name",
                            value: "{name}",
                            oninput: move |evt| {
                                name.set(evt.value.clone());
                            },
                        }
                    }
                    div {
                        class: "form-control w-full max-w-xs",
                        button {
                            class: "btn primary normal-case gap-2",
                            i {
                                class: "fa-solid fa-user-plus"
                            }
                            h4 { " Update user"}
                        }
                    }
                }
            }
        }
    };

    cx.render(view)
}
