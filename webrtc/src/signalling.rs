use protocol::{SessionId, Signal, UserId};
use wasm_bindgen::JsValue;
use web_sys::{BinaryType, ErrorEvent, MediaStream, RtcPeerConnection, WebSocket};

pub fn connect_server(url: &str) -> Result<WebSocket, JsValue> {
    let ws = WebSocket::new(url)?;
    ws.set_binary_type(BinaryType::Arraybuffer);

    Ok(ws)
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