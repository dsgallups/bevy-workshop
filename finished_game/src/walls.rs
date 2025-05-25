use avian2d::prelude::*;
use bevy::ecs::system::Commands;
use bevy::prelude::*;

use crate::CANVAS_SIZE;

const WALL_WIDTH: f32 = 25.;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_walls);
}

fn spawn_walls(mut commands: Commands) {
    let canvas_half_height = CANVAS_SIZE.y / 2.;
    let wall_length = CANVAS_SIZE.x;

    let top = canvas_half_height + WALL_WIDTH;
    let bottom = -(canvas_half_height + WALL_WIDTH);

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(wall_length, WALL_WIDTH)),
            ..default()
        },
        Transform::from_xyz(0., top, 0.0),
        RigidBody::Static,
        Collider::rectangle(wall_length, WALL_WIDTH),
        Restitution::PERFECTLY_ELASTIC,
        // Enable collision events for this entity.
        CollisionEventsEnabled,
        // Read entities colliding with this entity.
        CollidingEntities::default(),
    ));

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(wall_length, WALL_WIDTH)),
            ..default()
        },
        Transform::from_xyz(0., bottom, 0.0),
        RigidBody::Static,
        Collider::rectangle(wall_length, WALL_WIDTH),
        Restitution::PERFECTLY_ELASTIC,
        // Enable collision events for this entity.
        CollisionEventsEnabled,
        // Read entities colliding with this entity.
        CollidingEntities::default(),
    ));
}
