use protocol::{ParticipantId, RoomId, Signal};
use warp::ws::Message;

use crate::types::{Participant, Participants, Room, Rooms};

pub async fn send_signal(participant: &Participant, signal: Signal) -> Result<(), String> {
    log::info!(
        "Sending to user: {:#?} signal: {:#?}",
        participant.id,
        signal
    );
    let message = match serde_json::to_string(&signal) {
        Ok(msg) => msg,
        Err(_) => return Err(format!("can not serialize signal: {:?}", signal)),
    };

    match participant.sender.send(Ok(Message::text(message))) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn when_signal_recieved(
    sender_id: ParticipantId,
    message: Message,
    participants: Participants,
    rooms: Rooms,
) -> Result<(), String> {
    let message = match message.to_str() {
        Ok(m) => m,
        Err(_) => {
            return Err("message is not a str".to_string());
        }
    };

    let result: Signal = match serde_json::from_str(&message) {
        Ok(x) => x,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    log::info!("Handling signal: {:#?}", result);

    match result {
        Signal::CreateRoom => {
            let room_id = RoomId::new();
            log::info!("Created new room: {:?}", room_id);

            rooms.lock().await.entry(room_id.clone()).or_insert(Room {
                id: room_id.clone(),
                participants: Default::default(),
            });

            match participants.lock().await.get_mut(&sender_id) {
                Some(participant) => {
                    participant.room_id = Some(room_id.clone());
                    let sig_msg = Signal::RoomCreated(room_id.clone());

                    send_signal(&participant, sig_msg).await?;
                }
                None => return Err(format!("can not find user {:?}", sender_id)),
            }
        }

        Signal::JoinRoom(room_id) => match rooms.lock().await.get_mut(&room_id) {
            Some(room) => {
                room.participants.insert(sender_id.clone());
                let mut others = room.participants.clone();
                others.remove(&sender_id);

                match participants.lock().await.get_mut(&sender_id) {
                    Some(sender) => {
                        let sig_msg = Signal::JoinRoomSuccess(room_id.clone());
                        send_signal(&sender, sig_msg).await?;

                        for participant_id in others.clone() {
                            let sig_msg = Signal::NewParticipantJoined(
                                room_id.clone(),
                                participant_id.clone(),
                            );
                            send_signal(&sender, sig_msg).await?;
                        }
                    }
                    None => return Err(format!("can not find user {:?}", sender_id)),
                }

                for participant_id in others.clone() {
                    match participants.lock().await.get_mut(&participant_id) {
                        Some(participant) => {
                            let sig_msg =
                                Signal::NewParticipantJoined(room_id.clone(), sender_id.clone());
                            send_signal(&participant, sig_msg).await?;
                        }
                        None => {}
                    }
                }
            }
            None => match participants.lock().await.get(&sender_id) {
                Some(user) => {
                    let sig_msg = Signal::JoinRoomError(room_id);
                    send_signal(&user, sig_msg).await?;
                }
                None => return Err(format!("can not find user {:?}", sender_id)),
            },
        },

        Signal::SdpOffer(room_id, participant_id, offer) => {
            match rooms.lock().await.get(&room_id) {
                Some(_) => match participants.lock().await.get_mut(&participant_id) {
                    Some(participant) => {
                        let sig_msg =
                            Signal::SdpOffer(room_id.clone(), sender_id.clone(), offer.clone());
                        send_signal(&participant, sig_msg).await?;
                    }
                    None => {}
                },
                None => return Err(format!("can not find room {:?}", room_id)),
            }
        }

        Signal::SdpAnswer(room_id, participant_id, offer) => {
            match rooms.lock().await.get(&room_id) {
                Some(_) => match participants.lock().await.get_mut(&participant_id) {
                    Some(participant) => {
                        let sig_msg =
                            Signal::SdpAnswer(room_id.clone(), sender_id.clone(), offer.clone());
                        send_signal(&participant, sig_msg).await?;
                    }
                    None => {}
                },
                None => return Err(format!("can not find room {:?}", room_id)),
            }
        }

        Signal::ICECandidate(room_id, participant_id, candidate) => {
            match rooms.lock().await.get(&room_id) {
                Some(_) => match participants.lock().await.get_mut(&participant_id) {
                    Some(user) => {
                        let sig_msg = Signal::ICECandidate(
                            room_id.clone(),
                            sender_id.clone(),
                            candidate.clone(),
                        );
                        send_signal(&user, sig_msg).await?;
                    }
                    None => {}
                },
                None => return Err(format!("can not find room {:?}", room_id)),
            }
        }
        _ => {}
    }

    Ok(())
}
