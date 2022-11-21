use std::{cell::RefCell, rc::Rc, str::FromStr};

use protocol::{RoomId, Signal};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{MessageEvent, WebSocket};

use crate::{local_participant::LocalParticipant, participant::Participant, room::Room, signaling};

pub struct Client {
    room: Rc<RefCell<Room>>,
    handler: Rc<RefCell<Option<Box<dyn Fn(Signal)>>>>,
    ws: Option<WebSocket>,
    is_connected: bool,
}

impl Client {
    pub fn new() -> Self {
        let room = Rc::new(RefCell::new(Room::new()));
        let handler = Rc::new(RefCell::new(None));

        Self {
            room,
            handler,
            ws: None,
            is_connected: false,
        }
    }

    pub fn connected(&self) -> bool {
        self.is_connected
    }

    pub fn set_local_participant(&mut self, local_participant: LocalParticipant) {
        let rc_room = Rc::clone(&self.room);
        let mut room = (*rc_room).borrow_mut();
        room.set_local_participant(local_participant);
    }

    pub fn room(&self) -> Room {
        let rc_room = (*self).room.clone();
        let room = (*rc_room).borrow();

        room.clone()
    }

    pub async fn connect(&mut self, url: &str) {
        let rc_room = Rc::clone(&self.room);
        let mut room = (*rc_room).borrow_mut();
        let (ws, participant_id) = signaling::connect_server(url).await.unwrap();
        let mut local_participant = LocalParticipant::new_with_id(participant_id);

        let _ = local_participant.init_streaming().await;
        room.set_local_participant(local_participant);

        self.ws = Some(ws);
        self.is_connected = true;
        self.init();
    }

    pub fn join_room(&mut self, room_id: &str) {
        let cloned_ws = self.ws.clone().unwrap();
        signaling::join_room(cloned_ws, RoomId::from_str(room_id).unwrap());
    }

    pub fn set_onsignal(&mut self, handler: Box<dyn Fn(Signal) -> ()>) {
        let _ = self.handler = Rc::new(RefCell::new(Some(handler)));
    }

    fn init(&self) {
        let ws = self.ws.clone().unwrap();
        let rc_room = Rc::clone(&self.room);
        let rc_handler = (*self).handler.clone();

        let ws_ex = ws.clone();
        let on_callback = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
            let signal: Signal =
                serde_json::from_str(e.data().as_string().unwrap().as_str()).unwrap();

            match signal.clone() {
                Signal::NewParticipantJoined(room_id, participant_id) => {
                    let mut room = (*rc_room).borrow_mut();
                    let local_participant = room.clone().local_participant();
                    let ws = ws_ex.clone();
                    let cloned_local_participant = local_participant.clone();

                    if local_participant.is_some()
                        && local_participant.unwrap().id() != participant_id
                    {
                        log::info!("Received NewParticipantJoined");
                        let mut local_participant_ex = cloned_local_participant.unwrap();
                        let stream = local_participant_ex.stream();
                        let participant =
                            Participant::new(participant_id.clone(), room_id.clone(), ws.clone());

                        participant.publish(stream);
                        local_participant_ex.stop_streaming();
                        if local_participant_ex.id() < participant_id {
                            participant.create_and_send_offer();
                        }

                        room.add_participant(participant.clone());
                    }
                }

                Signal::JoinRoomSuccess(room_id) => {
                    log::info!("Received JoinRoomSuccess");
                    let mut room = (*rc_room).borrow_mut();
                    room.set_id(room_id);
                }

                Signal::SdpOffer(room_id, participant_id, offer) => {
                    log::info!("Received SdpOffer");
                    let ws = ws_ex.clone();
                    let room = (*rc_room).borrow();
                    let local_participant = room.local_participant().unwrap();

                    if local_participant.id() != participant_id.clone() {
                        let participant = room.get_participant(participant_id.clone());
                        log::info!("Received SdpOffer: {:?}", participant);
                        if let Some(participant) = participant {
                            participant.proccess_offer_and_send_answer(ws, room_id, offer);
                        }
                    }
                }

                Signal::SdpAnswer(_, participant_id, answer) => {
                    log::info!("Received SdpAnswer");
                    let room = (*rc_room).borrow();
                    let local_participant = room.local_participant().unwrap();

                    if local_participant.id() != participant_id.clone() {
                        let participant = room.get_participant(participant_id.clone());
                        log::info!("Proccess SdpAnswer: {:?}", participant);
                        if let Some(participant) = participant {
                            participant.process_sdp_answer(answer);
                        }
                    }
                }
                Signal::ICECandidate(_, participant_id, candidate) => {
                    log::info!("Received ICECandidate");
                    let room = (*rc_room).borrow();
                    let local_participant = room.local_participant().unwrap();

                    if local_participant.id() != participant_id.clone() {
                        let participant = room.get_participant(participant_id.clone());
                        log::info!("Proccess ICECandidate: {:?}", participant);
                        if let Some(participant) = participant {
                            participant.process_new_ice_candidate(candidate);
                        }
                    }
                }
                _ => {}
            };
            let ex_handler = (*rc_handler).borrow();
            if let Some(handler) = ex_handler.as_ref() {
                handler(signal.clone());
            }
        });

        ws.set_onmessage(Some(on_callback.as_ref().unchecked_ref()));
        on_callback.forget();
    }
}
