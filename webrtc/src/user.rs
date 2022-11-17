use js_sys::JSON;
use protocol::PeerId;
use uuid::Uuid;
use wasm_bindgen::prelude::Closure;
use web_sys::{MediaStream, RtcDataChannel, RtcPeerConnection, RtcPeerConnectionIceEvent};

#[derive(Clone)]
pub struct Partial {
    initiator: bool,
    id: Option<PeerId>,
    connection: RtcPeerConnection,
    channel: Option<RtcDataChannel>,
    streams: Vec<MediaStream>,
}

impl Peer {
    pub fn new() -> Result<Self, String >{
        let connection = RtcPeerConnection::new().map_err(|e| "Fail to init connection")?;
        let onicecandidate_callback =
        Closure::<dyn FnMut(RtcPeerConnectionIceEvent)>::new(
            move |e: RtcPeerConnectionIceEvent| {
                log::info!("NewPeerJoined - oncandidate");
                match e.candidate() {
                    Some(candidate) => {
                        let data = candidate.to_json();
                        let result = JSON::stringify(&data).unwrap();

                        signaling::send_ice_candidate(
                            cloned_ws.clone(),
                            cloned_channel_id.clone(),
                            cloned_peer_id.clone(),
                            result.into(),
                        );
                    }
                    _ => {}
                };
            },
        );
    connection.set_onicecandidate(Some(
        onicecandidate_callback.as_ref().unchecked_ref(),
    ));
    onicecandidate_callback.forget();
       Ok(Self {
            id: None,
            initiator: false,
            connection: connection,
            channel: None,
            streams: vec![],
        })
    }

    pub fn id(&self) -> Option<PeerId> {
        self.id
    }

    pub fn set_id(&mut self, id: PeerId) {
        self.id = Some(id);
    }

    pub fn initiator(&self) -> bool {
        self.initiator
    }

    pub fn set_initiator(&mut self, initiator: bool) {
        self.initiator = initiator;
    }

    pub fn streams(&self) -> Vec<MediaStream> {
        self.streams
    }

    pub fn add_stream(&mut self, stream: MediaStream) {
        self.streams.push(stream);
    }

    pub fn remove_stream(&mut self, stream: MediaStream) {
        let pos = self.streams.iter().position(|s|s.id() == stream.id());
        if let Some(pos) = pos {
            self.streams.remove(pos);
        }
    }


}

