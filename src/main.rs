use bevy::prelude::*;
use bevy_gltf_blueprints::*;
use bevy_xpbd_3d::prelude::*;

mod animation;
mod assets;
mod camera;
mod environment;
mod input;
mod item;
mod physics;
mod player;
mod types;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        bevy_xpbd_3d::prelude::PhysicsPlugins::default(),
    ))
    .add_plugins((
        input::InputPlugin,
        camera::CameraPlugin,
        player::PlayerPlugin,
        physics::PhysicsPlugin,
        assets::AssetPlugin,
        animation::AnimationPlugin,
        environment::EnvironmentPlugin,
        item::ItemPlugin,
    ))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 50.0,
    })
    .register_type::<animation::Animated>()
    .insert_state(GameState::Preload)
    .add_systems(Startup, setup)
    .add_systems(OnEnter(GameState::Overworld), post_load_spawn);
    #[cfg(feature = "debug-render")]
    {
        println!("Debug Renderer eneabled");
        app.add_plugins(bevy_xpbd_3d::prelude::PhysicsDebugPlugin::default());
    }
    #[cfg(feature = "inspector")]
    {
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        println!("Inpsector Enabled");
        app.add_plugins(WorldInspectorPlugin::default());
    }
    app.run();
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
            mesh: meshes.add(bevy::prelude::Cuboid::new(50.0, 0.5, 50.0).mesh()),
            material: materials.add(Color::PURPLE),
            transform: Transform::from_xyz(0.0, -0.25, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 0.5, 50.0),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(bevy::prelude::Sphere::new(0.5).mesh()),
            material: materials.add(Color::ORANGE),
            transform: Transform::from_xyz(5.0, 0.5, 5.0),
            ..default()
        },
        RigidBody::Static,
        Collider::sphere(0.5),
        item::OverworldItem {
            id: item::ItemId::Milkshake,
        },
        Sensor,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::ORANGE,
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });
}

fn post_load_spawn(mut commands: Commands) {
    for i in 1..8 {
        let x = i as f32;

        commands.spawn((
            BlueprintName("Streetlight".to_string()),
            TransformBundle {
                local: Transform::from_xyz(x, 0., -3.0),
                ..default()
            },
            SpawnHere,
        ));
    }

    commands.spawn((
        BlueprintName("Dumpster".to_string()),
        TransformBundle {
            local: Transform::from_xyz(0.0, 0., 10.0),
            ..default()
        },
        SpawnHere,
    ));
}
