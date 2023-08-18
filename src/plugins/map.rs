use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, load_map);
    }
}

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {}
