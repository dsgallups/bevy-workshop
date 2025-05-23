use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, player::plugin))
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
