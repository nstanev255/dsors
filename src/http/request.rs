use reqwest::blocking::Response;
use serde::{Deserialize};
use url::Url;
use crate::error::error::DsorsError;

pub fn send_req(url: Url) -> Result<Response, DsorsError>
{
    let req_url = url.as_str();

     match reqwest::blocking::get(req_url) {
        Ok(resp) => Ok(resp),
        Err(err) => { return Err(DsorsError::new(format!("Error sending GET request... Error: {:?}", err).as_str())); }
    }
}

fn get_req_body(response: Response) -> Result<String, DsorsError> {
    match response.text() {
        Ok(text) => { Ok(text) }
        Err(err) => { return Err(DsorsError::new(format!("Error reading response body.. Error: {:?}", err).as_str())) }
    }
}

pub fn response_to_json<T>(response: Response) -> Result<T, DsorsError>
    where T: for<'a> Deserialize<'a>
{
    let json = match get_req_body(response) {
        Ok(json) => { json }
        Err(error) => { return Err(error); }
    };

    let result: serde_json::Result<T> = serde_json::from_str(json.as_str());

    return match result {
        Ok(object) => { Ok(object) }
        Err(error) => { return Err(DsorsError::new(format!("Error reading json from response .... {:?}", error).as_str())); }
    };
}