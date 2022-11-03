use std::{collections::HashMap, convert::Infallible, env, sync::Arc};

use tokio::sync::Mutex;
use types::{Channels, Users};
use warp::{Filter, Rejection, Reply};
mod signal_handler;
mod types;
mod user_handler;

pub async fn ws_handler(
    ws: warp::ws::Ws,
    users: Users,
    channels: Channels,
) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| user_handler::when_user_connected(socket, users, channels)))
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

fn with_channels(
    channels: Channels,
) -> impl Filter<Extract = (Channels,), Error = Infallible> + Clone {
    warp::any().map(move || channels.clone())
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

    let users: Users = Arc::new(Mutex::new(HashMap::new()));
    let channels: Channels = Arc::new(Mutex::new(HashMap::new()));

    let ws_route = warp::any()
        .and(warp::ws())
        .and(with_users(users.clone()))
        .and(with_channels(channels.clone()))
        .and_then(ws_handler);

    warp::serve(ws_route).run(([127, 0, 0, 1], port)).await;
}
