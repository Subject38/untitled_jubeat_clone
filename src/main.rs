use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .add_system(mouse_input_system)
        .add_system(update_square_visuals)
        .run();
}

// Component to identify our squares
#[derive(Component)]
struct Square;

#[derive(Component, Default)]
struct MouseActive(bool);

#[derive(Component, Default)]
struct KeyboardActive(bool);

// Component to store the key associated with each square
#[derive(Component)]
struct KeyMapping(KeyCode);

fn setup(mut commands: Commands) {
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
            .insert(Square)
            .insert(KeyMapping(key))
            .insert(MouseActive(false))
            .insert(KeyboardActive(false));
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&KeyMapping, &mut KeyboardActive)>,
) {
    query.iter_mut().for_each(|(key_mapping, mut state)| {
        state.0 = keyboard_input.pressed(key_mapping.0);
    });
}

fn mouse_input_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(&Transform, &Sprite, &mut MouseActive, Entity)>,
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
        let p = cursor_pos - window_size / 2.0;
        camera_transform.compute_matrix() * p.extend(0.0).extend(1.0)
    } else {
        // If the cursor is not within the window, we set it to an out-of-range position
        Vec4::new(f32::MAX, f32::MAX, 0.0, 0.0)
    };

    for (transform, sprite, mut mouse_active, _entity) in query.iter_mut() {
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

fn update_square_visuals(mut query: Query<((&MouseActive, &KeyboardActive), &mut Sprite)>) {
    for ((mouse_active, keyboard_active), mut sprite) in query.iter_mut() {
        if mouse_active.0 || keyboard_active.0 {
            sprite.color = Color::rgb(1.0, 0.0, 0.0); // Active color
        } else {
            sprite.color = Color::rgb(0.5, 0.5, 0.5); // Inactive color
        }
    }
}
