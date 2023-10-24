use bevy::prelude::*;

use crate::api::poll::Poll;

use self::{layout::build_main_menu, systems::{text_input, connect, quit}, components::{InputResource, ApiClient}};

use super::{states::WindowState, components::PollResource};

pub mod components;
mod layout;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(InputResource("".to_string()))
            .insert_resource(ApiClient::default())
            .add_state::<WindowState>()
            .add_systems(OnEnter(WindowState::JoinPoll), build_main_menu)
            .add_systems(Update, connect.run_if(in_state(WindowState::JoinPoll))) // Function that runs on button press
            .add_systems(Update, quit.run_if(in_state(WindowState::JoinPoll))) // Function that runs on button press
            .add_systems(Update, debug) // Function that runs on button press
            .add_systems(Update, text_input.run_if(in_state(WindowState::JoinPoll)));
    }
}


fn debug(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<NextState<WindowState>>,
    mut poll: ResMut<PollResource>
) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(WindowState::VotePoll);
        poll.poll = Some(Poll::default());
    }
}
