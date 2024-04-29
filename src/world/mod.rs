use std::time::{SystemTime, UNIX_EPOCH};

use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec2;
use bevy::prelude::{
    in_state, Camera, Commands, Entity, GlobalTransform, IntoSystemConfigs, NextState, Query, Res,
    ResMut, State, Transform,
};
use bevy::sprite::{ColorMesh2dBundle, Mesh2dHandle};
use itertools::iproduct;
use oorandom::Rand32;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::constants::CELL_SCALE;
use crate::resources::{CellResources, World};
use crate::states::AppState;
use crate::world::components::Cell;

pub mod components;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::setup.run_if(in_state(AppState::InitialisingWorld)),
                Self::make_iteration.run_if(in_state(AppState::Playing)),
            ),
        );
    }
}

impl WorldPlugin {
    fn spawn_cell(
        commands: &mut Commands,
        cell_resources: &Res<CellResources>,
        camera: &Camera,
        camera_transform: &GlobalTransform,
        cell_id: usize,
        cell_x: f32,
        cell_y: f32,
    ) {
        let Some(translation) = camera.viewport_to_world_2d(
            camera_transform,
            Vec2::new(cell_x * CELL_SCALE, cell_y * CELL_SCALE),
        ) else {
            return;
        };

        commands.spawn((
            ColorMesh2dBundle {
                mesh: Mesh2dHandle::from(cell_resources.mesh.clone_weak()),
                material: cell_resources.material.clone_weak(),
                transform: Transform::from_xyz(translation.x, translation.y, 0.),
                ..Default::default()
            },
            Cell { id: cell_id },
        ));
    }
}

// Systems
impl WorldPlugin {
    fn setup(
        mut commands: Commands,
        mut world: ResMut<World>,
        mut app_state_setter: ResMut<NextState<AppState>>,
        app_state: Res<State<AppState>>,
        cell_resources: Res<CellResources>,
        camera: Query<(&Camera, &GlobalTransform)>,
    ) {
        let Some((camera, camera_transform)) = camera.iter().next() else {
            return;
        };

        let current_time = SystemTime::now();
        let current_time = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let mut random_generator = Rand32::new(current_time.as_secs());

        for y in 0..world.height {
            for x in 0..world.width {
                if random_generator.rand_range(0..101) > 5 {
                    world.cells.push(false);
                    continue;
                }

                world.cells.push(true);
                Self::spawn_cell(
                    &mut commands,
                    &cell_resources,
                    camera,
                    camera_transform,
                    world.calculate_cell_id(x, y),
                    x as f32,
                    y as f32,
                );
            }
        }

        app_state_setter.set(app_state.next());
    }

    fn make_iteration(
        mut commands: Commands,
        mut world: ResMut<World>,
        cells: Query<(Entity, &Cell)>,
        camera: Query<(&Camera, &GlobalTransform)>,
        cell_resources: Res<CellResources>,
    ) {
        let Some((camera, camera_transform)) = camera.iter().next() else {
            return;
        };

        let mut for_despawn = Vec::new();

        iproduct!(0..world.width, 0..world.height)
            .par_bridge()
            .map(|(x, y)| {
                let alive_around = world.count_alive_around(x, y);

                match alive_around == 2 {
                    true => (None, None),
                    false => {
                        let cell_id = world.calculate_cell_id(x, y);
                        let is_alive = world.is_alive(cell_id);

                        match true {
                            true if is_alive && alive_around != 3 => (None, Some(cell_id)),
                            true if !is_alive && alive_around == 3 => {
                                (Some((cell_id, x as f32, y as f32)), None)
                            }
                            _ => (None, None),
                        }
                    }
                }
            })
            .collect_vec_list()
            .iter()
            .flatten()
            .for_each(|(for_spawn, cell_id_for_despawn)| {
                if let Some((cell_id, x, y)) = for_spawn {
                    world.cells[*cell_id] = true;

                    Self::spawn_cell(
                        &mut commands,
                        &cell_resources,
                        camera,
                        camera_transform,
                        *cell_id,
                        *x,
                        *y,
                    );
                }

                if let Some(cell_id) = cell_id_for_despawn {
                    for_despawn.push(*cell_id);
                }
            });

        cells.iter().for_each(|(entity, cell)| {
            if for_despawn.contains(&cell.id) {
                if let Some(entity) = commands.get_entity(entity) {
                    world.cells[cell.id] = false;
                    entity.despawn_recursive();
                }
            }
        });
    }
}
