use bevy::window::close_on_esc;
use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowMode},
};

use coding_pet::animation::AnimationPlugin;
use coding_pet::camera::FollowCameraPlugin;
use coding_pet::collision::CollisionPlugin;
use coding_pet::debug::DebugPlugin;
use coding_pet::enemy::EnemyPlugin;
use coding_pet::gui::GuiPlugin;
use coding_pet::gun::GunPlugin;
use coding_pet::player::PlayerPlugin;
use coding_pet::state::GameState;
use coding_pet::world::WorldPlugin;
use coding_pet::*;

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,coding_pet=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tiktok game + Bevy!".into(),
                        name: Some("MyWindow".into()),
                        resolution: (1920., 1080.).into(),
                        present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
                        enabled_buttons: bevy::window::EnabledButtons {
                            maximize: false,
                            ..Default::default()
                        },
                        visible: true,
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
        .add_plugins(DebugPlugin)
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
