use serde::{Deserialize};
use url::Url;
use crate::error::error::DsorsError;
use crate::http::request;
use crate::http::request::send_req;

/**
This function will start up the connection to discord gateway api...
 */
pub fn start_connection() {
    // We need to get the most current gateway url...
    let gateway_url = match get_gateway_url() {
        Ok(url) => url,
        Err(error) => {
            // If we don't have the url - this means that we are unable to continue with the connection process, so we will panic..
            panic!("Error getting the gateway url, we will now exit... Error: {:?}", error);
        }
    };

    println!("gateway url: {}", gateway_url);
}

fn connect_to_ws(url: String) {}

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
    return match json {
        Ok(object) => { Ok(object.url) }
        Err(error) => { Err(error) }
    }
}