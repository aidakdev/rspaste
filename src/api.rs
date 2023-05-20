use thiserror::Error;
use crate::types::{GetResponse, PostResponse, DeleteResponse};

#[derive(Debug, Error)]
pub enum Error {
    #[error("[rspaste] There was an issue while performing the request.")]
    Ureq(#[from] ureq::Error),
    #[error("[rspaste] There was an issue while deserializing a response to JSON.")]
    Io(#[from] std::io::Error),
    #[error("[rspaste] There was an issue while fitting JSON to a strong type.")]
    Json(#[from] serde_path_to_error::Error<serde_json::Error>),
}

const BASE_URL: &str = "https://jspaste.tnfangel.repl.co/documents/";

pub type Result<T> = std::result::Result<T, Error>;

pub fn get(key: &str) -> Result<GetResponse> {
    let endpoint = format!("{}{}", BASE_URL, key);

    let response = ureq::get(&endpoint).call()?;
    let string = response.into_string()?;

    let final_r: GetResponse = serde_json::from_str(&string).unwrap();

    Ok(final_r)
}

pub fn post(body: &str) -> Result<PostResponse> {
    let endpoint = format!("{}", BASE_URL);

    let response: String = ureq::post(&endpoint)
        .send_string(body)?
        .into_string()?;

    let final_r: PostResponse = serde_json::from_str(&response).unwrap();

    Ok(final_r)
}

pub fn delete(key: &str, secret: &str) -> Result<DeleteResponse> {
    let endpoint = format!("{}{}/delete", BASE_URL, key);

    let response = ureq::post(&endpoint)
        .set("Secret", secret)
        .call()?;
    let string: String = response.into_string()?;
    let final_r: DeleteResponse = serde_json::from_str(&string).unwrap();

    Ok(final_r)
}