use bevy::{ui::Interaction, utils::HashMap};
use bevy::prelude::*;

use crate::api::poll::Choice;
use crate::api::{refresh_connection, self};
use crate::ui::components::{HeaderResource, PollResource};
use crate::{ui::{main_menu::components::ApiClient, components::VoteResource}, utils::constants::api::REFRESH_TIME};

use super::component::{ChoiceComponent, HasRefreshed, RefreshTimer, ResetVotes};

pub fn send_votes(
    mut btn_query: Query<(&Interaction, &mut UiImage, &ChoiceComponent), (Changed<Interaction>)>,
    asset_server: Res<AssetServer>,
    votes: Res<VoteResource>,
    client: Res<ApiClient>,
    header: Res<HeaderResource>,
    poll: Res<PollResource>
) {
    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();

    if let Ok((interaction, mut image, c)) = btn_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;

                if let Ok(rt) = tokio::runtime::Runtime::new() {
                    rt.block_on(async {
                        //send_votes(client: &Client, poll_id: usize, header: &str, votes: Vec<Choice>)

                        if let Some(p) = &poll.poll {
                            let res = api::send_votes(
                                &client.0,
                                p.pollId.unwrap_or_default(),
                                &header.0,
                                votes.votes.values().into_iter().map(|c| { return c.to_owned() }).collect::<Vec<Choice>>()
                            )
                            .await;

                            match res {
                                Ok(r) => println!("Tried sending votes got {} as respone", r.status()),
                                Err(e) => println!("Failed sending votes got {}", e),
                            }
                        } else {
                            panic!("NO POLL IN SEND_VOTES");
                        }                
                    });
                }

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

pub fn vote(
    mut btn_query: Query<(&Interaction, &mut UiImage, &ChoiceComponent), Changed<Interaction>>,
    asset_server: Res<AssetServer>,
    mut votes: ResMut<VoteResource>,
    poll: Res<PollResource>
) {

    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();

    if let Ok((interaction, mut image, c)) = btn_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;

                let mut map = votes.votes.clone();

                let posi_poll = poll.poll.clone();
                let some_poll = posi_poll.unwrap();

                let choice_poll = some_poll.clone();
                let temp = some_poll.clone();

                let choices = choice_poll.choices.iter().filter(|ch| { ch.title == c.0.title.clone() }).collect::<Vec<&Choice>>();

                if choices.len() == 0 {
                    panic!("Could not find choice by title: {:?}, in this poll: {:?}", c.0.title, temp);
                }

                let mut choice = choices[0].to_owned();
                choice.count = Some(1);

                match map.get(&c.0.title) {
                    Some(i) => {
                        let mut cho = i.clone();
                        if let Some(count) = i.count {
                            cho.count = Some(count + 1);
                        } else {
                            cho.count = Some(1);
                        }

                        map.insert(c.0.title.clone(), cho.clone());
                    },
                    None => {
                        map.insert(c.0.title.clone(), choice.to_owned());
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