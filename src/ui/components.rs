use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct MainMenu;


#[derive(Component)]
pub struct PlayButton;


#[derive(Component)]
pub struct HTPButton;


#[derive(Component)]
pub struct CreditsButton;

#[derive(Component)]
pub struct InputField;


#[derive(Component)]
pub struct QuitButton;

#[derive(Resource)]
pub struct InputResource(pub String);