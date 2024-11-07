use crate::*;
use bevy::color::palettes::css::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use std::collections::HashSet;

use bevy::math::{vec2, vec3};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_target_destination,
                handle_collision_events,
                mouse_input_handler,
                draw_box_selection,
                mouse_motion_handler,
            ),
        );
    }
}

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct TargetDestination(pub Option<Vec2>);

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct SelectionCircle;

fn update_target_destination(
    cursor_position: Res<CursorPosition>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut selected_entities: Query<&mut TargetDestination, (With<Selectable>, With<Selected>)>,
) {
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(cursor_position) = cursor_position.0 {
            for mut target_destination in selected_entities.iter_mut() {
                target_destination.0 = Some(cursor_position);
                println!("New target position: {:?}", cursor_position);
            }
        }
    }
}

fn mouse_input_handler(
    mut commands: Commands,
    mut events: EventReader<MouseButtonInput>,
    cursor_position: Res<CursorPosition>,
    box_selection_query: Query<(Entity, &mut BoxSelection)>,
    enemy_query: Query<Entity, (With<Selectable>, With<Selected>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    selection_circles_query: Query<Entity, With<SelectionCircle>>,
) {
    let material: Handle<ColorMaterial> = materials.add(Color::srgb(1.0, 0.0, 0.0));

    for event in events.read() {
        if event.button == MouseButton::Left {
            if let Some(cursor_position) = cursor_position.0 {
                match event.state {
                    ButtonState::Pressed => {
                        println!("Pressed: {:?} at {:?}", event.button, cursor_position);
                        let selected_box_entity =
                            spawn_cube(&mut commands, cursor_position.extend(0.0));

                        // Deselect previous selected entities
                        for enemy_entity in enemy_query.iter() {
                            commands.entity(enemy_entity).remove::<Selected>();
                        }

                        for selection_circle in selection_circles_query.iter() {
                            commands.entity(selection_circle).despawn();
                        }

                        commands
                            .entity(selected_box_entity)
                            .insert(BoxSelection {
                                start: cursor_position,
                                end: cursor_position,
                                selected: HashSet::default(),
                            })
                            .insert(CollisionGroups::new(SELECTION_GROUP, SELECTABLE_GROUP));
                    }
                    ButtonState::Released => {
                        println!("Released: {:?} at {:?}", event.button, cursor_position);
                        if let Ok((selected_box_entity, selected_box)) =
                            box_selection_query.get_single()
                        {
                            // Mark entities inside the selection box as selected
                            for selected in selected_box.selected.iter() {
                                commands.entity(*selected).insert(Selected).with_children(
                                    |parent| {
                                        parent.spawn((
                                            MaterialMesh2dBundle {
                                                mesh: meshes.add(Annulus::new(8.0, 8.5)).into(),
                                                material: material.clone(),
                                                transform: Transform::from_translation(vec3(
                                                    0.0, 0.0, 0.5,
                                                ))
                                                .with_scale(Vec3::splat(1.0)),
                                                ..Default::default()
                                            },
                                            SelectionCircle,
                                        ));
                                    },
                                );
                            }

                            // Despawn selection box
                            commands.entity(selected_box_entity).despawn();
                        }
                    }
                }
            }
        }
    }
}

fn handle_collision_events(
    mut events: EventReader<CollisionEvent>,
    mut box_selection_query: Query<(Entity, &mut BoxSelection)>,
) {
    if let Ok((entity, mut box_selection)) = box_selection_query.get_single_mut() {
        for event in events.read() {
            match dbg!(event) {
                CollisionEvent::Started(e1, e2, _flags) => {
                    if *e1 == entity {
                        box_selection.selected.insert(*e2);
                    } else if *e2 == entity {
                        box_selection.selected.insert(*e1);
                    }
                }
                CollisionEvent::Stopped(e1, e2, _flags) => {
                    if *e1 == entity {
                        box_selection.selected.remove(e2);
                    } else if *e2 == entity {
                        box_selection.selected.remove(e1);
                    }
                }
            }
        }
    }
}

const ENTITY_SIZE_IN_PIXELS: f32 = 64.0;
const ENTITY_SIZE_IN_METERS: f32 = 1.0;
const SELECTABLE_GROUP: Group = Group::GROUP_1;
const SELECTION_GROUP: Group = Group::GROUP_2;

fn spawn_cube(commands: &mut Commands, translation: Vec3) -> Entity {
    commands
        .spawn(Collider::cuboid(
            /* half_x */ ENTITY_SIZE_IN_METERS / 2.0,
            /* half_y */ ENTITY_SIZE_IN_METERS / 2.0,
        ))
        .insert(TransformBundle::from(Transform {
            translation,
            ..default()
        }))
        .insert(ActiveCollisionTypes::STATIC_STATIC)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor)
        .insert(CollisionGroups::new(SELECTABLE_GROUP, SELECTION_GROUP))
        .insert(ColliderDebugColor(Hsla {
            hue: 0.0,
            saturation: 0.0,
            lightness: 1.0,
            alpha: 1.0,
        }))
        .id()
}

#[derive(Component)]
struct BoxSelection {
    start: Vec2,
    end: Vec2,
    selected: HashSet<Entity>,
}

impl BoxSelection {
    fn display_gizmos(&self, gizmos: &mut Gizmos) {
        gizmos.circle_2d(self.start, 0.125, WHITE);
        gizmos.circle_2d(self.end, 0.125, WHITE);
    }
}

fn draw_box_selection(mut gizmos: Gizmos, box_selection_query: Query<&BoxSelection>) {
    if let Ok(box_selection) = box_selection_query.get_single() {
        box_selection.display_gizmos(&mut gizmos);
    }
}

fn mouse_motion_handler(
    mut gizmos: Gizmos,
    mut commands: Commands,
    mut events: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut box_selection_query: Query<(Entity, &mut BoxSelection)>,
) {
    if let Ok((entity, mut selected_box)) = box_selection_query.get_single_mut() {
        selected_box.display_gizmos(&mut gizmos);
        for event in events.read() {
            let (camera, camera_transform) = q_camera.single();
            if let Some(cursor_position) =
                camera.viewport_to_world_2d(camera_transform, event.position)
            {
                selected_box.end = cursor_position;

                let half_extents = (selected_box.start - selected_box.end).abs() / 2.0;
                let midpoint = (selected_box.start + selected_box.end) / 2.0;

                commands
                    .entity(entity)
                    .try_insert(Collider::cuboid(half_extents.x, half_extents.y))
                    .try_insert(Transform::from_xyz(midpoint.x, midpoint.y, 0.0));
            }
        }
    }
}
