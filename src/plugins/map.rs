use bevy::{prelude::*, window::PrimaryWindow};

pub struct MapPlugin;

//TODO: Load tiles based on player's position
//TODO: Map screen size and place sprites for every tile.
//TODO: Associate map chunk to every visible tile.
//TODO: Update tiles when position is updated.

#[derive(Component)]
pub struct MapChunk {
    position: Vec3,
    is_walkable: bool,
    texture: Option<Handle<Image>>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let map_entity = commands.spawn_empty().id();

    let tiles_count = (window.height() / 64.) * (window.width() / 64.);

    [0f32..tiles_count].iter().enumerate().map(|idx| idx.0 * 2.);
    commands.entity(map_entity).insert(MapChunk {
        position: Vec3::splat(1.),
        is_walkable: true,
        texture: None,
    });
}
