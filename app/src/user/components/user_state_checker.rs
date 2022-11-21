use dioxus::prelude::*;

use crate::{
    user::{
        functions::{get_user, set_user},
        types::SetUserInput,
    },
    wallet::WALLET,
};

pub fn UserStateChecker(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let wallet = wallet_state.read().wallet();
    let account_id = wallet_state.read().account_id();
    let account_id_clone = account_id.clone();
    let wallet_clone = wallet.clone();

    let user_fut = use_future(&cx, (), |_| async move {
        match account_id_clone {
            Some(account_id) => {
                let user = get_user(
                    wallet_clone.clone(),
                    crate::user::types::GetUserInput { account_id },
                )
                .await;

                match user {
                    Ok(user) => Some(user),
                    _ => None,
                }
            }
            _ => None,
        }
    });

    let handle_click = move |_| {
        let wallet_clone = wallet.clone();
        if let Some(account_id) = account_id.clone() {
            cx.spawn({
                async move {
                    let result = set_user(
                        wallet_clone,
                        SetUserInput {
                            name: account_id.to_string(),
                            image: None,
                        },
                    )
                    .await;
                }
            });
        }
    };

    cx.render(rsx! {
        div {
            match user_fut.value() {
                Some(value) => {
                    match value {
                        Some(_) => rsx!(""),
                        None => rsx!(
                            button {
                                
                                onclick: handle_click,
                                class: "p-2 text-info",
                                span {
                                    class: "text-xl mr-2",
                                    i {
                                        class: "fa-solid fa-circle-info"
                                    }
                                }                               
                            }
                        )
                    }
                }
                None => rsx!(""),
            }

        }
    })
}
