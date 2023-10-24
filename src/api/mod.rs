

use std::time::Duration;

use reqwest::{Client, Response, StatusCode};
use tokio::time::sleep;


use crate::utils::constants::api::apiroot;

use self::user::User;
mod user;
pub mod poll;

pub fn create_client() -> Client {
    Client::new()
}


pub async fn establish_connection(client: &Client, room_code: String) -> Result<Response, reqwest::Error> {
    client.execute(
        client
            .post(apiroot("users"))
            .json(&User {
                username: room_code.clone(),
                password: room_code,
            })
            .build()?
    ).await
}


pub async fn confirm_connection(client: Client, user: &User) -> Result<Response, reqwest::Error> {
    client
        .execute(
            client
                .post(apiroot("auth/login"))
                .json(user)
                .build()?
        )
        .await
}

pub async fn refresh_connection(client: &Client, user: &User) -> Result<Response, reqwest::Error> {
    client
        .execute(
            client
                .post(apiroot("auth/refresh"))
                .json(user)
                .build()?
        )
        .await
}

pub async fn disconnect(client: &Client, user: &User) -> Result<Response, reqwest::Error> {
    client
        .execute(
            client
                .post(apiroot("auth/logout"))
                .json(user)
                .build()?
        )
        .await
}