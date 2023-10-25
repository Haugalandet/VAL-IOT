use bevy::ui::Interaction;
use bevy::prelude::*;

use crate::ui::{main_menu::components::{InputResource, ApiClient}, states::WindowState, components::{PollResource, VoteResource, UserResource}};

use super::component::Choice;

pub fn vote(
    mut btn_query: Query<(&Interaction, &mut UiImage, &Choice), (Changed<Interaction>)>,
    asset_server: Res<AssetServer>,
    mut votes: ResMut<VoteResource>
) {

    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();

    if let Ok((interaction, mut image, c)) = btn_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                let mut map = votes.votes.clone();

                if let Some(r) = map.get(&c.0.clone()) {
                    map.insert(c.0.clone(), r + 1);
                }

                votes.votes = map;
            }
            Interaction::Hovered => {
                image.texture = asset_server.load("mainmenu/buttonhover.png").into();
            }
            Interaction::None => {
                image.texture = normal_button;
            }
        }
    }
}


pub fn refresh_poll_connection(
    client: Res<ApiClient>,
    mut user: ResMut<UserResource>
) {

}