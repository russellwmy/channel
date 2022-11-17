use futures::channel::oneshot;
use js_sys::JSON;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::{RtcIceCandidate, RtcIceCandidateInit, RtcPeerConnection, RtcPeerConnectionIceEvent};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IceCandidate {
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_m_line_index: u16,
}

pub async fn create_connection(connection: RtcPeerConnection) -> String {
    let (tx, rx) = oneshot::channel();
    let mut tx_opt = Some(tx);

    let on_callback = Closure::<dyn FnMut(_)>::new(move |e: RtcPeerConnectionIceEvent| {
        let data = e.candidate().unwrap();
        let candidate = data.to_json();
        let result = JSON::stringify(&candidate).unwrap_throw();
        let _ = tx_opt.take().unwrap().send(result.into());
    });
    connection.set_onicecandidate(Some(on_callback.as_ref().unchecked_ref()));
    on_callback.forget();

    rx.await.unwrap()
}

pub async fn process_new_ice_candidate(
    rtc_connection: RtcPeerConnection,
    candidate: String,
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
            let result = JsFuture::from(
                rtc_connection.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&x)),
            )
            .await?;
            log::info!("Added other peer's Ice Candidate ! {:?}", result);
        }
        Err(e) => {
            log::info!("Ice Candidate Addition error, {} | {:?}", candidate, e);
            return Err(e);
        }
    };
    Ok(())
}
