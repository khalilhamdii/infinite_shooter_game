use bevy::prelude::*;
use std::env;

use infinite_shooter_game::animation::AnimationPlugin;
use infinite_shooter_game::camera::FollowCameraPlugin;
use infinite_shooter_game::collision::CollisionPlugin;
use infinite_shooter_game::enemy::EnemyPlugin;
use infinite_shooter_game::gui::GuiPlugin;
use infinite_shooter_game::gun::GunPlugin;
use infinite_shooter_game::player::PlayerPlugin;
use infinite_shooter_game::state::GameState;
use infinite_shooter_game::world::WorldPlugin;
use infinite_shooter_game::*;

fn main() {
    // env::set_var("RUST_BACKTRACE", "1"); // Uncomment to enable backtrace
    App::new()
        // .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // mode: bevy::window::WindowMode::Fullscreen,
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .add_plugins(FollowCameraPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .insert_resource(Msaa::Off)
        .add_systems(Update, close_on_esc)
        .run();
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
