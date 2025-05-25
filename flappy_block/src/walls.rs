use avian2d::prelude::*;
use bevy::ecs::system::Commands;
use bevy::prelude::*;

use crate::CANVAS_SIZE;

pub const WALL_Y_LEN: f32 = 25.;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_walls);
}

fn spawn_walls(mut commands: Commands) {
    // the wall will sit on the top of the canvas
    let wall_x_length = CANVAS_SIZE.x;

    let top = CANVAS_SIZE.y / 2. - WALL_Y_LEN / 2.;
    let bottom = -CANVAS_SIZE.y / 2. + WALL_Y_LEN / 2.;

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(wall_x_length, WALL_Y_LEN)),
            ..default()
        },
        Transform::from_xyz(0., top, 0.0),
        RigidBody::Static,
        Collider::rectangle(wall_x_length, WALL_Y_LEN),
        Restitution::PERFECTLY_ELASTIC,
        // Enable collision events for this entity.
        CollisionEventsEnabled,
        // Read entities colliding with this entity.
        CollidingEntities::default(),
    ));

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(wall_x_length, WALL_Y_LEN)),
            ..default()
        },
        Transform::from_xyz(0., bottom, 0.0),
        RigidBody::Static,
        Collider::rectangle(wall_x_length, WALL_Y_LEN),
        Restitution::PERFECTLY_ELASTIC,
        // Enable collision events for this entity.
        CollisionEventsEnabled,
        // Read entities colliding with this entity.
        CollidingEntities::default(),
    ));
}
