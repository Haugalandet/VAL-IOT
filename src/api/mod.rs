use reqwest::{Client, Response};

use crate::utils::constants::api::{APIROOT, apiroot};


pub fn create_client() -> Client {
    Client::new()
}


pub async fn establish_connection(client: Client, room_code: String) -> Result<Response, reqwest::Error> {
    client.execute(
        client
            .post(apiroot("auth/test"))
            //.header("room_code", room_code.clone())
            .body(room_code)
            .build()?
    ).await
}