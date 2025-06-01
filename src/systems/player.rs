use crate::core::{components::*, resources::GameState};

const PLAYER_SPEED: f32 = 300.0;
const PLAYER_COLOR: Color = Color::srgb(0.0, 0.8, 0.0);
const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 30.0);

use bevy::{
    color::Color,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    log::info,
    math::{
        Vec2,
        bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    },
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

pub fn player_enemy_collision(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_state: ResMut<GameState>,
) {
    if let Ok((player_entity, player_transform)) = player_query.single() {
        for enemy_transform in enemy_query {
            let enemy_bounds = Aabb2d::new(
                enemy_transform.translation.truncate(),
                enemy_transform.scale.truncate() / 2.,
            );

            let player_bounds =
                BoundingCircle::new(player_transform.translation.truncate(), PLAYER_SIZE.x / 2.);

            if player_bounds.intersects(&enemy_bounds) {
                info!("player_enemy_collision");
                game_state.game_over = true;
            }
        }
    }
}
