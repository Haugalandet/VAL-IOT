use bevy::{prelude::*, utils::HashMap};

use crate::api::poll::Poll;

#[derive(Resource)]
pub struct PollResource {
    pub poll: Option<Poll>,
}

impl Default for PollResource {
    fn default() -> Self {
        Self { poll: None }
    }
}

#[derive(Resource)]
pub struct VoteResource {
    pub votes: HashMap<String, usize>
}

impl Default for VoteResource {
    fn default() -> Self {
        Self { votes: HashMap::new() }
    }
}


#[derive(Resource, Default)]
pub struct HeaderResource(pub String);