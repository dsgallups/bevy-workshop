use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_pipes);
}

#[derive(Component)]
pub struct PipePair;

fn spawn_pipes(mut commands: Commands) {
    //commands.spawn((PipePair))
}
