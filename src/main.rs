mod client;
mod votes;
mod api;
mod utils;
mod ui;

use bevy::prelude::*;
use ui::{UIPlugin, main_menu::MainMenuPlugin};

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
        .add_plugins(
            (
                UIPlugin,
                MainMenuPlugin
            )
        )
        .run();
}
