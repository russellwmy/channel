use futures_util::{stream::StreamExt, FutureExt};
use protocol::{Signal, UserId};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};

use crate::{signal_handler, types::User, Channels, Users};

pub async fn when_user_connected(ws: WebSocket, users: Users, sessions: Channels) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|res| {
        if let Err(e) = res {
            log::error!("error sending websocket msg: {}", e);
        }
    }));

    let user_id = UserId::new(Uuid::new_v4().to_string());
    let user = User {
        channel_id: None,
        sender: client_sender.clone(),
        user_id: user_id.clone(),
    };

    users.lock().await.insert(user_id.clone(), user);

    let new_user_signal = Signal::NewUser(user_id.clone());
    let message = match serde_json::to_string(&new_user_signal) {
        Ok(msg) => msg,
        Err(e) => {
            log::error!("error serializing NewUser{:?}: {:?}", new_user_signal, e);
            return;
        }
    };

    match client_sender.send(Ok(Message::text(message))) {
        Err(e) => {
            log::error!("error sending NewUser{:?}: {:?}", new_user_signal, e);
        }
        _ => {}
    }

    while let Some(res) = client_ws_rcv.next().await {
        let msg = match res {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("error receving message: {}", e);
                break;
            }
        };
        log::info!("MSG: {:?}", msg);
        match signal_handler::when_signal_recieved(
            user_id.clone(),
            msg.clone(),
            users.clone(),
            sessions.clone(),
        )
        .await
        {
            Ok(()) => {
                log::info!("successfully handled message")
            }
            Err(e) => {
                log::error!("error hadnling message: {:?}, msg: {:?}", e, msg);
            }
        }
    }

    users.lock().await.remove(&user_id);
}
