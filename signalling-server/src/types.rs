use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use protocol::{ParticipantId, RoomId};
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

pub type Participants = Arc<Mutex<HashMap<ParticipantId, Participant>>>;
pub type Rooms = Arc<Mutex<HashMap<RoomId, Room>>>;

#[derive(Debug)]
pub struct Room {
    pub id: RoomId,
    pub participants: HashSet<ParticipantId>,
}

#[derive(Debug)]
pub struct Participant {
    pub id: ParticipantId,
    pub sender: mpsc::UnboundedSender<Result<Message, warp::Error>>,
    pub room_id: Option<RoomId>,
}
