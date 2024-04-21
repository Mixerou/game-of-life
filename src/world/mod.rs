use bevy::app::{App, Plugin, Update};
use bevy::math::Vec2;
use bevy::prelude::{
    in_state, Camera, Commands, GlobalTransform, IntoSystemConfigs, NextState, Query, Res, ResMut,
    State, Transform,
};
use bevy::sprite::{ColorMesh2dBundle, Mesh2dHandle};
use bevy::window::Window;
use oorandom::Rand32;

use crate::constants::CELL_SCALE;
use crate::resources::{CellResources, World};
use crate::states::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::setup.run_if(in_state(AppState::InitialisingWorld)),
        );
    }
}

impl WorldPlugin {
    fn setup(
        mut commands: Commands,
        mut world: ResMut<World>,
        mut app_state_setter: ResMut<NextState<AppState>>,
        app_state: Res<State<AppState>>,
        cell_resources: Res<CellResources>,
        camera: Query<(&Camera, &GlobalTransform)>,
        window: Query<&Window>,
    ) {
        let Some((camera, camera_transform)) = camera.iter().next() else {
            return;
        };
        let Some(window) = window.iter().next() else {
            return;
        };

        let mut random_generator = Rand32::new(0);

        for x in (0..window.resolution.width() as u32).step_by(CELL_SCALE as usize) {
            for y in (0..window.resolution.height() as u32).step_by(CELL_SCALE as usize) {
                if random_generator.rand_range(0..101) < 25 {
                    world.cells.push(false);
                    continue;
                }

                world.cells.push(true);

                let Some(point) =
                    camera.viewport_to_world_2d(camera_transform, Vec2::new(x as f32, y as f32))
                else {
                    return;
                };

                commands.spawn(ColorMesh2dBundle {
                    mesh: Mesh2dHandle::from(cell_resources.mesh.clone_weak()),
                    material: cell_resources.material.clone_weak(),
                    transform: Transform::from_xyz(point.x, point.y, 0.),
                    ..Default::default()
                });
            }
        }

        app_state_setter.set(app_state.next());
    }
}
