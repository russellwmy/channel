use protocol::{ChannelId, Signal, UserId};
use uuid::Uuid;
use warp::ws::Message;

use crate::types::{Channel, Channels, User, Users};

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
    channels: Channels,
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
        Signal::NewChannel => {
            let channel_id = ChannelId::new(Uuid::new_v4().to_string());
            log::info!("Created new session: {:?}", channel_id);

            channels
                .lock()
                .await
                .entry(channel_id.clone())
                .or_insert(Channel {
                    channel_id: channel_id.clone(),
                    users: Default::default(),
                });

            match users.lock().await.get_mut(&sender_id) {
                Some(user) => {
                    user.channel_id = Some(channel_id.clone());
                    let sig_msg = Signal::ChannelCreated(channel_id.clone());
                    send_signal(&user, sig_msg).await?;
                }
                None => return Err(format!("can not find user {:?}", sender_id)),
            }
        }

        Signal::JoinChannel(channel_id) => match channels.lock().await.get_mut(&channel_id) {
            Some(session) => {
                session.users.insert(sender_id.clone());
                match users.lock().await.get_mut(&sender_id) {
                    Some(user) => {
                        let sig_msg = Signal::JoinChannelSuccess(channel_id);
                        send_signal(&user, sig_msg).await?;
                    }
                    None => return Err(format!("can not find user {:?}", sender_id)),
                }
            }
            None => match users.lock().await.get(&sender_id) {
                Some(user) => {
                    let sig_msg = Signal::JoinChannelError(channel_id);
                    send_signal(&user, sig_msg).await?;
                }
                None => return Err(format!("can not find user {:?}", sender_id)),
            },
        },

        Signal::SdpOffer(channel_id, recipient_id, offer) => {
            match channels.lock().await.get(&channel_id) {
                Some(_) => match users.lock().await.get(&recipient_id) {
                    Some(recipient) => {
                        let sig_msg = Signal::SdpOffer(channel_id, sender_id, offer);

                        send_signal(&recipient, sig_msg).await?;
                    }
                    None => return Err(format!("can not find user {:?}", recipient_id)),
                },

                None => return Err(format!("can not find session {:?}", channel_id)),
            }
        }

        Signal::SdpAnswer(channel_id, recipient_id, offer) => {
            match channels.lock().await.get(&channel_id) {
                Some(_) => match users.lock().await.get(&recipient_id) {
                    Some(recipient) => {
                        let sig_msg = Signal::SdpAnswer(channel_id, sender_id, offer);
                        send_signal(&recipient, sig_msg).await?;
                    }
                    None => return Err(format!("can not find user {:?}", recipient_id)),
                },
                None => return Err(format!("can not find session {:?}", channel_id)),
            }
        }

        Signal::ICECandidate(channel_id, recipient_id, candidate) => {
            match channels.lock().await.get(&channel_id) {
                Some(_) => {
                    log::info!(
                        "Got ICECandidate: user_id: {:#?}, session: {:#?}",
                        sender_id,
                        channel_id
                    );
                    match users.lock().await.get(&recipient_id) {
                        Some(recipient) => {
                            let sig_msg = Signal::ICECandidate(channel_id, sender_id, candidate);
                            send_signal(&recipient, sig_msg).await?;
                        }
                        None => return Err(format!("can not find user {:?}", recipient_id)),
                    }
                }
                None => return Err(format!("can not find session {:?}", channel_id)),
            }
        }
        _ => {}
    }

    Ok(())
}
