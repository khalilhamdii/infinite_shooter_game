use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use rand::Rng;

use crate::*;
use crate::{state::GameState, GlobalTextureAtlas};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), init_world_decorations)
            .add_systems(
                Update,
                (spawn_world_decorations, spawn_world_trees).run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}
#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub enum DecorationType {
    Decoration1,
    Decoration2,
    Decoration3,
    Decoration4,
    Decoration5,
    Decoration6,
}

impl DecorationType {
    fn get_rand_decoration() -> Self {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..6);
        return match rand_index {
            0 => Self::Decoration1,
            1 => Self::Decoration2,
            2 => Self::Decoration3,
            3 => Self::Decoration4,
            4 => Self::Decoration5,
            _ => Self::Decoration6,
        };
    }

    pub fn get_base_sprite_index(&self) -> usize {
        match self {
            DecorationType::Decoration1 => 48,
            DecorationType::Decoration2 => 49,
            DecorationType::Decoration3 => 50,
            DecorationType::Decoration4 => 51,
            DecorationType::Decoration5 => 52,
            DecorationType::Decoration6 => 53,
        }
    }
}

fn init_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        let decoration_type = DecorationType::get_rand_decoration();

        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: decoration_type.get_base_sprite_index(),
            },
            GameEntity,
        ));
    }
}

fn despawn_all_game_entities(
    mut commands: Commands,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    for e in all_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}

#[derive(Component)]
pub struct Decoration;

#[derive(Component)]
pub struct Tree;

fn spawn_world_decorations(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    camera_query: Query<&Transform, With<Camera>>,
    decoration_query: Query<&Transform, With<Decoration>>,
) {
    let mut rng = rand::thread_rng();
    let camera_transform = camera_query.single();

    // Count visible decorations
    let mut visible_decorations_count = 0;
    for decoration_transform in decoration_query.iter() {
        if is_within_camera_view(camera_transform, decoration_transform) {
            visible_decorations_count += 1;
        }
    }

    // println!("visible_decorations_count: {:?}", visible_decorations_count);

    // Spawn additional decorations if needed
    let decorations_needed = NUM_WORLD_DECORATIONS.saturating_sub(visible_decorations_count);
    for _ in 0..decorations_needed {
        let x = rng.gen_range(
            camera_transform.translation.x - WORLD_W..camera_transform.translation.x + WORLD_W,
        );
        let y = rng.gen_range(
            camera_transform.translation.y - WORLD_H..camera_transform.translation.y + WORLD_H,
        );

        let decoration_type = DecorationType::get_rand_decoration();

        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: decoration_type.get_base_sprite_index(),
            },
            Decoration,
        ));
    }
}

enum TreeType {
    Big,
    Small,
}

fn spawn_world_trees(
    mut commands: Commands,
    big_tree_handle: Res<BigTreeTextureAtlas>,
    small_tree_handle: Res<SmallTreeTextureAtlas>,
    camera_query: Query<&Transform, With<Camera>>,
    tree_query: Query<&Transform, With<Tree>>,
) {
    let mut rng = rand::thread_rng();
    let camera_transform = camera_query.single();

    // Count visible decorations
    let mut visible_trees_count = 0;
    let tree_type_index = rng.gen_range(0..=1);
    let tree_type = if tree_type_index == 0 {
        TreeType::Big
    } else {
        TreeType::Small
    };
    for trees_transform in tree_query.iter() {
        if is_within_camera_view(camera_transform, trees_transform) {
            visible_trees_count += 1;
        }
    }

    // Spawn additional trees if needed
    let trees_needed = NUM_WORLD_TREES.saturating_sub(visible_trees_count);
    for _ in 0..trees_needed {
        let x = rng.gen_range(
            camera_transform.translation.x - WORLD_W..camera_transform.translation.x + WORLD_W,
        );
        let y = rng.gen_range(
            camera_transform.translation.y - WORLD_H..camera_transform.translation.y + WORLD_H,
        );

        match tree_type {
            TreeType::Big => {
                commands.spawn((
                    SpriteBundle {
                        texture: big_tree_handle.image.clone().unwrap(),
                        transform: Transform::from_translation(Vec3::new(x, y, 0.0))
                            .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                        ..default()
                    },
                    TextureAtlas {
                        layout: big_tree_handle.layout.clone().unwrap(),
                        index: 0,
                    },
                    Tree,
                    RigidBody::Fixed,
                    Collider::cuboid(24.0, 32.0),
                ));
            }
            TreeType::Small => {
                commands.spawn((
                    SpriteBundle {
                        texture: small_tree_handle.image.clone().unwrap(),
                        transform: Transform::from_translation(Vec3::new(x, y, 0.0))
                            .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                        ..default()
                    },
                    TextureAtlas {
                        layout: small_tree_handle.layout.clone().unwrap(),
                        index: 0,
                    },
                    Tree,
                    RigidBody::Fixed,
                    Collider::cuboid(16.0, 24.0),
                ));
            }
        }
    }
}

fn is_within_camera_view(camera_transform: &Transform, entity_transform: &Transform) -> bool {
    let view_distance = Vec3::new(WORLD_W / 2.0, WORLD_H / 2.0, 0.0);
    let offset = entity_transform.translation - camera_transform.translation;

    offset.x.abs() <= view_distance.x
        && offset.y.abs() <= view_distance.y
        && offset.z.abs() <= view_distance.z
}
