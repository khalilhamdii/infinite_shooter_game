use bevy::utils::Duration;
use std::f32::consts::PI;

use bevy::math::{vec2, vec3};
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng;

use crate::animation::AnimationTimer;
use crate::controls::{Selectable, TargetDestination};
use crate::player::Player;
use crate::state::GameState;
use crate::world::GameEntity;
use crate::*;

use bevy_rapier2d::prelude::*;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
}

#[derive(Component)]
pub enum EnemyType {
    Green,
    Red,
    Skin,
    White,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemies_movements_based_on_click_position,
                despawn_dead_enemies,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn despawn_dead_enemies(mut commands: Commands, enemy_query: Query<(&Enemy, Entity), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    for (enemy, entity) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_enemies_movements_based_on_click_position(
    mut enemy_query: Query<(&mut Transform, &TargetDestination, &mut Velocity), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }

    for (mut transform, target_destination, mut velocity) in enemy_query.iter_mut() {
        if let Some(destination) = target_destination.0 {
            let dir = (destination.extend(5.0) - transform.translation).normalize();
            velocity.linvel = Vec2::ZERO;
            transform.translation += dir * ENEMY_SPEED;
        }
    }
}

// const AVOIDANCE_RADIUS: f32 = 10.5; // Adjust as needed

// fn update_enemies_movements_with_avoidance(
//     target_position: Res<TargetPosition>,
//     mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
//     obstacle_query: Query<&Transform, (With<RigidBody>, Without<Enemy>)>, // Query obstacles
// ) {
//     if enemy_query.is_empty() {
//         return;
//     }

//     if let Some(target_position) = target_position.0 {
//         for mut enemy_transform in enemy_query.iter_mut() {
//             // Calculate the initial direction toward the target
//             let mut direction =
//                 (target_position.extend(0.0) - enemy_transform.translation).normalize();

//             // Check for nearby obstacles and adjust direction
//             for obstacle_transform in obstacle_query.iter() {
//                 let obstacle_position = obstacle_transform.translation.truncate();
//                 let enemy_position = enemy_transform.translation.truncate();
//                 let distance_to_obstacle = enemy_position.distance(obstacle_position);

//                 if distance_to_obstacle < AVOIDANCE_RADIUS {
//                     // Calculate a repulsive force away from the obstacle
//                     let avoid_direction = (enemy_position - obstacle_position).normalize();
//                     direction +=
//                         avoid_direction.extend(0.0) * (AVOIDANCE_RADIUS - distance_to_obstacle);
//                 }
//             }

//             // Normalize the final direction and move the enemy
//             direction = direction.normalize();
//             enemy_transform.translation += direction * ENEMY_SPEED;
//         }
//     }
// }

// fn update_enemies_movements_based_on_player_pos(
//     player_query: Query<&Transform, With<Player>>,
//     mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
// ) {
//     if player_query.is_empty() || enemy_query.is_empty() {
//         return;
//     }

//     let player_pos = player_query.single().translation;
//     for mut transform in enemy_query.iter_mut() {
//         let dir = (player_pos - transform.translation).normalize();
//         transform.translation += dir * ENEMY_SPEED;
//     }
// }

// primitives
// const RECTANGLE: Rectangle = Rectangle {
//     half_size: Vec2::new(30.0, 30.0),
// };

// const CIRCLE: Circle = Circle { radius: 24.0 };

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(SPAWN_RATE_PER_SECOND);

    if num_enemies >= MAX_NUM_ENEMIES {
        return;
    }

    let player_pos = vec2(0., 0.);
    for _ in 0..enemy_spawn_count {
        let (x, y) = get_random_position_around(player_pos);
        let enemy_type = EnemyType::get_rand_enemy();
        commands
            .spawn((
                SpriteBundle {
                    texture: handle.image.clone().unwrap(),
                    transform: Transform::from_translation(vec3(x, y, 5.0))
                        .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: enemy_type.get_base_sprite_index(),
                },
                Enemy::default(),
                enemy_type,
                AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
                GameEntity,
                RigidBody::Dynamic,
                Collider::ball(8.0),
                LockedAxes::ROTATION_LOCKED,
                GravityScale(0.0),
                ColliderMassProperties::Density(1.0),
                AdditionalMassProperties::Mass(100.0),
                Sleeping::disabled(),
                Selectable,
                TargetDestination(None),
            ))
            .insert(Velocity::zero())
            .insert(ColliderDebugColor(Hsla {
                hue: 0.0,
                saturation: 0.0,
                lightness: 0.0,
                alpha: 0.0,
            }));
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(1000.0..5000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: ENEMY_HEALTH,
        }
    }
}

impl EnemyType {
    fn get_rand_enemy() -> Self {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..4);
        return match rand_index {
            0 => Self::Green,
            1 => Self::Red,
            2 => Self::Skin,
            _ => Self::White,
        };
    }

    pub fn get_base_sprite_index(&self) -> usize {
        match self {
            EnemyType::Green => 16,
            EnemyType::Red => 24,
            EnemyType::Skin => 32,
            EnemyType::White => 40,
        }
    }
}
