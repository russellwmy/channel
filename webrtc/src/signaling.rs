use futures::channel::oneshot::{self, Canceled};
use protocol::{ParticipantId, RoomId, Signal};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{BinaryType, MessageEvent, WebSocket};

pub async fn connect_server(url: &str) -> Result<(WebSocket, ParticipantId), Canceled> {
    let ws = WebSocket::new(url).unwrap();
    ws.set_binary_type(BinaryType::Arraybuffer);

    let (tx, rx) = oneshot::channel();
    let mut tx_opt = Some(tx);
    let mut ws_opt = Some(ws.clone());
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        let _ = tx_opt.take().unwrap().send(ws_opt.take().unwrap());
    });

    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();
    let onopen_rx = rx;

    let (tx, rx) = oneshot::channel();
    let mut tx_opt = Some(tx);
    let onmessage_callback = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
        let signal: Signal = serde_json::from_str(e.data().as_string().unwrap().as_str()).unwrap();
        match signal {
            Signal::ParticipantRegistered(participant_id) => {
                let _ = tx_opt.take().unwrap().send(participant_id);
            }
            _ => {}
        }
    });

    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();
    let onmessage_rx = rx;

    futures::try_join!(onopen_rx, onmessage_rx)
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

pub fn send_ice_candidate(
    ws: WebSocket,
    room_id: RoomId,
    participant_id: ParticipantId,
    candidate: String,
) {
    send_signal(ws, Signal::ICECandidate(room_id, participant_id, candidate));
}

pub fn create_room(ws: WebSocket) {
    send_signal(ws.clone(), Signal::CreateRoom);
}

pub fn join_room(ws: WebSocket, room_id: RoomId) {
    send_signal(ws, Signal::JoinRoom(room_id));
}

pub fn leave_room(ws: WebSocket, room_id: RoomId) {
    send_signal(ws, Signal::LeaveRoom(room_id));
}

pub fn send_sdp_offer(
    ws: WebSocket,
    room_id: RoomId,
    participant_id: ParticipantId,
    offer: String,
) {
    send_signal(ws, Signal::SdpOffer(room_id, participant_id, offer));
}

pub fn send_sdp_answer(
    ws: WebSocket,
    room_id: RoomId,
    participant_id: ParticipantId,
    answer: String,
) {
    send_signal(ws, Signal::SdpAnswer(room_id, participant_id, answer));
}
