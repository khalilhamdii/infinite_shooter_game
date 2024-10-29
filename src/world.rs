use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::animation::AnimationTimer;
use crate::gun::{Gun, GunTimer};
use crate::player::{Health, Player, PlayerState};
use crate::*;
use crate::{state::GameState, GlobalTextureAtlas};

pub struct WorldPlugin;

#[derive(Component)]
pub struct GameEntity;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, init_world_decorations),
        )
        .add_systems(
            Update,
            spawn_world_decorations.run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(GameState::InGame), despawn_all_game_entities);
    }
}

fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR).with_z(10.)),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 0,
        },
        Player,
        Health(PLAYER_HEALTH),
        PlayerState::default(),
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        GameEntity,
        RigidBody::Dynamic,
        Collider::ball(12.0),
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        ColliderMassProperties::Density(1.0),
        AdditionalMassProperties::Mass(100.0),
    ));

    commands.spawn((
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 17,
        },
        Gun,
        GunTimer(Stopwatch::new()),
        GameEntity,
    ));

    next_state.set(GameState::InGame);
}

fn init_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(24..=25),
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
struct Decoration;

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

    println!("visible_decorations_count: {:?}", visible_decorations_count);

    // Spawn additional decorations if needed
    let decorations_needed = NUM_WORLD_DECORATIONS.saturating_sub(visible_decorations_count);
    for _ in 0..decorations_needed {
        let x = rng.gen_range(
            camera_transform.translation.x - WORLD_W..camera_transform.translation.x + WORLD_W,
        );
        let y = rng.gen_range(
            camera_transform.translation.y - WORLD_H..camera_transform.translation.y + WORLD_H,
        );
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: rng.gen_range(24..=25),
            },
            Decoration,
        ));
    }
}

fn is_within_camera_view(camera_transform: &Transform, entity_transform: &Transform) -> bool {
    let view_distance = Vec3::new(WORLD_W / 2.0, WORLD_H / 2.0, 0.0);
    let offset = entity_transform.translation - camera_transform.translation;

    offset.x.abs() <= view_distance.x
        && offset.y.abs() <= view_distance.y
        && offset.z.abs() <= view_distance.z
}
