use std::net::TcpStream;
use serde::{Deserialize};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
use url::Url;
use crate::core::websocket::{connect_ws};
use crate::error::dsors_error::DsorsError;
use crate::http::request;
use crate::http::request::send_req;

/**
This function will start up the connection to discord gateway api...
 */
pub fn start_connection() -> WebSocket<MaybeTlsStream<TcpStream>> {
    // We need to get the most current gateway url...
    let gateway_url = match get_gateway_url() {
        Ok(url) => url,
        Err(error) => {
            // If we don't have the url - this means that we are unable to continue with the connection process, so we will panic..
            panic!("Error getting the gateway url, we will now exit... Error: {:?}", error);
        }
    };

    println!("gateway url: {}", gateway_url);
    // Initialize the connection to the websocket...

    let socket = connect_ws(Url::parse(gateway_url.as_str()).unwrap());
    match socket {
        Ok(socket) => { println!("successfully connected to discord !"); socket }
        Err(err) => {
            // If we can't connect to discord we will just print out an error...
            panic!("Error connecting to {}. Error: {:?}", gateway_url, err );
        }
    }

}


#[derive(Deserialize)]
struct GatewayUrlResponse {
    url: String,
}

fn get_gateway_url() -> Result<String, DsorsError> {
    let resp = match send_req(Url::parse("https://discord.com/api/v10/gateway").unwrap()) {
        Ok(resp) => { resp }
        Err(err) => { return Err(err) }
    };

    let json: Result<GatewayUrlResponse, DsorsError> = request::response_to_json(resp);
    match json {
        Ok(object) => { Ok(object.url) }
        Err(error) => { Err(error) }
    }
}