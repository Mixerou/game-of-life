use bevy::app::{App, Plugin, Update};
use bevy::asset::{Assets, Handle};
use bevy::math::Vec2;
use bevy::prelude::{
    in_state, Color, Commands, IntoSystemConfigs, Mesh, NextState, Query, Rectangle, ResMut,
    Resource, State, Window,
};
use bevy::sprite::ColorMaterial;

use crate::constants::CELL_SCALE;
use crate::states::AppState;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::setup.run_if(in_state(AppState::InitialisingResources)),
        );
    }
}

impl ResourcesPlugin {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut app_state_setter: ResMut<NextState<AppState>>,
        app_state: ResMut<State<AppState>>,
        window: Query<&Window>,
    ) {
        // World
        let Some(window) = window.iter().next() else {
            return;
        };
        let window_width = window.resolution.width() as usize;
        let window_height = window.resolution.height() as usize;

        commands.insert_resource(World {
            cells: Vec::with_capacity(window_width * window_height),
        });

        // CellResources
        let mesh = Rectangle::from_size(Vec2::splat(1. * CELL_SCALE));
        let mesh = meshes.add(mesh);

        let material = ColorMaterial::from(Color::rgb_u8(212, 211, 208));
        let material = materials.add(material);

        commands.insert_resource(CellResources { mesh, material });

        app_state_setter.set(app_state.next());
    }
}

#[derive(Resource)]
pub struct World {
    pub cells: Vec<bool>,
}

#[derive(Resource)]
pub struct CellResources {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}
