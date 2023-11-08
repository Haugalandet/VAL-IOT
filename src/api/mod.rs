

use bevy::prelude::default;
use reqwest::{Client, Response, StatusCode};



use crate::utils::constants::api::apiroot;

pub mod poll;

pub fn create_client() -> Client {
    Client::new()
}

pub async fn connect_to_poll(client: &Client, poll_id: usize) -> Result<Response, reqwest::Error> {
    client
        .execute(
            client
                .post(apiroot(&format!("polls/{}/iot", poll_id)))
                .build()?
        )
        .await
}

pub async fn refresh_connection(client: &Client, header: &str) -> Result<Response, reqwest::Error> {
    client
        .execute(
            client
                .post(apiroot("auth/refresh"))
                .header("Authorization", header)
                .build()?
        )
        .await
}