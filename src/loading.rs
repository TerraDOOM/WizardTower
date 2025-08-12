//! A loading screen during which game assets are loaded if necessary.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{asset_tracking::ResourceHandles, Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_loading_screen);

    app.add_systems(Update, enter_gameplay_screen.run_if(all_assets_loaded));
}

fn spawn_loading_screen(mut commands: Commands) {
    println!("Loading");
}

fn enter_gameplay_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    println!("Everything loaded");
    resource_handles.is_all_done()
}
