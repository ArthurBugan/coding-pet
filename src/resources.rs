use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::state::GameState;

pub struct ResourcesPlugin;

#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
            image: None,
        }
    }
}

#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(CursorPosition(None))
            .add_systems(OnEnter(GameState::Loading), load_textures)
            .add_systems(Update, check_textures.run_if(in_state(GameState::Loading)))
            .add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Resource, Default)]
struct RpgSpriteFolder(Handle<LoadedFolder>);

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    // load multiple, individual sprites from a folder
    commands.insert_resource(RpgSpriteFolder(asset_server.load_folder("textures")));
}

fn check_textures(
    mut next_state: ResMut<NextState<GameState>>,
    rpg_sprite_folder: Res<RpgSpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
    mut handle: ResMut<GlobalTextureAtlas>,
) {
    // Advance the `GameState` once all sprite handles have been loaded by the `AssetServer`
    for event in events.read() {
        if event.is_loaded_with_dependencies(&rpg_sprite_folder.0) {
            next_state.set(GameState::MainMenu);
        }
    }
}

fn setup(
    mut commands: Commands,
    rpg_sprite_handles: Res<RpgSpriteFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
    mut handle: ResMut<GlobalTextureAtlas>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let loaded_folder = loaded_folders.get(&rpg_sprite_handles.0).unwrap();

    // create texture atlases with different padding and sampling

    let (texture_atlas_linear, linear_texture) =
        create_texture_atlas(loaded_folder, None, &mut textures);

    let atlas = textures.get(&linear_texture).unwrap();

    info!(
        "atlas width {:?}, atlas height {:?}",
        atlas.width(),
        atlas.height()
    );

    /*
      commands.spawn(SpriteBundle {
        texture: linear_texture.clone(),
        transform: Transform {
            translation: Vec3::new(-250.0, -130.0, 0.0),
            scale: Vec3::splat(0.8),
            ..default()
        },
        ..default()
    });
    */
    let grid_layout = TextureAtlasLayout::from_grid(
        Vec2::new(128.0, 128.0),     // Tile size
        50,                          // Width of the texture atlas
        50,                          // Height of the texture atlas
        Some(Vec2::new(64.0, 64.0)), // Optional minimum size
        Some(Vec2::new(32.0, 32.0)), // Optional maximum size
    );

    let texture_atlas_handle = texture_atlases.add(grid_layout);

    handle.layout = Some(texture_atlas_handle);
    handle.image = Some(linear_texture.clone());
}

/// Create a texture atlas with the given padding and sampling settings
/// from the individual sprites in the given folder.
fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder = TextureAtlasBuilder::default()
        .padding(padding.unwrap_or_default())
        .max_size(Vec2::new(8000.0, 10000.0));

    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        info!("id {}, path {:?}", id, handle.path());
        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();

    info!("size {}", texture_atlas_layout.size);
    let texture = textures.add(texture);

    (texture_atlas_layout, texture)
}

/// Create and spawn a sprite from a texture atlas
fn create_sprite_from_atlas(
    commands: &mut Commands,
    translation: (f32, f32, f32),
    sprite_index: usize,
    atlas_handle: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(translation.0, translation.1, translation.2),
                scale: Vec3::splat(3.0),
                ..default()
            },
            texture,
            ..default()
        },
        TextureAtlas {
            layout: atlas_handle,
            index: sprite_index,
        },
    ));
}

/// Create and spawn a label (text)
fn create_label(
    commands: &mut Commands,
    translation: (f32, f32, f32),
    text: &str,
    text_style: TextStyle,
) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(text, text_style).with_justify(JustifyText::Center),
        transform: Transform {
            translation: Vec3::new(translation.0, translation.1, translation.2),
            ..default()
        },
        ..default()
    });
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
