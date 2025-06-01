mod core;
mod systems;

use core::resources::{EnemySpawnTimer, GameState};

use bevy::prelude::*;
use systems::{
    enemy::{move_enemies, spawn_enemy},
    player::{move_player, player_enemy_collision, spawn_player},
    projectile::{move_projectiles, projectile_enemy_collision, shoot_projectile},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                spawn_enemy,
                move_enemies,
                shoot_projectile,
                move_projectiles,
                projectile_enemy_collision,
                player_enemy_collision,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        0.5,
        TimerMode::Repeating,
    )));
    commands.insert_resource(GameState {
        game_over: false,
        score: 0,
    });
    spawn_player(commands);
}
