use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};

use super::components::{InputField, InputResource};


pub fn input_field(
    mut time_text_query: Query<&mut Text, With<InputField>>,
    mut char_input_events: EventReader<ReceivedCharacter>,
    mut text: ResMut<InputResource>,
    input: Res<Input<KeyCode>>
) {

    if let Ok(mut txt) = time_text_query.get_single_mut() {

        let mut str = String::new();

        for c in char_input_events.iter() {

            str += &(c.char.to_string());

        }

        if input.pressed(KeyCode::Back) {
            // TODO
        }
        
        text.0 += &str;

        txt.sections[0].value = text.0.clone();

    }

}