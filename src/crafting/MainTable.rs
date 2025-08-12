use bevy::prelude::*;

use crate::asset_tracking::LoadResource;
use crate::Focus;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<MergeAssets>();
    app.load_resource::<MergeAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct MergeAssets {
    #[dependency]
    background: Handle<Image>,
    #[dependency]
    placeholder: Handle<Image>,
}

impl FromWorld for MergeAssets {
    fn from_world(world: &mut World) -> Self {
        use crate::util::make_nearest;
        let assets = world.resource::<AssetServer>();
        Self {
            background: assets.load_with_settings("merge_assets/item_rocket.png", make_nearest),
            placeholder: assets.load_with_settings("merge_assets/item_rocket.png", make_nearest),
        }
    }
}

pub fn spawn_main_table(mut commands: Commands, merge_assets: Res<MergeAssets>) {
    commands.spawn((
        Name::new("Background"),
        //StateScoped(Focus::Crafting),
        Transform::from_xyz(0.0, 0.0, -1.0),
        Sprite {
            color: Color::Srgba(Srgba {
                red: 1.0,
                green: 0.3,
                blue: 0.3,
                alpha: 1.0,
            }),
            custom_size: Some(Vec2 { x: 800.0, y: 100.0 }),
            ..default()
        },
    ));
}
