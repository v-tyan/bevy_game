use crate::core::components::*;

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_COLOR: Color = Color::srgb(0.0, 0.8, 0.0);
const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 30.0);

use bevy::{
    color::Color,
    ecs::{
        query::With,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::Vec2,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerBundle {
            marker: Player,
            speed: Speed(PLAYER_SPEED),
            sprite: Sprite::from_color(PLAYER_COLOR, PLAYER_SIZE),
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
    ));
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in &mut player_transform {
        let mut direction = Vec2::new(0.0, 0.0);

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        transform.translation.x += direction.x * time.delta_secs() * PLAYER_SPEED;
        transform.translation.y += direction.y * time.delta_secs() * PLAYER_SPEED;
    }
}
