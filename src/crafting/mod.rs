mod main_table;
mod metal_table;
mod wood_table;

use bevy::prelude::*;

use crate::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(main_table::plugin);

    app.add_systems(OnEnter(Screen::Gameplay), main_table::spawn_main_table);
}
