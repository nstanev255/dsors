use std::fmt::Debug;
use url::Url;
use tungstenite::connect as t_connect;

pub fn connect() {
    let url = Url::parse("wss://gateway.discord.gg/?v=9&encoding=json").expect("error");
    let (mut socket, _) =
        t_connect(url)
            .expect("error");

    // Send initial heartbeat event.
    //
    //

    let message = socket.read();
    if(message.is_ok()) {
        println!("message {}", message.unwrap().to_text().unwrap())
    }
}