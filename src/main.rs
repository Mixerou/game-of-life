use bevy::app::{App, PluginGroup};
use bevy::window::{PresentMode, Window, WindowPlugin, WindowResizeConstraints};
use bevy::DefaultPlugins;

use crate::camera::CameraPlugin;
use crate::world::WorldPlugin;

mod camera;
mod constants;
mod world;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: (1280., 720.).into(),
            title: "The Game of Life".into(),
            resize_constraints: WindowResizeConstraints {
                min_width: 640.,
                min_height: 360.,
                ..Default::default()
            },
            present_mode: PresentMode::Fifo,
            ..Default::default()
        }),
        ..Default::default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .run();
}
