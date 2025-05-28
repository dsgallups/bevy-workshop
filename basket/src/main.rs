use bevy::prelude::*;

mod basket;
mod bevy_ball;

const CANVAS_HEIGHT: f32 = 500.;
const CANVAS_WIDTH: f32 = 800.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((bevy_ball::plugin, basket::plugin))
        .init_resource::<Count>()
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Default)]
pub struct Count(u32);

#[derive(Component)]
pub struct CountUi;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((Node { ..default() }, CountUi, Text::new("0")));

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
