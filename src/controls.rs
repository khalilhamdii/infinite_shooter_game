use crate::*;
use bevy::prelude::*;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (mouse_input_system, debug_selected_entities).chain(),
        );
    }
}

use bevy::prelude::*;
use enemy::Enemy;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Movable {
    pub destination: Option<Vec3>,
}

fn my_cursor_system(windows: Query<&Window>, camera_q: Query<(&Camera, &GlobalTransform)>) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}

fn mouse_input_system(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut q_selectables: Query<(Entity, &Transform), With<Selectable>>,
    mut q_selected: Query<&mut Movable, With<Selected>>,
) {
    let window = windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        // Screen to World Conversion
        let (camera, camera_transform) = q_camera.single();
        let world_position = screen_to_world(cursor_position, window, camera_transform, camera);

        // Left Click - Select units
        if buttons.just_pressed(MouseButton::Left) {
            // Clear previous selections
            for entity in q_selectables.iter().map(|(entity, _)| entity) {
                commands.entity(entity).remove::<Selected>();
            }

            // Select units within range
            for (entity, transform) in &mut q_selectables {
                if within_selection_range(transform.translation, world_position) {
                    commands.entity(entity).insert(Selected);
                }
            }
        }

        // // Right Click - Move selected units
        // if buttons.just_pressed(MouseButton::Right) {
        //     for mut movable in &mut q_selected {
        //         movable.destination = Some(world_position);
        //     }
        // }
    }
}

fn within_selection_range(unit_position: Vec3, cursor_position: Vec3) -> bool {
    let selection_radius = 50.0; // Adjust this as needed
    unit_position.distance(cursor_position) <= selection_radius
}

fn screen_to_world(
    cursor_position: Vec2,
    window: &Window,
    camera_transform: &GlobalTransform,
    camera: &Camera,
) -> Vec3 {
    let screen_position = cursor_position - Vec2::new(window.width() / 2.0, window.height() / 2.0);
    let ndc =
        (screen_position / Vec2::new(window.width() / 2.0, window.height() / 2.0)).extend(-1.0);

    let world_position =
        camera_transform.compute_matrix() * camera.clip_from_view().inverse() * ndc.extend(1.0);

    // Convert Vec4 to Vec3 by dividing by w and taking the x, y, and z components
    world_position.truncate() / world_position.w
}

fn debug_selected_entities(selected_query: Query<&Selected, With<Enemy>>) {
    let selected_count = selected_query.iter().count();
    println!("Number of selected entities: {}", selected_count);
}
// fn handle_movement_command(
//     mouse_button_input: Res<Input<MouseButton>>,
//     windows: Res<Windows>,
//     mut selection_state: ResMut<SelectionState>,
// ) {
//     if mouse_button_input.just_pressed(MouseButton::Right) {
//         let window = windows.get_primary().unwrap();
//         if let Some(cursor_pos) = window.cursor_position() {
//             // Set target destination for all selected units
//             let destination = screen_to_world(cursor_pos);
//             selection_state.destination = Some(destination);
//         }
//     }
// }

// fn move_selected_units(
//     mut query: Query<(&mut Transform, &Movable), With<Selected>>,
//     time: Res<Time>,
//     selection_state: Res<SelectionState>,
// ) {
//     if let Some(destination) = selection_state.destination {
//         for (mut transform, _movable) in query.iter_mut() {
//             let direction = (destination - transform.translation.truncate()).normalize();
//             transform.translation += direction.extend(0.0) * time.delta_seconds() * MOVEMENT_SPEED;

//             // Check if close to the destination to stop
//             if transform.translation.truncate().distance(destination) < 5.0 {
//                 selection_state.destination = None;
//             }
//         }
//     }
// }
