use serde::{Deserialize};
use url::Url;
use crate::error::dsors_error::DsorsError;
use crate::http::request;
use crate::http::request::send_req;


#[derive(Deserialize)]
struct GatewayUrlResponse {
    url: String,
}

pub fn get_gateway_url() -> Result<String, DsorsError> {
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