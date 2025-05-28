use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::CANVAS_HEIGHT;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_bevy_ball.run_if(on_timer(Duration::from_secs(2))),
                update_gravity,
            ),
        )
        .add_systems(PostUpdate, despawn);
}

#[derive(Component)]
pub struct BevyBall;

#[derive(Resource)]
pub struct BevyIcon(Handle<Image>);

#[derive(Component, Default)]
pub struct Velocity(Vec2);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BevyIcon(asset_server.load("icon.png")));
}

fn spawn_bevy_ball(mut commands: Commands, icon: Res<BevyIcon>) {
    commands.spawn((
        Sprite {
            image: icon.0.clone(),
            custom_size: Some(Vec2::splat(50.)),
            ..default()
        },
        BevyBall,
        Velocity::default(),
        Transform::from_xyz(0., CANVAS_HEIGHT / 2., 0.),
    ));
}

fn update_gravity(
    mut bevy_balls: Query<(&mut Transform, &mut Velocity), With<BevyBall>>,
    time: Res<Time>,
) {
    const GRAVITY_ACCEL: f32 = -9.81;

    for (mut transform, mut velocity) in &mut bevy_balls {
        velocity.0.y += GRAVITY_ACCEL * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

fn despawn(mut commands: Commands, bevy_balls: Query<(Entity, &Transform), With<BevyBall>>) {
    for (entity, transform) in &bevy_balls {
        if transform.translation.y < -CANVAS_HEIGHT / 2. {
            commands.entity(entity).despawn();
        }
    }
}
