use std::io::{Read};
use reqwest::blocking::Response;
use serde::{Deserialize};
use url::Url;
use crate::error::error::DsorsError;

pub fn send_req<T>(url: Url) -> Result<T, DsorsError>
    where T: for<'a> Deserialize<'a>
{
    let req_url = url.as_str();

    // Send request
    let mut response = match reqwest::blocking::get(req_url) {
        Ok(resp) => resp,
        Err(err) => { return Err(DsorsError::new(format!("Error sending GET request... Error: {:?}", err).as_str())); }
    };

    // Read the json string from the request...
    let json_body = match get_req_body(&mut response) {
        Ok(response) => { response }
        Err(error) => { return Err(error); }
    };

    let object: Result<T, DsorsError> = response_to_json(&mut response);
    return match object {
        Ok(obj) => { Ok(obj) }
        Err(error) => { return Err(error); }
    };
}

pub fn get_req_body(response: &mut Response) -> Result<String, DsorsError> {
    let mut json_str = String::new();

    match response.read_to_string(&mut json_str) {
        Ok(_) => Ok(json_str),
        Err(err) => { return Err(DsorsError::new(format!("Error reading response body.. Error: {:?}", err).as_str())); }
    }
}

fn response_to_json<T>(response: &mut Response) -> Result<T, DsorsError>
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