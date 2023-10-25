use bevy::prelude::*;

#[derive(Component)]
pub struct VotePoll;

#[derive(Component)]
pub struct Choice(pub String);

pub struct HasRefreshed(pub bool);

impl Default for HasRefreshed {
    fn default() -> Self {
        Self(false)
    }
}

pub struct RefreshTimer(pub f32);


impl Default for RefreshTimer {
    fn default() -> Self {
        Self(0.0)
    }
}
#[derive(Component)]
pub struct SendVotes;


#[derive(Component)]
pub struct ResetVotes;