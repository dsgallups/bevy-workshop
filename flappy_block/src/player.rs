use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind::RED_500, input::common_conditions::input_just_pressed, prelude::*,
};

use crate::GameState;

const SQUARE_LEN: f32 = 30.;
const SPACEBAR_VELOCITY: f32 = 300.;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Countdown), spawn_player)
        .add_systems(OnExit(GameState::GameOver), despawn_player)
        .add_systems(
            Update,
            jump.run_if(in_state(GameState::Playing).and(input_just_pressed(KeyCode::Space))),
        );
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    let square_sprite = Sprite {
        color: RED_500.into(),
        custom_size: Some(Vec2::splat(SQUARE_LEN)),
        ..default()
    };

    commands.spawn((
        Player,
        square_sprite,
        RigidBody::Dynamic,
        LockedAxes::ALL_LOCKED.unlock_translation_y(),
        Collider::rectangle(SQUARE_LEN, SQUARE_LEN),
    ));
}

fn despawn_player(mut commands: Commands, player: Single<Entity, With<Player>>) {
    commands.entity(*player).despawn();
}

fn jump(mut linear_velocity: Single<&mut LinearVelocity, With<Player>>) {
    linear_velocity.y = SPACEBAR_VELOCITY;
}
