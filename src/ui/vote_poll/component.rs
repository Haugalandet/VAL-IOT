use bevy::prelude::*;

#[derive(Component)]
pub struct VotePoll;

#[derive(Component)]
pub struct Choice(pub String);