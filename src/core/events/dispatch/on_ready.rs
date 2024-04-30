use serde::Deserialize;
use crate::core::websocket::WsConnection;

#[derive(Deserialize, Debug)]
struct User {
    // verified: bool,
    username: String,
    id: String,
    // email: Option<String>,
    // bot: bool,
    avatar: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    user: User
}

#[derive(Deserialize, Debug)]
struct OnReady {
    d: Data
}


pub async fn on_ready(ws_connection: &mut WsConnection, req_message: String) {
    let on_ready_message: OnReady = serde_json::from_str(req_message.as_str()).unwrap();

    // On the OnReady event we need to just set the needed fields.
    ws_connection.bot.avatar = on_ready_message.d.user.avatar;
    ws_connection.bot.username = on_ready_message.d.user.username;
    ws_connection.bot.id = on_ready_message.d.user.id;

    //TODO: plug the onReady user defined event...

}