pub mod ui {
    use bevy::prelude::*;
    
    pub fn create_text_bundle(
        asset_server: &Res<AssetServer>,
        text: &str,
        font_size: f32
    ) -> TextBundle {
        TextBundle {
            text: Text{
                sections: vec![
                    TextSection::new(text, 
                    TextStyle {
                        font: asset_server.load("upheavtt.ttf"),
                        font_size,
                        color: Color::WHITE.into(),
                    })
                ],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        }
    }

    pub fn create_button_bundle(
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

}