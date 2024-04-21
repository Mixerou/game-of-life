use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{
    in_state, Camera2dBundle, Commands, IntoSystemConfigs, NextState, Res, ResMut, State,
};

use crate::states::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            Self::setup.run_if(in_state(AppState::InitialisingCamera)),
        );
    }
}

impl CameraPlugin {
    fn setup(
        mut commands: Commands,
        mut app_state_setter: ResMut<NextState<AppState>>,
        app_state: Res<State<AppState>>,
    ) {
        commands.spawn(Camera2dBundle::default());
        app_state_setter.set(app_state.next());
    }
}
