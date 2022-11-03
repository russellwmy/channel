use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use protocol::{ChannelId, UserId};
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

pub type Users = Arc<Mutex<HashMap<UserId, User>>>;
pub type Channels = Arc<Mutex<HashMap<ChannelId, Channel>>>;

#[derive(Debug)]
pub struct Channel {
    pub channel_id: ChannelId,
    pub users: HashSet<UserId>,
}

#[derive(Debug)]
pub struct User {
    pub sender: mpsc::UnboundedSender<Result<Message, warp::Error>>,
    pub channel_id: Option<ChannelId>,
    pub user_id: UserId,
}
