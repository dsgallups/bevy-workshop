use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;

mod pipes;
mod player;
mod walls;

const GRAVITY_SCALE: f32 = 50.;

const CANVAS_SIZE: Vec2 = Vec2::new(600., 400.);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vector::NEG_Y * 9.81 * GRAVITY_SCALE));

    app.add_plugins((player::plugin, walls::plugin, pipes::plugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
