
use bevy::{prelude::*, app::AppExit};


use crate::{utils::constants::ui::INPUT_MAX_COUNT, api::{create_client, poll::{get_poll, Poll}, connect_to_poll}, ui::{states::WindowState, components::{PollResource, HeaderResource}}};

use super::components::{InputField, InputResource, ApiClient, ConnectButton, QuitButton};


pub fn text_input(
    mut time_text_query: Query<&mut Text, With<InputField>>,
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut string: ResMut<InputResource>,
) {
    // Backspace
    if kbd.just_pressed(KeyCode::Back) {
        let mut chars: std::str::Chars<'_> = string.0.chars();
        chars.next_back();
        string.0 = chars.as_str().to_string();
    }
    
    // Control + Backspace
    if kbd.just_pressed(KeyCode::Back) && kbd.pressed(KeyCode::ControlLeft) {
        string.0 = "".to_string();
    }
    
    // Adds all pushed chars to the string
    for ev in evr_char.iter() {

        // ignore control (special) characters
        // Ensures only 9 chars can be added
        if !ev.char.is_control() && string.0.len() < INPUT_MAX_COUNT {
            string.0 += &ev.char.to_string();
        }
    }

    if let Ok(mut txt) = time_text_query.get_single_mut() {
        txt.sections[0].value = string.0.clone();
    }
}


pub fn connect(
    string: Res<InputResource>,
    client: Res<ApiClient>,
    mut btn_query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<ConnectButton>)>,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<WindowState>>,
    mut poll: ResMut<PollResource>,
    mut header: ResMut<HeaderResource>
) {

    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();

    if let Ok((interaction, mut image)) = btn_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                {
                    let c = client.0.clone();

                    let s = string.0.clone();
                    let id = s.parse::<usize>().unwrap_or_default();

                    if let Ok(rt) = tokio::runtime::Runtime::new() {
                        rt.block_on(async {

                            let final_res: Result<bool, reqwest::Error> = async {
                                let res = connect_to_poll(&c, id).await?;
                                if !res.status().is_success() { 
                                    println!("Could not establish connection:\n{:?}\n", res);
                                    return Ok(false);
                                }
                                
                                let head = res.headers().clone();

                                let p: Poll = res.json().await?;

                                poll.poll = Some(p);

                                if let Some(auth) = head.get("Authorization") {

                                    if let Ok(token) = auth.to_str() {
                                        header.0 = token.to_string();
                                    }
                                }


                                Ok(true)
                            }.await;

                            if let Ok(final_res) = final_res {
                                if final_res {
                                    if (&poll.poll).is_some() {
                                        state.set(WindowState::VotePoll);
                                        println!("Found poll: {:?}", &poll.poll);
                                        println!("Switching to poll window");
                                    } else {
                                        println!("Could not get poll");
                                    }
                                }
                            }
                        });
                    }
                }
            }
            Interaction::Hovered => {
                image.texture = asset_server.load("mainmenu/buttonhover.png").into();
            }
            Interaction::None => {
                image.texture = asset_server.load("mainmenu/button.png").into();
            }
        }
    }
}

pub fn quit(
    mut btn_query: Query<(&Interaction, &mut UiImage), (Changed<Interaction>, With<QuitButton>)>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    asset_server: Res<AssetServer>
) {
    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();

    if let Ok((interaction, mut image)) = btn_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                image.texture = asset_server.load("mainmenu/buttonhover.png").into();
            }
            Interaction::None => {
                image.texture = asset_server.load("mainmenu/button.png").into();
            }
        }
    }
}