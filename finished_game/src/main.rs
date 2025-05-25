use avian2d::{math::Vector, prelude::*};
use bevy::prelude::*;

mod player;
mod walls;

const GRAVITY_SCALE: f32 = 50.;
const SPACEBAR_VELOCITY: f32 = 300.;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vector::NEG_Y * 9.81 * GRAVITY_SCALE));

    app.add_plugins((player::plugin, walls::plugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
