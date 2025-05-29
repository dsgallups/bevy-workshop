use basket::Basket;
use bevy::prelude::*;
use bevy_ball::BevyBall;

mod basket;
mod bevy_ball;

const CANVAS_HEIGHT: f32 = 500.;
const CANVAS_WIDTH: f32 = 800.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((bevy_ball::plugin, basket::plugin))
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(Update, (catch_bevy_ball, update_count))
        .run();
}

#[derive(Resource, Default)]
pub struct Score(u32);

#[derive(Component)]
pub struct ScoreUi;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((Node { ..default() }, ScoreUi, Text::new("0")));

    //outer rectangle
    const BORDER: f32 = 20.;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(
            CANVAS_WIDTH + BORDER,
            CANVAS_HEIGHT + BORDER,
        ))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(0., 0., -2.),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(CANVAS_WIDTH, CANVAS_HEIGHT))),
        MeshMaterial2d(materials.add(Color::BLACK)),
        Transform::from_xyz(0., 0., -1.),
    ));
}

fn catch_bevy_ball(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bevy_balls: Query<(Entity, &Transform), With<BevyBall>>,
    basket: Single<(&Sprite, &Transform), With<Basket>>,
) {
    let (basket_sprite, basket_location) = basket.into_inner();
    let basket_x = basket_location.translation.x;
    let basket_size = basket_sprite.custom_size.unwrap();

    for (bevy_ball, ball_location) in &bevy_balls {
        // the ball location is too high to be caught
        if ball_location.translation.y > basket_location.translation.y + basket_size.y / 2. {
            continue;
        }
        let ball_x = ball_location.translation.x;

        if ball_x > (basket_x - basket_size.x / 2.) && ball_x < (basket_x + basket_size.x / 2.) {
            commands.entity(bevy_ball).despawn();
            score.0 += 1;
        }
    }
}

fn update_count(score: Res<Score>, mut score_text: Single<&mut Text, With<ScoreUi>>) {
    score_text.0 = score.0.to_string();
}
