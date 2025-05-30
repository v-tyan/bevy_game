use bevy::{
    ecs::{bundle::Bundle, component::Component},
    sprite::Sprite,
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Speed(pub f32);

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
