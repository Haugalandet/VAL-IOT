use bevy::prelude::*;

use self::{systems::setup, components::{PollResource, UserResource}};

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
            .init_resource::<UserResource>()
            .add_systems(Startup, setup);
    }
}