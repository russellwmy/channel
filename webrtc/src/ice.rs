use js_sys::JSON;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    RtcIceCandidate,
    RtcIceCandidateInit,
    RtcPeerConnection,
    RtcPeerConnectionIceEvent,
    WebSocket,
};

use crate::{
    signalling,
    types::{IceCandidate, SessionState},
};

pub fn sleep(ms: i32) -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms)
            .unwrap();
    })
}

pub async fn attach_ice_candiate_callback(
    ws: WebSocket,
    peer: RtcPeerConnection,
    state: SessionState,
) -> Result<RtcPeerConnection, JsValue> {
    let onicecandidate_callback = Closure::wrap(Box::new(move |event: RtcPeerConnectionIceEvent| {
        let ws = ws.clone();
        let state = state.clone();

        send_ice_candidate(ws, state, event);
    })
        as Box<dyn FnMut(RtcPeerConnectionIceEvent)>);
    peer.set_onicecandidate(Some(onicecandidate_callback.as_ref().unchecked_ref()));
    onicecandidate_callback.forget();
    Ok(peer)
}

pub fn send_ice_candidate(ws: WebSocket, state: SessionState, event: RtcPeerConnectionIceEvent) {
    match event.candidate() {
        Some(candidate) => {
            let candidate_object = candidate.to_json();
            let candidate_string = JSON::stringify(&candidate_object).unwrap_throw();
            let result = String::from(candidate_string.clone());
            let ws = ws.clone();

            signalling::send_ice_candidate(ws, state.session_id, state.user_id, result);
        }
        None => {
            log::info!("No ICE candidate in RtcPeerConnectionIceEvent");
        }
    }
}

pub async fn process_new_ice_candidate(
    candidate: String,
    peer: RtcPeerConnection,
) -> Result<(), JsValue> {
    log::warn!("ICECandidate Received! {}", candidate);

    let ice_candidate = serde_json::from_str::<IceCandidate>(&candidate).map_err(|_| {
        let message = format!("Could not deserialize Ice Candidate {} ", candidate);
        JsValue::from_str(&message)
    })?;

    let mut new_ice_candidate_init = RtcIceCandidateInit::new("");
    new_ice_candidate_init.candidate(&ice_candidate.candidate);
    new_ice_candidate_init.sdp_m_line_index(Some(ice_candidate.sdp_m_line_index));
    new_ice_candidate_init.sdp_mid(Some(&ice_candidate.sdp_mid));

    match RtcIceCandidate::new(&new_ice_candidate_init) {
        Ok(x) => {
            let result =
                JsFuture::from(peer.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&x))).await?;
            log::info!("Added other peer's Ice Candidate ! {:?}", result);
        }
        Err(e) => {
            log::info!("Ice Candidate Addition error, {} | {:?}", candidate, e);
            return Err(e);
        }
    };
    Ok(())
}
