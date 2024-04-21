use bevy::app::{App, PluginGroup};
use bevy::window::{EnabledButtons, PresentMode, Window, WindowPlugin};
use bevy::DefaultPlugins;

use crate::camera::CameraPlugin;
use crate::resources::ResourcesPlugin;
use crate::states::StatesPlugin;
use crate::world::WorldPlugin;

mod camera;
mod constants;
mod resources;
mod states;
mod world;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: (1280., 720.).into(),
            title: "The Game of Life".into(),
            resizable: false,
            enabled_buttons: EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            present_mode: PresentMode::Fifo,
            ..Default::default()
        }),
        ..Default::default()
    };

    App::new()
        .add_plugins(CameraPlugin)
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(ResourcesPlugin)
        .add_plugins(StatesPlugin)
        .add_plugins(WorldPlugin)
        .run();
}
