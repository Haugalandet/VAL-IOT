use bevy::prelude::*;

use self::{layout::build_vote_poll, systems::vote};

use super::{states::WindowState, components::VoteResource};

mod systems;
mod layout;
mod component;

pub struct VotePollPlugin;

impl Plugin for VotePollPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<VoteResource>()
            .add_systems(OnEnter(WindowState::VotePoll), build_vote_poll)
            .add_systems(Update, debug)
            .add_systems(Update, vote);
    }
}

fn debug(
    input: Res<Input<KeyCode>>,
    poll: Res<VoteResource>
) {
    if input.just_pressed(KeyCode::W) {
        println!("Votes: {:?}", poll.votes.values());
    }
}