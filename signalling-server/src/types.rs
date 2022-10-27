use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use protocol::{SessionId, UserId};
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

pub type Users = Arc<Mutex<HashMap<UserId, User>>>;
pub type Sessions = Arc<Mutex<HashMap<SessionId, Session>>>;

#[derive(Debug)]
pub struct Session {
    pub session_id: SessionId,
    pub users: HashSet<UserId>,
}

#[derive(Debug)]
pub struct User {
    pub sender: mpsc::UnboundedSender<Result<Message, warp::Error>>,
    pub session_id: Option<SessionId>,
    pub user_id: UserId,
}
