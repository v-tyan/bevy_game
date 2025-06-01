use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec2,
    sprite::Sprite,
};

// markers
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Direction(pub Vec2);

// bundles
#[derive(Bundle)]
pub struct PlayerBundle {
    pub(crate) marker: Player,
    pub(crate) speed: Speed,
    pub(crate) sprite: Sprite,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub(crate) marker: Enemy,
    pub(crate) speed: Speed,
    pub(crate) sprite: Sprite,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub(crate) marker: Projectile,
    pub(crate) speed: Speed,
    pub(crate) direction: Direction,
    pub(crate) sprite: Sprite,
}
