use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component, Default)]
pub struct MouseActive(pub bool);

#[derive(Component, Default)]
pub struct KeyboardActive(pub bool);

// Component to store the key associated with each square
#[derive(Component)]
pub struct KeyMapping(pub KeyCode);

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&KeyMapping, &mut KeyboardActive)>,
) {
    query.iter_mut().for_each(|(key_mapping, mut state)| {
        state.0 = keyboard_input.pressed(key_mapping.0);
    });
}

// Code shamelessly stolen from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
pub fn mouse_input_system(
    mouse_button_input: Res<Input<MouseButton>>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut query: Query<(&Transform, &Sprite, &mut MouseActive)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for (transform, sprite, mut mouse_active) in query.iter_mut() {
            let size = sprite.custom_size.unwrap_or(Vec2::new(100.0, 100.0));
            let min = transform.translation - Vec3::new(size.x / 2.0, size.y / 2.0, 0.0);
            let max = transform.translation + Vec3::new(size.x / 2.0, size.y / 2.0, 0.0);

            mouse_active.0 = world_position.x >= min.x
                && world_position.x <= max.x
                && world_position.y >= min.y
                && world_position.y <= max.y
                && mouse_button_input.pressed(MouseButton::Left);
        }
    }
}

// TODO: Add HID controller support. Will require rebindable keys of course.
// Also implement P4IO handling. This can thankfully be much more hardcoded.
