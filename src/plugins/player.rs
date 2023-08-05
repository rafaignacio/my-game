use bevy::prelude::*;

#[derive(Default)]
#[cfg_attr(feature = "debug", derive(Reflect))]
pub enum FacingDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

#[derive(Resource)]
pub struct PlayerSprites {
    facing_up: Handle<Image>,
    facing_down: Handle<Image>,
}

#[derive(Component, Default)]
#[cfg_attr(feature = "debug", derive(Reflect), reflect(Component))]
pub struct Player {
    pub level: u16,
    pub speed: f32,
    pub facing_direction: FacingDirection,
}

impl Player {
    fn change_direction(&mut self, facing_direction: FacingDirection) {
        self.facing_direction = facing_direction;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_player_sprites)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, update_player_sprite);
        #[cfg(feature = "debug")]
        app.register_type::<Player>();
    }
}

fn load_player_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerSprites {
        facing_down: asset_server.load("sprites/character_maleAdventurer_idle.png"),
        facing_up: asset_server.load("sprites/character_maleAdventurer_back.png"),
    })
}

fn spawn_player(mut commands: Commands, player_sprites: Res<PlayerSprites>) {
    commands.spawn((
        SpriteBundle {
            texture: player_sprites.facing_down.clone(),
            ..default()
        },
        Player {
            level: 1,
            speed: 100.0,
            facing_direction: FacingDirection::Down,
        },
    ));
}

fn move_player(mut player_query: Query<&mut Player>, input: Res<Input<KeyCode>>) {
    let mut player = player_query.single_mut();
    if input.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        player.change_direction(FacingDirection::Down);
    }

    if input.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        player.change_direction(FacingDirection::Up);
    }
}

fn update_player_sprite(
    mut player_query: Query<(&mut Handle<Image>, &Player), With<Player>>,
    player_sprites: Res<PlayerSprites>,
) {
    let (mut texture, player) = player_query.single_mut();
    match player.facing_direction {
        FacingDirection::Down => {
            *texture = player_sprites.facing_down.clone();
        }
        FacingDirection::Up => {
            *texture = player_sprites.facing_up.clone();
        }
        FacingDirection::Left => todo!(),
        FacingDirection::Right => todo!(),
    }
}
