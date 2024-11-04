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
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut target_position: ResMut<TargetPosition>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(cursor_position) = cursor_position.0 {
            target_position.0 = Some(cursor_position);
            println!("New target position: {:?}", cursor_position);
        }
    }
}
