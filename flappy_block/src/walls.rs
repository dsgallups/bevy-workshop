use avian2d::prelude::*;
use bevy::ecs::system::Commands;
use bevy::prelude::*;

use crate::CanvasSize;

pub const WALL_Y_LEN: f32 = 25.;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_walls)
        .add_systems(PreUpdate, update_walls);
}

#[derive(Component)]
struct Wall;
#[derive(Component)]
struct TopWall;

fn spawn_walls(mut commands: Commands, canvas_size: Res<CanvasSize>) {
    // the wall will sit on the top of the canvas
    let wall_x_length = canvas_size.x;

    let top = canvas_size.y / 2. - WALL_Y_LEN / 2.;
    let bottom = -canvas_size.y / 2. + WALL_Y_LEN / 2.;

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
        Wall,
        TopWall,
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
        Wall,
    ));
}

fn update_walls(
    mut walls: Query<(&mut Transform, &mut Sprite, Has<TopWall>), With<Wall>>,
    canvas_size: Res<CanvasSize>,
) {
    let top = canvas_size.y / 2. - WALL_Y_LEN / 2.;
    let bottom = -canvas_size.y / 2. + WALL_Y_LEN / 2.;
    for (mut transform, mut sprite, top_wall) in &mut walls {
        sprite.custom_size.as_mut().unwrap().x = canvas_size.x;
        if top_wall {
            transform.translation.y = top;
        } else {
            transform.translation.y = bottom;
        }
    }
}
