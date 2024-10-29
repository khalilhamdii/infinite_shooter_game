use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::state::GameState;
use crate::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component, Default)]
pub enum PlayerState {
    #[default]
    Idle,
    Run,
}

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEnemyCollisionEvent>().add_systems(
            Update,
            (
                handle_player_death,
                handle_player_input,
                handle_player_enemy_collision_events,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_enemy_collision_events(
    mut player_query: Query<&mut Health, With<Player>>,
    mut events: EventReader<PlayerEnemyCollisionEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut health = player_query.single_mut();
    for _ in events.read() {
        health.0 -= ENEMY_DAMAGE;
    }
}

fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player_query.is_empty() {
        return;
    }
    let health = player_query.single();
    if health.0 <= 0.0 {
        next_state.set(GameState::MainMenu);
    }
}

// fn handle_player_input(
//     mut player_query: Query<(&mut Velocity, &mut PlayerState), With<Player>>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// ) {
//     if player_query.is_empty() {
//         return;
//     }

//     let (mut rb_vels, mut player_state) = player_query.single_mut();
//     let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
//     let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
//     let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
//     let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

//     let x_axis = -(left as i8) + right as i8;
//     let y_axis = -(down as i8) + up as i8;

//     let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
//     if move_delta != Vec2::ZERO {
//         move_delta /= move_delta.length();
//     }

//     // Update the velocity on the rigid_body_component,
//     // the bevy_rapier plugin will update the Sprite transform.
//     rb_vels.linvel += move_delta * PLAYER_SPEED;

//     if move_delta.is_finite() && (up || left || down || right) {
//         *player_state = PlayerState::Run;
//     } else {
//         *player_state = PlayerState::Idle;
//     }
// }

fn handle_player_input(
    mut player_query: Query<(&mut Transform, &mut PlayerState, &mut Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut transform, mut player_state, mut velocity) = player_query.single_mut();
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);

    let mut delta = Vec2::ZERO;
    if w_key {
        delta.y += 1.0;
    }
    if s_key {
        delta.y -= 1.0;
    }
    if a_key {
        delta.x -= 1.0;
    }
    if d_key {
        delta.x += 1.0;
    }
    delta = delta.normalize();

    velocity.linvel = Vec2::ZERO; // Set linear velocity to zero

    if delta.is_finite() && (w_key || a_key || s_key || d_key) {
        transform.translation += vec3(delta.x, delta.y, 0.0) * PLAYER_SPEED;
        transform.translation.z = 10.0;
        *player_state = PlayerState::Run;
    } else {
        *player_state = PlayerState::Idle;
    }
}
