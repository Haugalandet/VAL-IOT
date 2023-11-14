use reqwest::{Client, Response, Body};



use crate::utils::constants::api::apiroot;

use self::poll::Choice;

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

pub async fn send_votes(client: &Client, poll_id: usize, header: &str, votes: Vec<Choice>) -> Result<Response, reqwest::Error> {

    todo!();

    client
        .execute(
            client
                .post(apiroot(&format!("iot/polls/{}/votes", poll_id)))
                .body("")
                .header("Authorization", header)
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