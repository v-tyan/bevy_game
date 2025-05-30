mod core;
mod systems;

use core::resources::EnemySpawnTimer;

use bevy::prelude::*;
use systems::{
    enemy::{move_enemies, spawn_enemy},
    player::{move_player, spawn_player},
};

fn main() {
    App::new()
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, spawn_enemy, move_enemies))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    spawn_player(commands);
}
