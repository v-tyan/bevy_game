mod core;
mod systems;

use core::resources::{EnemySpawnTimer, GameState};

use bevy::prelude::*;
use systems::{enemy::*, player::*, projectile::*, ui::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Menu), menu_system)
        .add_systems(Update, start_game_button.run_if(in_state(AppState::Menu)))
        .add_systems(OnEnter(AppState::InGame), (setup, setup_ui))
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
                display_score,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(EnemySpawnTimer(Timer::from_seconds(
        0.5,
        TimerMode::Repeating,
    )));
    commands.insert_resource(GameState {
        game_over: false,
        score: 0,
    });
    spawn_player(commands, asset_server);
}
