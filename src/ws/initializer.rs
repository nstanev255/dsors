use serde::{Deserialize};
use url::Url;
use crate::error::error;
use crate::http::request::send_req;

/**
This function will start up the connection to discord...
 */
pub fn start_connection() {
    // We need to get the most current gateway url...
    let gateway_url = match get_gateway_url() {
        Ok(url) => url,
        Err(_) => {
            // At this point we are unable to get the gateway url, so the program will not continue execution...
            panic!("Error getting the gateway url, we will now exit...");
        }
    };
}

fn connect_to_ws(url: String) {}

#[derive(Deserialize)]
struct GatewayUrlResponse {
    url: String,
}

fn get_gateway_url() -> Result<String, error::DsorsError> {
    let response: Result<GatewayUrlResponse, error::DsorsError> = send_req(Url::parse("https://discord.com/api/v10/gateway").unwrap());

    return match response {
        Ok(resp) => { Ok(resp.url) }
        Err(err) => { Err(err) }
    };
}