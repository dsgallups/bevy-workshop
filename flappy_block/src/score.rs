use bevy::prelude::*;

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.init_resource::<Score>()
        .add_systems(Startup, spawn_score_ui)
        .add_systems(OnEnter(GameState::Countdown), |mut score: ResMut<Score>| {
            score.0 = 0;
        })
        .add_systems(Update, update_score_ui.run_if(resource_changed::<Score>));
}

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Component)]
struct ScoreText;
fn spawn_score_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            ScoreText,
            Text::default(),
            TextFont::from_font_size(30.),
            TextColor(Color::WHITE)
        )],
    ));
}

fn update_score_ui(score: Res<Score>, mut text: Query<&mut Text, With<ScoreText>>) {
    let mut text = text.single_mut().unwrap();
    text.0 = score.0.to_string();
}
