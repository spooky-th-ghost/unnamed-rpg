use crate::input::{InputBuffer, PlayerAction};
use crate::player::PlayerData;
use crate::GameState;

use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};

const RES_WIDTH: u32 = 854;
const RES_HEIGHT: u32 = 480;

const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(1);

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Overworld), spawn_camera)
            .insert_resource(CameraData::default())
            .insert_resource(Msaa::Off)
            .add_plugins(TraditionalCameraPlugin)
            .register_type::<MainCamera>();
    }
}

struct TraditionalCameraPlugin;

impl Plugin for TraditionalCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_camera_desired_position,
                position_camera.after(crate::physics::systems::lateral_movement),
                rotate_camera,
                adjust_offset,
                fit_canvas,
            )
                .run_if(in_state(GameState::Overworld)),
        );
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MainCamera {
    offset: Vec3,
    y_offset_max: f32,
    y_offset_min: f32,
    angle: f32,
    easing: f32,
    camera_mode: CameraMode,
    desired_position: Vec3,
}

#[derive(Resource, Default)]
pub struct CameraData {
    pub camera_position: Vec3,
    pub camera_rotation: Quat,
}

impl CameraData {
    pub fn translate_direction_in_camera_space(&self, x: f32, z: f32) -> Vec3 {
        let camera_transform =
            Transform::from_translation(self.camera_position).with_rotation(self.camera_rotation);

        let mut forward = *camera_transform.forward();
        forward.y = 0.0;

        let mut right = *camera_transform.right();
        right.y = 0.0;

        let right_vec: Vec3 = x * right;
        let forward_vec: Vec3 = z * forward;

        right_vec + forward_vec
    }
}

#[derive(Default, Reflect)]
pub enum CameraMode {
    #[default]
    Fixed,
    Free,
    Follow,
}

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct InGameCamera;

#[derive(Component)]
struct OuterCamera;

impl CameraMode {
    fn shift_up(&self) -> CameraMode {
        match self {
            CameraMode::Fixed => CameraMode::Free,
            CameraMode::Free => CameraMode::Free,
            CameraMode::Follow => CameraMode::Free,
        }
    }
    fn shift_down(&self) -> CameraMode {
        match self {
            CameraMode::Fixed => CameraMode::Follow,
            CameraMode::Free => CameraMode::Follow,
            CameraMode::Follow => CameraMode::Follow,
        }
    }
}
fn spawn_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window>,
) {
    let canvas_size = Extent3d {
        width: RES_WIDTH,
        height: RES_HEIGHT,
        ..default()
    };

    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    canvas.resize(canvas_size);

    let image_handle = images.add(canvas);

    // The Camera that renders our pixelated view to the canvas
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        MainCamera {
            offset: Vec3::new(0.0, 6.5, 10.0),
            y_offset_max: 9.5,
            y_offset_min: 4.5,
            angle: 0.0,
            easing: 4.0,
            camera_mode: CameraMode::Free,
            desired_position: Vec3::ZERO,
        },
        InGameCamera,
    ));

    // Canvas that the main camera is rendered to
    commands.spawn((
        SpriteBundle {
            texture: image_handle,
            ..default()
        },
        Canvas,
        HIGH_RES_LAYER,
    ));

    let window = windows.single();

    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                near: -1000.0,
                scale: calculate_outter_camera_scale(
                    window.physical_width() as f32,
                    window.physical_height() as f32,
                ),
                ..default()
            },
            ..default()
        },
        OuterCamera,
        HIGH_RES_LAYER,
    ));
}

fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut projections: Query<&mut OrthographicProjection, With<OuterCamera>>,
) {
    for event in resize_events.read() {
        let mut projection = projections.single_mut();
        projection.scale = calculate_outter_camera_scale(event.width, event.height);
    }
}

fn calculate_outter_camera_scale(width: f32, height: f32) -> f32 {
    let h_scale = width / RES_WIDTH as f32;
    let v_scale = height / RES_HEIGHT as f32;
    0.8 / h_scale.min(v_scale).round()
}

fn update_camera_desired_position(
    mut camera_query: Query<&mut MainCamera>,
    player_data: Res<PlayerData>,
) {
    for mut camera in &mut camera_query {
        let mut starting_transform = Transform::from_translation(player_data.player_position);

        starting_transform.rotation = Quat::default();
        starting_transform.rotate_y(camera.angle.to_radians());
        let dir = starting_transform.forward().normalize();
        camera.desired_position =
            starting_transform.translation + (dir * camera.offset.z) + (Vec3::Y * camera.offset.y);
    }
}

fn adjust_offset(player_data: Res<PlayerData>, mut camera_query: Query<&mut MainCamera>) {
    for mut camera in &mut camera_query {
        let speed_percentage =
            (player_data.player_current_speed / player_data.player_max_speed) * 2.0;
        camera.offset.y = 2.5
            + camera
                .y_offset_max
                .lerp(camera.y_offset_min, speed_percentage);
    }
}

fn position_camera(
    time: Res<Time>,
    player_data: Res<PlayerData>,
    mut camera_data: ResMut<CameraData>,
    mut camera_query: Query<(&mut Transform, &MainCamera)>,
) {
    for (mut transform, camera) in &mut camera_query {
        camera_data.camera_position = transform.translation;
        camera_data.camera_rotation = transform.rotation;
        match camera.camera_mode {
            CameraMode::Fixed | CameraMode::Free => {
                let lerped_position = transform.translation.lerp(
                    camera.desired_position,
                    time.delta_seconds() * camera.easing,
                );
                transform.translation = lerped_position;
                transform.look_at(player_data.player_position, Vec3::Y);
            }
            _ => (),
        }
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut camera_query: Query<&mut MainCamera>,
    actions_query: Query<&InputBuffer>,
) {
    for mut camera in &mut camera_query {
        for action in &actions_query {
            if action.just_pressed(PlayerAction::CamModeChangePositive) {
                camera.camera_mode = camera.camera_mode.shift_up();
            }
            if action.just_pressed(PlayerAction::CamModeChangeNegative) {
                camera.camera_mode = camera.camera_mode.shift_down();
            }
            match camera.camera_mode {
                CameraMode::Fixed => {
                    if action.just_pressed(PlayerAction::CamRotateLeft) {
                        camera.angle -= 45.0;
                    }
                    if action.just_pressed(PlayerAction::CamRotateRight) {
                        camera.angle += 45.0;
                    }

                    let angle_i16 = camera.angle as i16;
                    let angle_difference = angle_i16 % 45;
                    let angle_change = if angle_difference <= 22 {
                        -1 * angle_difference
                    } else {
                        45 - angle_difference
                    };
                    let new_angle = (angle_i16 + angle_change) as f32;
                    camera.angle = new_angle;
                }
                CameraMode::Free => {
                    if action.pressed(PlayerAction::CamRotateLeft) {
                        camera.angle -= 180.0 * time.delta_seconds();
                    }
                    if action.pressed(PlayerAction::CamRotateRight) {
                        camera.angle += 180.0 * time.delta_seconds();
                    }
                }
                _ => (),
            }

            if camera.angle > 360.0 {
                camera.angle -= 360.0;
            }

            if camera.angle < -360.0 {
                camera.angle += 360.0;
            }
        }
    }
}
