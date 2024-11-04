use crate::*;
use bevy::prelude::*;
use enemy::Enemy;
use resources::TargetPosition;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_target_position);
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Movable {
    pub destination: Option<Vec3>,
}

fn update_target_position(
    windows: Query<&Window>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut target_position: ResMut<TargetPosition>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        let window = windows.single();
        let (camera, camera_transform) = camera_q.single();
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            target_position.0 = Some(world_position);
            println!("New target position: {:?}", world_position);
        }
    }
}

// fn my_cursor_system(windows: Query<&Window>, camera_q: Query<(&Camera, &GlobalTransform)>) {
//     let window = windows.single();
//     let (camera, camera_transform) = camera_q.single();

//     if let Some(world_position) = window
//         .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
//     {
//         eprintln!("World coords: {}/{}", world_position.x, world_position.y);
//     }
// }
