use js_sys::Reflect;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RtcPeerConnection, RtcSdpType, RtcSessionDescriptionInit};

pub async fn process_sdp_answer(
    connection: RtcPeerConnection,
    answer: String,
) -> Result<(), JsValue> {
    log::debug!("SDP: Receive Answer {:?}", answer);

    // Setting Remote Description
    let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
    answer_obj.sdp(&answer);
    let srd_promise = connection.set_remote_description(&answer_obj);
    JsFuture::from(srd_promise).await?;
    Ok(())
}

pub async fn process_sdp_offer(
    connection: RtcPeerConnection,
    offer: String,
) -> Result<String, JsValue> {
    log::debug!("SDP: SDP Offer Receive! {}", offer);

    // Set Remote Description
    let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    offer_obj.sdp(&offer);
    let srd_promise = connection.set_remote_description(&offer_obj);
    JsFuture::from(srd_promise).await?;

    // Create SDP Answer
    let answer = JsFuture::from(connection.create_answer()).await?;
    let answer_sdp = Reflect::get(&answer, &JsValue::from_str("sdp"))?
        .as_string()
        .unwrap();

    let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
    answer_obj.sdp(&answer_sdp);

    let sld_promise = connection.set_local_description(&answer_obj);
    JsFuture::from(sld_promise).await?;

    log::debug!("SDP: Sending SDP Answer {:?}", answer_sdp);
    Ok(answer_sdp)
}

pub async fn create_sdp_offer(connection: RtcPeerConnection) -> Result<String, JsValue> {
    // Create SDP Offer
    let offer = JsFuture::from(connection.create_offer()).await?;
    let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))?
        .as_string()
        .unwrap();

    // Set SDP Type -> Offer
    let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    offer_obj.sdp(&offer_sdp);

    // Set SDP Type -> Offer
    let sld_promise = connection.set_local_description(&offer_obj);
    JsFuture::from(sld_promise).await?;

    // Send Offer from Peer A -> Peer B Via WebSocket
    log::debug!("SDP: Sending Offer {:?}", offer_sdp);

    Ok(offer_sdp)
}
