use bevy::prelude::*;

pub struct MapPlugin;
use super::helpers;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, load_map)
            .add_systems(Update, helpers::camera::movement);
    }
}

fn load_map(mut commands: Commands, asset_server: Res<AssetServer>) {}
