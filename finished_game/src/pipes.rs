use std::time::Duration;

use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind::{GREEN_500, GREEN_800},
    prelude::*,
    time::common_conditions::on_timer,
};

use crate::{CANVAS_SIZE, GameState, walls::WALL_Y_LEN};

const PIPE_X_LEN: f32 = 50.;
const PIPE_GAP: f32 = 120.;
const PIPE_SPEED: f32 = 5.;

pub fn plugin(app: &mut App) {
    app.add_observer(insert_new_pipe)
        .add_systems(
            Update,
            spawn_pipes
                .run_if(in_state(GameState::Playing).and(on_timer(Duration::from_millis(1000)))),
        )
        .add_systems(
            FixedUpdate,
            move_pipes_left.run_if(in_state(GameState::Playing)),
        );
}

#[derive(Event)]
pub struct InsertPipe {
    center_y: f32,
}

#[derive(Component)]
pub struct PipePair;

#[derive(Component)]
pub struct Pipe;

fn insert_new_pipe(trigger: Trigger<InsertPipe>, mut commands: Commands) {
    let event = trigger.event();

    let center_y = event.center_y;

    // where on the x axis does the pair sit
    let pair_x = CANVAS_SIZE.x / 2. + PIPE_X_LEN / 2.;

    /*
        a variable with `start` is the y value that is greater than the `end` value.
    */
    let pipe_top_start = CANVAS_SIZE.y / 2. - WALL_Y_LEN;
    let pipe_top_end = center_y + PIPE_GAP / 2.;

    let top_pipe_height = pipe_top_start - pipe_top_end;
    let top_pipe_center_y = center_y + PIPE_GAP / 2. + top_pipe_height / 2.;

    let pipe_bottom_start = pipe_top_end - PIPE_GAP;
    let pipe_bottom_end = -CANVAS_SIZE.y / 2. + WALL_Y_LEN;
    let bottom_pipe_height = pipe_bottom_start - pipe_bottom_end;
    let bottom_pipe_center_y = center_y - PIPE_GAP / 2. - bottom_pipe_height / 2.;

    commands.spawn((
        PipePair,
        Transform::from_xyz(pair_x, 0., 0.),
        InheritedVisibility::VISIBLE,
        RigidBody::Kinematic,
        TransformInterpolation,
        children![
            (
                Pipe,
                Sprite {
                    color: GREEN_500.into(),
                    custom_size: Some(Vec2::new(PIPE_X_LEN, top_pipe_height)),
                    ..default()
                },
                Collider::rectangle(PIPE_X_LEN, top_pipe_height),
                TransformInterpolation,
                Transform::from_xyz(0., top_pipe_center_y, 2.),
            ),
            (
                Pipe,
                Sprite {
                    color: GREEN_800.into(),
                    custom_size: Some(Vec2::new(PIPE_X_LEN, bottom_pipe_height)),
                    ..default()
                },
                Collider::rectangle(PIPE_X_LEN, bottom_pipe_height),
                TransformInterpolation,
                Transform::from_xyz(0., bottom_pipe_center_y, 2.),
            )
        ],
    ));

    // for (i, (color, y)) in [
    //     (BLUE_500.into(), pipe_top_start),
    //     (BLUE_600.into(), pipe_top_end),
    //     (BLUE_700.into(), top_pipe_center_y),
    //     (PURPLE_500.into(), pipe_bottom_start),
    //     (PURPLE_600.into(), pipe_bottom_end),
    //     (PURPLE_700.into(), bottom_pipe_center_y),
    // ]
    // .into_iter()
    // .enumerate()
    // {
    //     let offset_x = match i % 3 {
    //         0 => pair_x - PIPE_X_LEN / 2.,
    //         1 => pair_x,
    //         _ => pair_x + PIPE_X_LEN / 2.,
    //     };
    //     commands.spawn((
    //         Sprite {
    //             color,
    //             custom_size: Some(Vec2::new(10., 10.)),
    //             ..default()
    //         },
    //         Transform::from_xyz(offset_x, y, 3.),
    //     ));
    // }
}

fn spawn_pipes(mut commands: Commands) {
    let min_pipe_length = 20.;

    let min_y = -CANVAS_SIZE.y / 2. + min_pipe_length + PIPE_GAP / 2.;
    let max_y = CANVAS_SIZE.y / 2. - min_pipe_length - PIPE_GAP / 2.;

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
