use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState;
use crate::*;

pub struct ResourcesPlugin;

#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct BigTreeTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct SmallTreeTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Loading)
            .insert_resource(GlobalTextureAtlas::default())
            .insert_resource(BigTreeTextureAtlas::default())
            .insert_resource(SmallTreeTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)),
            );
    }
}

fn load_assets(
    mut global_handle: ResMut<GlobalTextureAtlas>,
    mut big_tree_handle: ResMut<BigTreeTextureAtlas>,
    mut small_tree_handle: ResMut<SmallTreeTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    global_handle.image = Some(asset_server.load(GLOBAL_SPRITE_SHEET_PATH));

    let global_layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    global_handle.layout = Some(texture_atlas_layouts.add(global_layout));

    big_tree_handle.image = Some(asset_server.load(BIG_TREE_SPRITE_SHEET_PATH));

    let big_tree_layout = TextureAtlasLayout::from_grid(UVec2::new(48, 64), 1, 1, None, None);
    big_tree_handle.layout = Some(texture_atlas_layouts.add(big_tree_layout));

    small_tree_handle.image = Some(asset_server.load(SMALL_TREE_SPRITE_SHEET_PATH));

    let small_tree_layout = TextureAtlasLayout::from_grid(UVec2::new(32, 48), 1, 1, None, None);
    small_tree_handle.layout = Some(texture_atlas_layouts.add(small_tree_layout));

    next_state.set(GameState::MainMenu);
}

fn update_cursor_position(
    mut cursor_pos: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_pos.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();
    cursor_pos.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

impl Default for BigTreeTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

impl Default for SmallTreeTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}
