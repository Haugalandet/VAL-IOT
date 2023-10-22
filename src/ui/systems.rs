use bevy::prelude::*;

pub fn setup(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle::default());
}