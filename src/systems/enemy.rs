use bevy::{
    color::Color,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::Vec2,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};

use crate::core::{
    components::{Enemy, EnemyBundle, Speed},
    resources::EnemySpawnTimer,
};

const ENEMY_SPEED: f32 = 150.0;
const ENEMY_COLOR: Color = Color::srgb(0.8, 0.0, 0.0);
const ENEMY_SIZE: Vec2 = Vec2::new(15.0, 15.0);

pub fn spawn_enemy(mut commands: Commands, time: Res<Time>, mut timer: ResMut<EnemySpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            EnemyBundle {
                marker: Enemy,
                speed: Speed(ENEMY_SPEED),
                sprite: Sprite::from_color(ENEMY_COLOR, ENEMY_SIZE),
            },
            Transform::from_xyz((rand::random::<f32>() - 0.5) * 800.0, 1000.0, 0.0),
        ));
    }
}

pub fn move_enemies(mut enemy_transform: Query<&mut Transform, With<Enemy>>, time: Res<Time>) {
    for mut transform in &mut enemy_transform {
        transform.translation.y -= time.delta_secs() * ENEMY_SPEED;
    }
}
