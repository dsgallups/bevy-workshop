use std::time::Duration;

use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind::{GREEN_500, GREEN_800},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::{CanvasSize, GameState, player::Player, score::Score, walls::WALL_Y_LEN};

const PIPE_X_LEN: f32 = 50.;
const PIPE_GAP: f32 = 180.;
const PIPE_SPEED: f32 = 5.;

pub fn plugin(app: &mut App) {
    app.add_observer(insert_new_pipe)
        .add_systems(
            Update,
            (
                spawn_pipes.run_if(
                    in_state(GameState::Playing).and(on_timer(Duration::from_millis(1000))),
                ),
                check_if_passed,
            ),
        )
        .add_systems(
            FixedUpdate,
            move_pipes_left.run_if(in_state(GameState::Playing)),
        )
        //.add_systems(PreUpdate, update_pipes)
        .add_systems(PostUpdate, despawn_passed_pipes)
        .add_systems(OnExit(GameState::GameOver), despawn_all_pipes);
}

#[derive(Event)]
pub struct InsertPipe {
    center_y: f32,
}

#[derive(Component)]
pub struct PipePair;

#[derive(Component)]
pub struct Pipe;

fn spawn_pipes(mut commands: Commands, canvas_size: Res<CanvasSize>) {
    let min_pipe_length = 20.;

    let pipe_top_start = pipe_top_start(canvas_size.y);
    let pipe_bottom_end = pipe_bottom_end(canvas_size.y);

    let min_y = pipe_bottom_end + min_pipe_length + PIPE_GAP / 2.;
    let max_y = pipe_top_start - min_pipe_length - PIPE_GAP / 2.;

    let random_center = rand::random_range((min_y..max_y));

    commands.trigger(InsertPipe {
        center_y: random_center,
    });
}

fn move_pipes_left(mut pairs: Query<&mut Transform, With<PipePair>>) {
    for mut pair in &mut pairs {
        pair.translation.x -= PIPE_SPEED;
    }
}

fn despawn_passed_pipes(
    mut commands: Commands,
    pipe_pairs: Query<(Entity, &Transform), With<PipePair>>,
    canvas_size: Res<CanvasSize>,
) {
    for (pipe_pair, transform) in pipe_pairs {
        if transform.translation.x <= (-canvas_size.x / 2. + PIPE_X_LEN) {
            info!(
                "Canvas size: {}, {}",
                canvas_size.x,
                canvas_size.x / 2. - PIPE_X_LEN
            );
            commands.entity(pipe_pair).despawn();
        }
    }
}

fn insert_new_pipe(
    trigger: Trigger<InsertPipe>,
    mut commands: Commands,
    canvas_size: Res<CanvasSize>,
) {
    let event = trigger.event();

    let center_y = event.center_y;

    // where on the x axis does the pair sit
    let pair_x = canvas_size.x / 2. + PIPE_X_LEN / 2.;

    /*
        a variable with `start` is the y value that is greater than the `end` value.
    */
    let pipe_top_end = center_y + PIPE_GAP / 2.;

    let top_pipe_height = pipe_top_start(canvas_size.y) - pipe_top_end;
    let top_pipe_center_y = center_y + PIPE_GAP / 2. + top_pipe_height / 2.;

    let pipe_bottom_start = pipe_top_end - PIPE_GAP;
    let bottom_pipe_height = pipe_bottom_start - pipe_bottom_end(canvas_size.y);
    let bottom_pipe_center_y = center_y - PIPE_GAP / 2. - bottom_pipe_height / 2.;

    let pair = commands
        .spawn((
            PipePair,
            Transform::from_xyz(pair_x, 0., 0.),
            InheritedVisibility::VISIBLE,
            RigidBody::Kinematic,
            TransformInterpolation,
        ))
        .id();

    commands
        .spawn((
            Pipe,
            Sprite {
                color: GREEN_500.into(),
                custom_size: Some(Vec2::new(PIPE_X_LEN, top_pipe_height)),
                ..default()
            },
            Collider::rectangle(PIPE_X_LEN, top_pipe_height),
            CollisionEventsEnabled,
            TransformInterpolation,
            Transform::from_xyz(0., top_pipe_center_y, 2.),
            ChildOf(pair),
        ))
        .observe(on_collision);
    commands
        .spawn((
            Pipe,
            Sprite {
                color: GREEN_800.into(),
                custom_size: Some(Vec2::new(PIPE_X_LEN, bottom_pipe_height)),
                ..default()
            },
            Collider::rectangle(PIPE_X_LEN, bottom_pipe_height),
            CollisionEventsEnabled,
            TransformInterpolation,
            Transform::from_xyz(0., bottom_pipe_center_y, 2.),
            ChildOf(pair),
        ))
        .observe(on_collision);
}

fn on_collision(
    trigger: Trigger<OnCollisionStart>,
    player: Query<&Player>,
    mut state: ResMut<NextState<GameState>>,
) {
    let event = trigger.event();

    if player.contains(event.collider) {
        state.set(GameState::GameOver);
    }
}

/// Marks a pair of pipes that have passed the player and added to the score
#[derive(Component)]
struct Scored;

fn check_if_passed(
    mut commands: Commands,
    pipes: Query<(Entity, &Transform), (With<PipePair>, Without<Scored>)>,
    mut score: ResMut<Score>,
) {
    for (pipe, transform) in &pipes {
        // the player is always at 0.
        if transform.translation.x >= 0. {
            continue;
        }
        score.0 += 1;
        commands.entity(pipe).insert(Scored);
    }
}

fn despawn_all_pipes(mut commands: Commands, pipes: Query<Entity, With<PipePair>>) {
    for pipe in pipes {
        commands.entity(pipe).despawn();
    }
}

pub fn pipe_top_start(canvas_height: f32) -> f32 {
    canvas_height / 2. - WALL_Y_LEN
}

pub fn pipe_bottom_end(canvas_height: f32) -> f32 {
    -canvas_height / 2. + WALL_Y_LEN
}
