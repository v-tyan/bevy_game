use bevy::{
    color::Color,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    input::{ButtonInput, mouse::MouseButton},
    math::{
        Vec2,
        bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    },
    render::camera::Camera,
    sprite::Sprite,
    time::Time,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

use crate::core::components::{Direction, Enemy, Player, Projectile, ProjectileBundle, Speed};

const PROJECTILE_SPEED: f32 = 1500.0;
const PROJECTILE_COLOR: Color = Color::srgb(0.0, 0.0, 0.5);
const PROJECTILE_SIZE: Vec2 = Vec2::new(10.0, 20.0);

pub fn shoot_projectile(
    mut commands: Commands,
    player_query: Query<&mut Transform, With<Player>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(player_transform) = player_query.single() {
            let window = window_query.single();
            let (camera, global_transform) = camera_query.single().unwrap();

            if let Some(cursor_in_world_position) = window
                .unwrap()
                .cursor_position()
                .and_then(|cursor| Some(camera.viewport_to_world(global_transform, cursor)))
                .map(|ray| ray.unwrap().origin.truncate())
            {
                let direction = (cursor_in_world_position
                    - player_transform.translation.truncate())
                .normalize();

                commands.spawn((
                    ProjectileBundle {
                        marker: Projectile,
                        speed: Speed(PROJECTILE_SPEED),
                        sprite: Sprite::from_color(PROJECTILE_COLOR, PROJECTILE_SIZE),
                        direction: Direction(direction),
                    },
                    Transform::from_translation(player_transform.translation),
                ));
            }
        }
    }
}

pub fn move_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform, &Speed, &Direction), With<Projectile>>,
    time: Res<Time>,
) {
    for (entity, mut transform, speed, direction) in projectiles.iter_mut() {
        let velocity = speed.0 * direction.0 * time.delta_secs();
        transform.translation += velocity.extend(0.0);

        if transform.translation.length() > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_enemy_collision(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (projectile_entity, projectile_transform) in projectile_query {
        for (enemy_entity, enemy_transform) in enemy_query {
            let projectile_bounds = BoundingCircle::new(
                projectile_transform.translation.truncate(),
                PROJECTILE_SIZE.x,
            );
            let enemy_bounds = Aabb2d::new(
                enemy_transform.translation.truncate(),
                enemy_transform.scale.truncate() / 2.,
            );
            if projectile_bounds.intersects(&enemy_bounds) {
                commands.entity(projectile_entity).despawn();
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
