use crate::input::{KeyMapping, KeyboardActive, MouseActive};
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    // Set up a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Define the layout and size of the squares
    let square_size = Vec2::new(100.0, 100.0);
    let spacing = 120.0;
    let start_x = -1.5 * spacing;
    let start_y = 1.5 * spacing;

    // Define the mapping from keys to positions
    let keys = [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Q,
        KeyCode::W,
        KeyCode::E,
        KeyCode::R,
        KeyCode::A,
        KeyCode::S,
        KeyCode::D,
        KeyCode::F,
        KeyCode::Z,
        KeyCode::X,
        KeyCode::C,
        KeyCode::V,
    ];

    for (i, &key) in keys.iter().enumerate() {
        let x = start_x + (i % 4) as f32 * spacing;
        let y = start_y - (i / 4) as f32 * spacing;

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 0.5),
                    custom_size: Some(square_size),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            })
            .insert(KeyMapping(key))
            .insert(MouseActive(false))
            .insert(KeyboardActive(false));
    }
}
