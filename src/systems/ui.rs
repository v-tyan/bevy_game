use bevy::{
    color::palettes::css::WHITE,
    core_pipeline::core_2d::Camera2d,
    ecs::{
        component::Component,
        entity::Entity,
        query::{self, With},
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    log::info,
    state::state::{NextState, States},
    text::{JustifyText, TextColor, TextFont, TextLayout},
    time::Time,
    ui::{AlignItems, JustifyContent, Node, TextShadow, Val, widget::Text},
};

use crate::core::{
    components::{Enemy, Projectile},
    resources::GameState,
};

#[derive(Component)]
pub struct ScoreText;
#[derive(Component)]
pub struct MenuText;

pub fn update_score(mut game_state: ResMut<GameState>, time: Res<Time>) {
    let score = game_state.score;
    let state = game_state.game_over;
    info!("#{state}#{score}");
    if !game_state.game_over {
        info!("#{state}#{score}");
        game_state.score += 1;
    }
}

pub fn display_score(game_state: Res<GameState>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        text.clear();
        text.push_str(format!("Score: {}", game_state.score).as_str());
    }
}

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 40.0,
            ..Default::default()
        },
        TextColor(WHITE.into()),
        ScoreText,
    ));
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    GameOver,
}

pub fn menu_system(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("Press ENTER"),
        TextFont {
            font_size: 60.0,
            ..Default::default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        TextColor(WHITE.into()),
        MenuText,
    ));
}

pub fn start_game_button(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<NextState<AppState>>,
    query: Query<Entity, With<MenuText>>,
) {
    if let Ok(menu_text) = query.single() {
        if keyboard_input.just_pressed(KeyCode::Enter) {
            app_state.set(AppState::InGame);
            commands.entity(menu_text).despawn();
        }
    }
}

pub fn game_over(mut commands: Commands, query: Query<Entity, With<Enemy>>) {
    for entity in query {
        commands.entity(entity).despawn();
    }
    commands.spawn((
        Text::new("GAME OVER\nPress ENTER"),
        TextFont {
            font_size: 60.0,
            ..Default::default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        TextColor(WHITE.into()),
        MenuText,
    ));
}
