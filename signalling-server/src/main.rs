use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    env,
    str::FromStr,
    sync::Arc,
};

use protocol::RoomId;
use tokio::sync::Mutex;
use types::{Participants, Room, Rooms};
use warp::{Filter, Rejection, Reply};

mod participant_handler;
mod signal_handler;
mod types;

pub async fn ws_handler(
    ws: warp::ws::Ws,
    participants: Participants,
    rooms: Rooms,
) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| {
        participant_handler::when_participant_connected(socket, participants, rooms)
    }))
}

fn with_participants(
    participants: Participants,
) -> impl Filter<Extract = (Participants,), Error = Infallible> + Clone {
    warp::any().map(move || participants.clone())
}

fn with_rooms(rooms: Rooms) -> impl Filter<Extract = (Rooms,), Error = Infallible> + Clone {
    warp::any().map(move || rooms.clone())
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // this only shows access logs.
        env::set_var("RUST_LOG", "signalling-server=info");
    }
    let port = match env::var("port") {
        Ok(port) => port.parse::<u16>().unwrap(),
        _ => 9000,
    };
    pretty_env_logger::init();

    let participants: Participants = Arc::new(Mutex::new(HashMap::new()));

    let mut room_data = HashMap::new();
    let default_room_id = RoomId::from_str("7c50f4d3-30ad-4aab-886d-a27c6f5c804f").unwrap();

    log::info!("Default channel: {:?}", default_room_id);

    room_data.insert(
        default_room_id.clone(),
        Room {
            id: default_room_id.clone(),
            participants: HashSet::new(),
        },
    );

    let rooms: Rooms = Arc::new(Mutex::new(room_data));
    let ws_route = warp::any()
        .and(warp::ws())
        .and(with_participants(participants.clone()))
        .and(with_rooms(rooms.clone()))
        .and_then(ws_handler);

    warp::serve(ws_route).run(([127, 0, 0, 1], port)).await;
}
