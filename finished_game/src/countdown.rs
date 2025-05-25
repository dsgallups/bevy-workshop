use std::time::Duration;

use avian2d::prelude::{Physics, PhysicsTime};
use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Countdown), start_countdown)
        .add_systems(
            Update,
            (tick_countdown, countdown_over, update_ui)
                .chain()
                .run_if(in_state(GameState::Countdown)),
        )
        .add_systems(OnExit(GameState::Countdown), on_exit_countdown);
}

#[derive(Resource)]
pub struct Countdown {
    timer: Timer,
    count: u8,
}

impl Default for Countdown {
    fn default() -> Self {
        let timer = Timer::new(Duration::from_secs(1), TimerMode::Repeating);
        Self { timer, count: 3 }
    }
}
#[derive(Component)]
pub struct CountdownUI;

#[derive(Component)]
pub struct CountdownText;

pub fn start_countdown(mut commands: Commands, mut physics_time: ResMut<Time<Physics>>) {
    commands.init_resource::<Countdown>();
    physics_time.pause();

    commands.spawn((
        CountdownUI,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            CountdownText,
            Text::default(),
            TextFont::from_font_size(90.),
            TextColor(WHITE.into()),
        )],
    ));
}

pub fn tick_countdown(mut countdown: ResMut<Countdown>, time: Res<Time>) {
    countdown.timer.tick(time.delta());
    if countdown.timer.just_finished() {
        countdown.count -= 1;
    }
}

pub fn countdown_over(countdown: Res<Countdown>, mut game_state: ResMut<NextState<GameState>>) {
    if countdown.count == 0 {
        game_state.set(GameState::Playing);
    }
}

pub fn update_ui(countdown: Res<Countdown>, mut ui: Query<&mut Text, With<CountdownText>>) {
    let mut ui = ui.single_mut().unwrap();

    ui.0 = countdown.count.to_string();
}

pub fn on_exit_countdown(
    mut commands: Commands,
    ui: Query<Entity, With<CountdownUI>>,
    mut physics_time: ResMut<Time<Physics>>,
) {
    commands.remove_resource::<Countdown>();
    physics_time.unpause();
    let ui = ui.single().unwrap();

    commands.entity(ui).despawn();
}
