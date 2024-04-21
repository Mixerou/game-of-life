use bevy::app::{App, Plugin, Startup};
use bevy::asset::Assets;
use bevy::math::Vec2;
use bevy::prelude::{Color, Commands, Mesh, Rectangle, ResMut};
use bevy::sprite::{ColorMaterial, ColorMesh2dBundle, Mesh2dHandle};

use crate::constants::ENTITY_SCALE;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup);
    }
}

impl WorldPlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let mesh = Rectangle::from_size(Vec2::splat(1. * ENTITY_SCALE));
        let mesh = meshes.add(mesh);

        let material = ColorMaterial::from(Color::rgb_u8(212, 211, 208));
        let material = materials.add(material);

        commands.spawn(ColorMesh2dBundle {
            mesh: Mesh2dHandle::from(mesh),
            material,
            ..Default::default()
        });
    }
}
