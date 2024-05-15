use crate::camera::CameraData;
use bevy::prelude::*;

pub struct BaseUiPlugin;

impl Plugin for BaseUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_ui.run_if(resource_exists::<CameraData>));
    }
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_data: Res<CameraData>,
    mut has_run: Local<bool>,
) {
    if !*has_run {
        let font_handle = asset_server.load("Alexandria.ttf");

        commands
            .spawn((
                TargetCamera(camera_data.camera_id.unwrap()),
                NodeBundle {
                    background_color: Color::NONE.into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(5.5),
                        left: Val::Percent(9.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|ui| {
                ui.spawn(TextBundle::from_section(
                    "RATSSSS",
                    TextStyle {
                        font: font_handle,
                        font_size: 30.0,
                        color: Color::YELLOW,
                    },
                ));
            });
        *has_run = true;
    }
}
