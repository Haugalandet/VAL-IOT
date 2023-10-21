use bevy::prelude::States;

/// States used to identify what stage the application is at.

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum WindowState {
    #[default]
    JoinPoll,
    VotePoll,
    LostConnection,
    Error
}