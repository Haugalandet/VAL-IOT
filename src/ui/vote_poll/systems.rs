use bevy::{ui::Interaction, utils::HashMap};
use bevy::prelude::*;

use crate::api::refresh_connection;
use crate::ui::components::HeaderResource;
use crate::{ui::{main_menu::components::ApiClient, components::VoteResource}, utils::constants::api::REFRESH_TIME};

use super::component::{Choice, HasRefreshed, RefreshTimer, ResetVotes};

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

                match map.get(&c.0) {
                    Some(i) => {
                        map.insert(c.0.clone(), i + 1);
                    },
                    None => {
                        map.insert(c.0.clone(), 1);
                    }
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

pub fn reset(
    mut btn_query: Query<(&Interaction, &mut UiImage, &ResetVotes), Changed<Interaction>>,
    asset_server: Res<AssetServer>,
    mut votes: ResMut<VoteResource>
) {

    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();

    if let Ok((interaction, mut image, _)) = btn_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;

                votes.votes = HashMap::new();
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
    mut refresh: Local<HasRefreshed>,
    mut local_time: Local<RefreshTimer>,
    mut header: ResMut<HeaderResource>,
    time: Res<Time>
) {

    if header.0.is_empty() {
        return;
    }

    local_time.0 += time.delta_seconds();


    if local_time.0 < REFRESH_TIME {
        return;
    }

    refresh.0 = true;
        

    let c = client.0.clone();

    if !refresh.0 {
        return;
    }

    if let Ok(rt) = tokio::runtime::Runtime::new() {
        rt.block_on(async {
            let _: Result<(), reqwest::Error> = async {
                let res = refresh_connection(&c, &header.0).await?;

                if res.status().is_success() {
                    let data: String = res.json().await?;
                    header.0 = data;
                }

                refresh.0 = false;

                Ok(())
            }.await;
        });
    }
}