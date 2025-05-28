use bevy::prelude::*;

use crate::CANVAS_HEIGHT;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, move_basket);
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
        Transform::from_xyz(0., -CANVAS_HEIGHT / 2., 0.),
    ));
}

fn move_basket(
    mut basket: Single<&mut Transform, With<Basket>>,
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // in units / sec
    let move_speed = 120.;
    if button_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        basket.translation.x -= move_speed * time.delta_secs();
    }
    if button_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        basket.translation.x += move_speed * time.delta_secs();
    }
}
