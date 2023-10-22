use bevy::prelude::*;

use self::{layout::build_main_menu, systems::{text_input, connect, quit}, components::{InputResource, ApiClient}};

mod components;
mod layout;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InputResource("".to_string()))
            .insert_resource(ApiClient::default())
            .add_systems(Startup, build_main_menu)
            .add_systems(Update, connect) // Function that runs on button press
            .add_systems(Update, quit) // Function that runs on button press
            .add_systems(Update, text_input);
    }
}
