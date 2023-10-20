use reqwest::{Client, Response};

use crate::utils::constants::api::APIROOT;


pub fn create_client() -> Client {
    Client::new()
}


pub async fn establish_connection(client: &Client, room_code: &str) -> Result<Response, reqwest::Error> {
    let req = client
        .post(format!("{}/auth/{}", APIROOT, room_code))
        .header("room_code", room_code)
        .build()?;
    
    client.execute(req).await
}