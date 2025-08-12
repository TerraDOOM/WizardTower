mod MainTable;
mod MetalTable;
mod WoodTable;

use bevy::prelude::*;

use crate::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(MainTable::plugin);

    app.add_systems(OnEnter(Screen::Gameplay), MainTable::spawn_main_table);
}
