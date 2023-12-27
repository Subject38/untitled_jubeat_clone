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

// TODO: If this ever breaks, rewrite this entire method
// since it was written entirely with AI and I don't
// actually understand what's going on here...
pub fn mouse_input_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(&Transform, &Sprite, &mut MouseActive)>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&GlobalTransform, With<Camera2d>>,
) {
    let Ok(window) = primary_query.get_single() else {
        return;
    };
    let cursor_pos = window.cursor_position();

    let camera_transform = camera_query.single();

    // Convert screen position to world position
    let world_position = if let Some(cursor_pos) = cursor_pos {
        let window_size = Vec2::new(window.width(), window.height());
        let cursor_pos_flipped = Vec2::new(cursor_pos.x, window.height() - cursor_pos.y);
        let p = cursor_pos_flipped - window_size / 2.0;
        camera_transform.compute_matrix() * p.extend(0.0).extend(1.0)
    } else {
        // If the cursor is not within the window, we set it to an out-of-range position
        Vec4::new(f32::MAX, f32::MAX, 0.0, 0.0)
    };

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

// TODO: Add HID controller support. Will require rebindable keys of course.
// Also implement P4IO handling. This can thankfully be much more hardcoded.