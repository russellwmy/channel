use dioxus::prelude::*;

use crate::{app, chat, chat_room, home, mic_check, user, wallet::WALLET};

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
        Route { to: "/lobby", chat_room::pages::LobbyPage{}}
        Route { to: "/room/:room_id", chat_room::pages::ChatRoomPage{}}
        Route { to: "", app::pages::PageNotFound{}}
    ))
}
