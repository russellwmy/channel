use js_sys::JSON;
use protocol::{ParticipantId, RoomId};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    MediaStream,
    MediaStreamTrack,
    RtcDataChannel,
    RtcPeerConnection,
    RtcPeerConnectionIceEvent,
    RtcTrackEvent,
    WebSocket,
};

use crate::{dom, ice, sdp, signaling};

#[derive(Debug, Clone)]
pub struct Participant {
    id: ParticipantId,
    room_id: RoomId,
    ws: WebSocket,
    streams: Vec<MediaStream>,
    connection: RtcPeerConnection,
}

impl Participant {
    pub fn new(id: ParticipantId, room_id: RoomId, ws: WebSocket) -> Self {
        let connection = ice::create_connection();
        let cloned_ws = ws.clone();
        let cloned_room_id = room_id.clone();
        let cloned_id = id.clone();
        let onicecandidate_callback = Closure::<dyn FnMut(RtcPeerConnectionIceEvent)>::new(
            move |e: RtcPeerConnectionIceEvent| {
                match e.candidate() {
                    Some(candidate) => {
                        let data = candidate.to_json();
                        let result = JSON::stringify(&data).unwrap();

                        signaling::send_ice_candidate(
                            cloned_ws.clone(),
                            cloned_room_id.clone(),
                            cloned_id.clone(),
                            result.into(),
                        );
                    }
                    _ => {}
                };
            },
        );
        connection.set_onicecandidate(Some(onicecandidate_callback.as_ref().unchecked_ref()));
        onicecandidate_callback.forget();

        let ontrack_callback = Closure::<dyn FnMut(RtcTrackEvent)>::new(move |e: RtcTrackEvent| {
            log::info!("ontrack");
            for stream in e.streams().iter() {
                dom::attach_stream(stream.into());
            }
        });
        connection.set_ontrack(Some(ontrack_callback.as_ref().unchecked_ref()));
        ontrack_callback.forget();

        Self {
            id,
            connection,
            ws,
            room_id,
            streams: vec![],
        }
    }

    pub fn id(&self) -> ParticipantId {
        self.id.clone()
    }

    pub fn connection(&self) -> RtcPeerConnection {
        self.connection.clone()
    }

    pub fn publish(&self, stream: &MediaStream) {
        for track in stream.get_tracks().iter() {
            let track = track.dyn_into::<MediaStreamTrack>().unwrap();
            log::info!("track id: {}", track.id());
            self.connection.add_track_0(&track, &stream);
        }
    }

    pub fn create_and_send_offer(&self) {
        let participant_id = self.id.clone();
        let connection = self.connection.clone();
        let room_id = self.room_id.clone();
        let ws = self.ws.clone();

        spawn_local(async move {
            if let Ok(offer) = sdp::create_sdp_offer(connection.clone()).await {
                log::info!("LocalParticipant - create_and_send_offer");
                signaling::send_sdp_offer(
                    ws.clone(),
                    room_id.clone(),
                    participant_id.clone(),
                    offer,
                );
            }
        });
    }

    pub fn proccess_offer_and_send_answer(&self, ws: WebSocket, room_id: RoomId, offer: String) {
        let participant_id = self.id.clone();
        let connection = self.connection.clone();
        let room_id = self.room_id.clone();

        spawn_local(async move {
            let answer = sdp::process_sdp_offer(connection.clone(), offer)
                .await
                .unwrap();
            signaling::send_sdp_answer(ws.clone(), room_id.clone(), participant_id.clone(), answer);
        });
    }

    pub fn process_sdp_answer(&self, answer: String) {
        let connection = self.connection.clone();

        spawn_local(async move {
            let _ = sdp::process_sdp_answer(connection.clone(), answer).await;
        });
    }

    pub fn process_new_ice_candidate(&self, candidate: String) {
        let connection = self.connection.clone();

        spawn_local(async move {
            let _ = ice::process_new_ice_candidate(connection.clone(), candidate).await;
        });
    }
}
