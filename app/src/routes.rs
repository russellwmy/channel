use dioxus::prelude::*;

use crate::{app, chat, home, mic_check, user, wallet::WALLET, chat_room};

#[derive(Props)]
struct PrivateProps<'a> {
    children: Element<'a>,
}

fn Private<'a>(cx: Scope<'a, PrivateProps<'a>>) -> Element {
    let wallet_state = use_atom_ref(&cx, WALLET);
    let connected = wallet_state.read().is_connected();
    let router = use_router(&cx);

    if !connected {
        router.push_route("/unauthorized", None, None);
    }

    cx.render(rsx!(&cx.props.children))
}

pub fn Routes(cx: Scope) -> Element {
    cx.render(rsx!(
        Route { to: "/", chat_room::pages::LobbyPage{}}
        Route { to: "/mic-check", mic_check::pages::MicCheckPage{}}
        Route { to: "/lobby", chat_room::pages::LobbyPage{}}
        Route { to: "/profile", user::pages::ProfilePage{}}
        Route { to: "", app::pages::PageNotFound{}}
    ))
}
