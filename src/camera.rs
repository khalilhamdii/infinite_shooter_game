use bevy::{math::vec3, prelude::*};
use bevy_pancam::{DirectionKeys, PanCam, PanCamPlugin};

use crate::player::Player;
use crate::state::GameState;

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam {
        // grab_buttons: vec![MouseButton::Middle],
        grab_buttons: vec![],
        min_scale: 1.5, // prevent the camera from zooming too far in
        // max_scale: 2.5,
        move_keys: DirectionKeys {
            // the keyboard buttons used to move the camera
            up: vec![KeyCode::ArrowUp, KeyCode::KeyW],
            down: vec![KeyCode::ArrowDown, KeyCode::KeyS],
            left: vec![KeyCode::ArrowLeft, KeyCode::KeyA],
            right: vec![KeyCode::ArrowRight, KeyCode::KeyD],
        },
        ..default()
    });
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;
    let (x, y) = (player_transform.x, player_transform.y);

    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
}
