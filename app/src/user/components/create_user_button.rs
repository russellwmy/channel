use dioxus::{hooks::use_effect, prelude::*};
use log::info;

use crate::{
    user::{
        functions::{create_user, get_user_info},
        types::{GetUserInfoInput, NewUserInput},
    },
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
                    let result = create_user(
                        wallet_clone,
                        NewUserInput {
                            name: "steph".to_string(),
                            image: Some("steph_image".to_string()),
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
