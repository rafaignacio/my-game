use bevy::{prelude::*, window::PrimaryWindow};

pub struct MapPlugin;

//TODO: Load tiles based on player's position
//TODO: Map screen size and place sprites for every tile.
//TODO: Associate map chunk to every visible tile.
//TODO: Update tiles when position is updated.

const DEFAULT_TILE_SIZE: f32 = 64.;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MapChunk {
    position: Vec3,
    is_walkable: bool,
    texture: Option<Handle<Image>>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        #[cfg(feature = "debug")]
        app.register_type::<MapChunk>();
    }
}

fn create_map_chunk(position: Vec3) -> (SpriteBundle, MapChunk) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(position.x, position.y, 0.3),
                custom_size: Some(Vec2::splat(64.)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..Default::default()
        },
        MapChunk {
            position: Vec3 {
                x: f32::round(position.x / DEFAULT_TILE_SIZE),
                y: f32::round(position.y / DEFAULT_TILE_SIZE),
                z: 0.,
            },
            is_walkable: true,
            texture: None,
        },
    )
}

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let (height, width) = (window.height(), window.width());

    let tiles_count = (((height + DEFAULT_TILE_SIZE) / DEFAULT_TILE_SIZE)
        * ((width + DEFAULT_TILE_SIZE) / DEFAULT_TILE_SIZE)) as u32;
    let width_tiles_count = ((width / DEFAULT_TILE_SIZE) as u32) + 1;

    let (mut x, mut y) = (-(width / 2.), -(height / 2.));
    let chunks: Vec<Entity> = (0..tiles_count)
        .map(|i| {
            let pos = Vec3::new(x, y, 0.);
            x += DEFAULT_TILE_SIZE;

            if i > 0 && i % width_tiles_count == 0 {
                y += DEFAULT_TILE_SIZE;
                x = -(width / 2.);
            }
            commands.spawn(create_map_chunk(pos)).id()
        })
        .collect();
    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("Map"))
        .push_children(&chunks);
}
