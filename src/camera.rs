use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Camera2dBundle, Commands};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
    }
}

impl CameraPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn(Camera2dBundle::default());
    }
}
