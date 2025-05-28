use bevy::prelude::*;

use crate::CANVAS_HEIGHT;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

#[derive(Component)]
pub struct Basket;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("bucket.png"),
            custom_size: Some(Vec2::splat(70.)),
            ..default()
        },
        Basket,
        Transform::from_xyz(0., -CANVAS_HEIGHT / 2., 2.),
    ));
}
