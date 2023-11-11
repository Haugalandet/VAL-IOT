mod api;
mod utils;
mod ui;

use bevy::prelude::*;
use ui::{UIPlugin, main_menu::MainMenuPlugin, vote_poll::VotePollPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "VAL-IOT".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }) // Window stuff
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(
            (
                UIPlugin,
                MainMenuPlugin,
                VotePollPlugin
            )
        )
        .run();
}
