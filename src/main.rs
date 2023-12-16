mod input;
mod setup;

use bevy::prelude::*;
use input::{keyboard_input_system, mouse_input_system, KeyboardActive, MouseActive};
use setup::setup;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, mouse_input_system)
        .add_systems(Update, update_square_visuals)
        .run();
}

fn update_square_visuals(mut query: Query<((&MouseActive, &KeyboardActive), &mut Sprite)>) {
    for ((mouse_active, keyboard_active), mut sprite) in query.iter_mut() {
        if mouse_active.0 || keyboard_active.0 {
            sprite.color = Color::rgb(1.0, 0.0, 0.0); // Active color
        } else {
            sprite.color = Color::rgb(0.5, 0.5, 0.5); // Inactive color
        }
    }
}
