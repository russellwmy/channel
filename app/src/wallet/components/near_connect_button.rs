use dioxus::prelude::*;

use crate::wallet::{components::ConnectButton, WALLET};

pub fn NearConnectButton(cx: Scope) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let connected = wallet_state.read().is_connected();

    let handle_click = move |_| {
        let wallet_state = wallet_state.to_owned();
        match connected {
            true => {
                wallet_state.write().disconnect();
            }
            false => {
                cx.spawn({
                    async move {
                        wallet_state.write().connect(Some(vec![]), None, None).await;
                    }
                });
            }
        }
    };

    cx.render(rsx!(ConnectButton {
        connected: connected,
        onclick: handle_click
    }))
}
