use bevy::prelude::*;

use self::{systems::setup, states::WindowState, components::PollResource};

pub mod main_menu;
pub mod vote_poll;
mod states;
mod systems;
mod components;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PollResource>()
            .add_systems(Startup, setup);
    }
}