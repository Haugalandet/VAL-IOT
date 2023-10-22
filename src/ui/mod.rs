use bevy::prelude::*;

use self::systems::setup;

pub mod main_menu;
mod states;
mod systems;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}