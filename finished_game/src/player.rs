use avian2d::prelude::*;
use bevy::{color::palettes::tailwind::RED_500, prelude::*};

use crate::SPACEBAR_VELOCITY;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player)
        .add_systems(Update, on_space);
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    const SQUARE_LEN: f32 = 30.;
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

    //commands.spawn()

    //todo
}

fn on_space(
    button_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut LinearVelocity, With<Player>>,
) {
    if !button_input.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(mut linear_velocity) = player.single_mut() else {
        return;
    };

    linear_velocity.y = SPACEBAR_VELOCITY;
}
