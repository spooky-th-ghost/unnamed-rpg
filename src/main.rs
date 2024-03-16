use bevy::{
    math::bounding::{BoundingSphere, IntersectsVolume},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

mod assets;
mod camera;
mod input;
mod physics;
mod player;
mod types;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .add_plugins((
            input::InputPlugin,
            camera::CameraPlugin,
            player::PlayerPlugin,
            physics::systems::PhysicsPlugin,
            assets::AssetPlugin,
        ))
        .insert_state(GameState::Preload)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_mover, detect_red_sphere))
        .run();
}

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Preload,
    Load,
    Overworld,
    MainMenu,
}

#[derive(Component)]
pub struct Mover;
#[derive(Component)]
pub struct SphereBoy;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.0).mesh().build()),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        },
        SphereBoy,
    ));

    commands.spawn((TransformBundle::default(), Mover));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(bevy::prelude::Cuboid::new(50.0, 0.5, 50.0).mesh()),
            material: materials.add(Color::PURPLE),
            transform: Transform::from_xyz(0.0, -2.0, 0.0),
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(50.0, 0.5, 50.0),
    ));
}

fn move_mover(time: Res<Time>, mut mover_query: Query<&mut Transform, With<Mover>>) {
    for mut transform in &mut mover_query {
        transform.translation.x = time.elapsed_seconds().sin() * 4.0;
    }
}

fn detect_red_sphere(
    mut gizmos: Gizmos,
    mover_query: Query<&Transform, With<Mover>>,
    sphere_query: Query<&Transform, With<SphereBoy>>,
) {
    for mover_transform in &mover_query {
        for sphere_transform in &sphere_query {
            let mover_bounding = BoundingSphere::new(mover_transform.translation, 2.0);
            let sphere_bounding = BoundingSphere::new(sphere_transform.translation, 1.0);
            let gizmo_color = if mover_bounding.intersects(&sphere_bounding) {
                Color::YELLOW
            } else {
                Color::BLUE
            };
            gizmos.sphere(
                mover_transform.translation,
                Quat::IDENTITY,
                2.0,
                gizmo_color,
            );
        }
    }
}
