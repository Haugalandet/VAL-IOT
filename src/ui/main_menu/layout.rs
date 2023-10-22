/// Creates the menues/windows seen in the application.

use bevy::prelude::*;

use super::components::*;


fn create_text_bundle(
    asset_server: &Res<AssetServer>,
    text: &str
) -> TextBundle {
    TextBundle {
        text: Text{
            sections: vec![
                TextSection::new(text, 
                TextStyle {
                    font: asset_server.load("upheavtt.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE.into(),
                })
            ],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

fn create_button_bundle(
    height: Val,
    width: Val,
    asset_server: &Res<AssetServer>
) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            height,
            width,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        image: UiImage { 
            texture: asset_server.load("mainmenu/button.png").into(),
            ..default()
        },
        ..default()
    }
}

/// Builds and spawns the main menu
pub fn build_main_menu(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(
        (NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0), 
                    width: Val::Percent(100.0), 
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent|{
            // == Title ==
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        height: Val::Px(120.0),
                        width: Val::Px(350.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    // == Text ==
                    parent.spawn(create_text_bundle(&asset_server, "Enter Room Code"));
                });
            // == Play ==
            parent.spawn(
                (
                    create_button_bundle(Val::Px(80.0), Val::Px(350.0), &asset_server),
                    InputField,
                )
            ).with_children(|parent|{
                parent.spawn(
                    (
                            create_text_bundle(&asset_server, ""),
                            InputField
                        )
                );
            });
            // == Play ==
            parent.spawn(
                (
                    create_button_bundle(Val::Px(80.0), Val::Px(350.0), &asset_server),
                    ConnectButton,
                )
            ).with_children(|parent|{
                parent.spawn(
                    create_text_bundle(&asset_server, "Connect"),
                );
            });
            parent.spawn(
                (
                    create_button_bundle(Val::Px(80.0), Val::Px(350.0), &asset_server),
                    QuitButton,
                )
            ).with_children(|parent|{
                parent.spawn(
                    create_text_bundle(&asset_server, "Quit"),
                );
            });
        });
}