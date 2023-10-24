use bevy::prelude::*;

use crate::{ui::{main_menu::components::MainMenu, components::PollResource}, utils::funcs::ui::{create_text_bundle, create_button_bundle}};

use super::component::{VotePoll, Choice};

pub fn build_vote_poll(
    mut cmd: Commands,
    main_menu: Query<Entity, With<MainMenu>>,
    asset_server: Res<AssetServer>,
    mut poll: ResMut<PollResource>
) {
    if let Some(p) = poll.poll.clone() {
        if let Ok(e) = main_menu.get_single() {
            cmd.entity(e).despawn_recursive();
        }
    
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
                VotePoll,
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
                        // == Title Text ==
                        parent.spawn(create_text_bundle(&asset_server, &p.title));
                    });
                // == Description ==
                parent.spawn(
                    (
                        create_button_bundle(Val::Px(350.0), Val::Px(550.0), &asset_server),
                    )
                ).with_children(|parent| {
                    parent.spawn(
                        create_text_bundle(&asset_server, &p.description)
                    );
                });
            });

            cmd.spawn(
                (
                    NodeBundle {
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
                    VotePoll,
                )
            )
            .with_children(|parent| {
                for c in p.choices {
                    parent.spawn(
                        (
                            create_button_bundle(Val::Px(80.0), Val::Px(350.0), &asset_server),
                            Choice(c.clone())
                        )
                        
                    ).with_children(|parent| {
                        parent.spawn(
                            create_text_bundle(&asset_server, &c)
                        );
                    });
                }
            });
    }
}