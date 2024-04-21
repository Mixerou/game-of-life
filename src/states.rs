use bevy::app::{App, Plugin};
use bevy::prelude::States;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    InitialisingCamera,
    InitialisingResources,
    InitialisingWorld,
    Playing,
}

impl AppState {
    pub fn next(&self) -> Self {
        match self {
            AppState::InitialisingCamera => AppState::InitialisingResources,
            AppState::InitialisingResources => AppState::InitialisingWorld,
            AppState::InitialisingWorld => AppState::Playing,
            AppState::Playing => AppState::Playing,
        }
    }
}
