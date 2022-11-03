use std::borrow::BorrowMut;

use futures::channel::oneshot;
use protocol::{SessionId, Signal, UserId};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{BinaryType, RtcPeerConnection, WebSocket};

pub async fn connect_server(url: &str) -> WebSocket {
    let (tx, rx) = oneshot::channel();
    let ws = WebSocket::new(url).unwrap();
    ws.set_binary_type(BinaryType::Arraybuffer);

    let mut tx_opt = Some(tx);
    let mut ws_opt = Some(ws.clone());
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        tx_opt.take().unwrap().send(ws_opt.take().unwrap());
    });

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    rx.await.unwrap()
}

fn send_signal(ws: WebSocket, signal: Signal) {
    match ws.ready_state() {
        1 => {
            let message = match serde_json::to_string(&signal) {
                Ok(msg) => msg,
                Err(e) => {
                    log::error!("error serializing SessionNew{:?}: {:?}", signal, e);
                    return;
                }
            };
            let _ = ws.send_with_str(&message);
        }
        _ => {
            log::error!("web socket not opened");
        }
    };
}

pub fn handle_signal(ws: WebSocket, connection: RtcPeerConnection, signal: Signal) {
    match signal {
        Signal::NewUser(user_id) => todo!(),
        Signal::JoinSession(_) => todo!(),
        Signal::SessionCreated(session_id) => todo!(),
        Signal::JoinSessionSuccess(session_id) => todo!(),
        Signal::JoinSessionError(session_id) => todo!(),
        Signal::SdpOffer(session_id, user_id, offer) => todo!(),
        Signal::SdpAnswer(session_id, user_id, answer) => todo!(),
        Signal::ICECandidate(session_id, user_id, candidate) => todo!(),
        Signal::ICEError(session_id, message) => todo!(),
        Signal::NewSession => todo!(),
        Signal::Debug => todo!(),
    }
}

pub fn send_ice_candidate(
    ws: WebSocket,
    session_id: SessionId,
    user_id: UserId,
    candidate: String,
) {
    send_signal(ws, Signal::ICECandidate(session_id, user_id, candidate));
}

pub fn create_session(ws: WebSocket) {
    send_signal(ws, Signal::NewSession);
}

pub fn join_session(ws: WebSocket, session_id: SessionId) {
    send_signal(ws, Signal::JoinSession(session_id));
}

pub fn sdp_offer(ws: WebSocket, session_id: SessionId, user_id: UserId, offer: String) {
    send_signal(ws, Signal::SdpOffer(session_id, user_id, offer));
}

pub fn sdp_answer(ws: WebSocket, session_id: SessionId, user_id: UserId, offer: String) {
    send_signal(ws, Signal::SdpAnswer(session_id, user_id, offer));
}
