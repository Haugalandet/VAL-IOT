
use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}, tasks::ComputeTaskPool, app::AppExit};
use tokio::runtime::Runtime;

use crate::{utils::constants::ui::INPUT_MAX_COUNT, api::{establish_connection, create_client, poll::get_poll}, ui::{states::WindowState, components::PollResource}};

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
    mut poll: ResMut<PollResource>
) {

    let normal_button : Handle<Image> = asset_server.load("mainmenu/button.png").into();

    if let Ok((interaction, mut image)) = btn_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                image.texture = normal_button;
                {
                    let c = client.0.clone();

                    let s = string.0.clone();

                    if let Ok(rt) = tokio::runtime::Runtime::new() {
                        rt.block_on(async {
                            match establish_connection(&c, s).await {
                                Ok(res) => if res.status().is_success() {
                                    println!("Established connection.");

                                    if let Ok(p) = get_poll(&c, 0).await {
                                        state.set(WindowState::VotePoll);
                                        poll.poll = Some(p);
                                    } else {
                                        println!("Could not get poll");
                                    }

                                }
                                Err(res) => println!("Error: {:?}", res),
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