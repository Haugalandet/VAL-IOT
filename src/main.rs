mod client;
mod votes;
mod api;
mod utils;
mod ui;

use bevy::prelude::*;
use ui::{layout::build_main_menu, systems::{text_input}, components::InputResource};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "What is a Monad".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }) // Window stuff
            .set(ImagePlugin::default_nearest())
        )
        .insert_resource(InputResource("".to_string()))
        .add_systems(Startup, build_main_menu)
        .add_systems(Update, text_input)
        .run();
}
