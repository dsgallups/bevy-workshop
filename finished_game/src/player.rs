use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_player);
}

fn spawn_player(mut commands: Commands) {
    //todo
}
