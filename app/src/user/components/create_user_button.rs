use dioxus::prelude::*;

use crate::{
    user::{functions::set_user, types::NewUserInput},
    wallet::WALLET,
};

pub fn CreateUserButton(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();
    let account = wallet_state.read().account_id();
    let account_clone = account.clone();
    let wallet_clone = wallet.clone();

    let handle_click = move |_| {
        let wallet_clone = wallet.clone();
        if let Some(account_id) = account.clone() {
            cx.spawn({
                async move {
                    let result = set_user(
                        wallet_clone,
                        NewUserInput {
                            name: "steph".to_string(),
                            image: None,
                        },
                    )
                    .await;
                }
            });
        }
    };

    let view = rsx! {
        div {
            // key: cx.props.id,
            button {
                onclick: handle_click,
                class: "btn primary normal-case",
                i {
                    class: "fa-solid fa-user-plus"
                }
                h4 { " Create user"}
            }
        }
    };

    cx.render(view)
}
