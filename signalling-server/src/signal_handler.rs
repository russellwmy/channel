use protocol::{SessionId, Signal, UserId};
use uuid::Uuid;
use warp::ws::Message;

use crate::types::{Session, Sessions, User, Users};

pub async fn send_signal(user: &User, signal: Signal) -> Result<(), String> {
    log::info!("Sending to user: {:#?} signal: {:#?}", user.user_id, signal);
    let message = match serde_json::to_string(&signal) {
        Ok(msg) => msg,
        Err(_) => return Err(format!("can not serialize signal: {:?}", signal)),
    };

    match user.sender.send(Ok(Message::text(message))) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn when_signal_recieved(
    sender_id: UserId,
    msg: Message,
    users: Users,
    sessions: Sessions,
) -> Result<(), String> {
    let msg = match msg.to_str() {
        Ok(m) => m,
        Err(_) => {
            return Err("message is not a str".to_string());
        }
    };

    let result: Signal = match serde_json::from_str(&msg) {
        Ok(x) => x,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    log::info!("Handling signal: {:#?}", result);

    match result {
        Signal::NewSession => {
            let session_id = SessionId::new(Uuid::new_v4().to_string());
            log::info!("Created new session: {:?}", session_id);

            sessions
                .lock()
                .await
                .entry(session_id.clone())
                .or_insert(Session {
                    session_id: session_id.clone(),
                    users: Default::default(),
                });

            match users.lock().await.get_mut(&sender_id) {
                Some(user) => {
                    user.session_id = Some(session_id.clone());
                    let sig_msg = Signal::SessionCreated(session_id.clone());
                    send_signal(&user, sig_msg).await?;
                }
                None => return Err(format!("can not find user {:?}", sender_id)),
            }
        }

        Signal::JoinSession(session_id) => match sessions.lock().await.get_mut(&session_id) {
            Some(session) => {
                session.users.insert(sender_id.clone());
                match users.lock().await.get_mut(&sender_id) {
                    Some(user) => {
                        let sig_msg = Signal::JoinSessionSuccess(session_id);
                        send_signal(&user, sig_msg).await?;
                    }
                    None => return Err(format!("can not find user {:?}", sender_id)),
                }
            }
            None => match users.lock().await.get(&sender_id) {
                Some(user) => {
                    let sig_msg = Signal::JoinSessionError(session_id);
                    send_signal(&user, sig_msg).await?;
                }
                None => return Err(format!("can not find user {:?}", sender_id)),
            },
        },

        Signal::SdpOffer(session_id, recipient_id, offer) => {
            match sessions.lock().await.get(&session_id) {
                Some(_) => match users.lock().await.get(&recipient_id) {
                    Some(recipient) => {
                        let sig_msg = Signal::SdpOffer(session_id, sender_id, offer);

                        send_signal(&recipient, sig_msg).await?;
                    }
                    None => return Err(format!("can not find user {:?}", recipient_id)),
                },

                None => return Err(format!("can not find session {:?}", session_id)),
            }
        }

        Signal::SdpAnswer(session_id, recipient_id, offer) => {
            match sessions.lock().await.get(&session_id) {
                Some(_) => match users.lock().await.get(&recipient_id) {
                    Some(recipient) => {
                        let sig_msg = Signal::SdpAnswer(session_id, sender_id, offer);
                        send_signal(&recipient, sig_msg).await?;
                    }
                    None => return Err(format!("can not find user {:?}", recipient_id)),
                },
                None => return Err(format!("can not find session {:?}", session_id)),
            }
        }

        Signal::ICECandidate(session_id, recipient_id, candidate) => {
            match sessions.lock().await.get(&session_id) {
                Some(_) => {
                    log::info!(
                        "Got ICECandidate: user_id: {:#?}, session: {:#?}",
                        sender_id,
                        session_id
                    );
                    match users.lock().await.get(&recipient_id) {
                        Some(recipient) => {
                            let sig_msg = Signal::ICECandidate(session_id, sender_id, candidate);
                            send_signal(&recipient, sig_msg).await?;
                        }
                        None => return Err(format!("can not find user {:?}", recipient_id)),
                    }
                }
                None => return Err(format!("can not find session {:?}", session_id)),
            }
        }
        _ => {}
    }

    Ok(())
}
