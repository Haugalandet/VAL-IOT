use bevy::{prelude::{Component, Resource}, tasks::Task, ecs::query::WorldQuery};
use reqwest::{Response, Client};

use crate::api::create_client;

#[derive(Component)]
pub struct MainMenu;


#[derive(Component)]
pub struct ConnectButton;



#[derive(Component)]
pub struct InputField;


#[derive(Component)]
pub struct QuitButton;

#[derive(Resource)]
pub struct InputResource(pub String);


#[derive(Resource)]
pub struct ApiClient(pub Client);

impl Default for ApiClient {
    fn default() -> Self {
        Self(create_client())
    }
}