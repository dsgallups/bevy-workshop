use bevy::{
    color::palettes::{css::RED, tailwind::RED_600},
    prelude::*,
};

use crate::{GameState, score::Score};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::GameOver), spawn_gameover_ui)
        .add_systems(OnExit(GameState::GameOver), remove_gameover_ui);
}

#[derive(Component)]
pub struct GameOverUI;

pub fn spawn_gameover_ui(mut commands: Commands, final_score: Res<Score>) {
    let ui = commands
        .spawn((
            GameOverUI,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(40.),
                ..default()
            },
            BackgroundColor(RED.with_alpha(0.2).into()),
            ZIndex(2),
        ))
        .id();

    commands.spawn((
        Text::new(format!("Final Score: {}", final_score.0)),
        TextColor(Color::WHITE),
        TextFont::from_font_size(90.),
        ChildOf(ui),
    ));

    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(300.),
                height: Val::Px(80.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
            BackgroundColor(RED_600.into()),
            ChildOf(ui),
            children![(
                Text::new("Restart"),
                TextFont::from_font_size(40.),
                TextColor(Color::WHITE),
            )],
        ))
        .observe(restart);

    //todo
}

fn restart(_trigger: Trigger<Pointer<Click>>, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Countdown);
}

pub fn remove_gameover_ui(mut commands: Commands, ui: Query<Entity, With<GameOverUI>>) {
    let ui = ui.single().unwrap();
    commands.entity(ui).despawn();
}
