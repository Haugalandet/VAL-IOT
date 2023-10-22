use bevy::{prelude::*, input::{keyboard::KeyboardInput, ButtonState}};

use super::components::{InputField, InputResource};


pub fn text_input(
    mut time_text_query: Query<&mut Text, With<InputField>>,
    mut evr_char: EventReader<ReceivedCharacter>,
    kbd: Res<Input<KeyCode>>,
    mut string: ResMut<InputResource>,
) {
    if kbd.just_pressed(KeyCode::Back) {
        let mut chars: std::str::Chars<'_> = string.0.chars();
        chars.next_back();
        string.0 = chars.as_str().to_string();
    }
    if kbd.just_pressed(KeyCode::Back) && kbd.pressed(KeyCode::ControlLeft) {
        string.0 = "".to_string();
    }
    for ev in evr_char.iter() {
        // ignore control (special) characters
        if !ev.char.is_control() {
            string.0 += &ev.char.to_string();
        }
    }

    if let Ok(mut txt) = time_text_query.get_single_mut() {
        txt.sections[0].value = string.0.clone();
    }
}
