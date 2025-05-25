use avian2d::prelude::*;
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_walls);
}

fn spawn_walls(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    let window = windows.single().unwrap();

    let size = window.size();
    let half = size.y / 2.;
    let width = size.x;

    let top = half;
    let bottom = -half;

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(width, 50.0)),
            ..default()
        },
        Transform::from_xyz(0., top, 0.0),
        RigidBody::Static,
        Collider::rectangle(width, 50.0),
        Restitution::PERFECTLY_ELASTIC,
        // Enable collision events for this entity.
        CollisionEventsEnabled,
        // Read entities colliding with this entity.
        CollidingEntities::default(),
    ));

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(width, 50.0)),
            ..default()
        },
        Transform::from_xyz(0., bottom, 0.0),
        RigidBody::Static,
        Collider::rectangle(width, 50.0),
        Restitution::PERFECTLY_ELASTIC,
        // Enable collision events for this entity.
        CollisionEventsEnabled,
        // Read entities colliding with this entity.
        CollidingEntities::default(),
    ));
}
