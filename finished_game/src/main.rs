use avian2d::{math::Vector, prelude::*};
use bevy::{prelude::*, render::camera::ScalingMode};

mod countdown;
mod pipes;
mod player;
mod score;
mod walls;

const GRAVITY_SCALE: f32 = 50.;

const CANVAS_SIZE: Vec2 = Vec2::new(600., 400.);

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    Countdown,
    Playing,
    GameOver,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vector::NEG_Y * 9.81 * GRAVITY_SCALE));

    app.init_state::<GameState>();

    app.add_plugins((
        player::plugin,
        walls::plugin,
        pipes::plugin,
        countdown::plugin,
        score::plugin,
    ))
    .add_systems(Startup, spawn_camera)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: CANVAS_SIZE.x,
                max_height: CANVAS_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
