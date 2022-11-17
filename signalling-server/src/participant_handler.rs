use std::borrow::BorrowMut;

use futures_util::{stream::StreamExt, FutureExt};
use protocol::{ParticipantId, Signal};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

use crate::{
    signal_handler,
    types::{Participant, Participants, Rooms},
};

pub async fn when_participant_connected(
    ws: WebSocket,
    participants: Participants,
    channels: Rooms,
) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|res| {
        if let Err(e) = res {
            log::error!("error sending websocket msg: {}", e);
        }
    }));

    let participant_id = ParticipantId::new();
    let participant = Participant {
        room_id: None,
        id: participant_id.clone(),
        sender: client_sender.clone(),
    };

    participants
        .lock()
        .await
        .insert(participant_id.clone(), participant);

    let new_participant_signal = Signal::ParticipantRegistered(participant_id.clone());
    let message = match serde_json::to_string(&new_participant_signal) {
        Ok(msg) => msg,
        Err(e) => {
            log::error!(
                "error serializing ParticipantRegistered {:?}: {:?}",
                new_participant_signal,
                e
            );
            return;
        }
    };

    match client_sender.send(Ok(Message::text(message))) {
        Err(e) => {
            log::error!(
                "error sending ParticipantRegistered {:?}: {:?}",
                new_participant_signal,
                e
            );
        }
        _ => {}
    }

    while let Some(res) = client_ws_rcv.next().await {
        let message = match res {
            Ok(message) => message,
            Err(e) => {
                log::error!("error receving message: {}", e);
                break;
            }
        };

        match signal_handler::when_signal_recieved(
            participant_id.clone(),
            message.clone(),
            participants.clone(),
            channels.clone(),
        )
        .await
        {
            Ok(()) => {
                log::info!("successfully handled message")
            }
            Err(e) => {
                log::error!("error hadnling message: {:?}, msg: {:?}", e, message);
            }
        }
    }

    for (_, channel) in channels.lock().await.iter_mut() {
        (*channel).participants.borrow_mut().remove(&participant_id);
    }

    participants.lock().await.remove(&participant_id);
}
