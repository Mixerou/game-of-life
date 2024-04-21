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
        let world_width = (window.resolution.width() / CELL_SCALE) as usize;
        let world_height = (window.resolution.height() / CELL_SCALE) as usize;

        commands.insert_resource(World {
            cells: Vec::with_capacity(world_width * world_height),
            width: world_width,
            height: world_height,
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
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn calculate_cell_id(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn is_alive(&self, id: usize) -> bool {
        self.cells[id]
    }

    pub fn count_alive_around(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let x = x as isize;
        let y = y as isize;
        let width = self.width as isize;

        for cell_id in [
            (y - 1) * width + (x - 1),
            (y - 1) * width + x,
            (y - 1) * width + (x + 1),
            y * width + (x - 1),
            y * width + (x + 1),
            (y + 1) * width + (x - 1),
            (y + 1) * width + x,
            (y + 1) * width + (x + 1),
        ] {
            if cell_id < 0 {
                continue;
            }

            if let Some(cell) = self.cells.get(cell_id as usize) {
                if *cell {
                    count += 1
                }
            }
        }

        count
    }
}

#[derive(Resource)]
pub struct CellResources {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}
