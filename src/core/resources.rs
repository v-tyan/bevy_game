use bevy::{ecs::resource::Resource, time::Timer};

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource)]
pub struct GameState {
    pub game_over: bool,
    pub score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            game_over: false,
            score: 0,
        }
    }
}
