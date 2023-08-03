use bevy::prelude::*;

#[derive(Default, Reflect)]
pub enum FacingDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
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
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
        #[cfg(feature = "debug")]
        app.register_type::<Player>();
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/character_maleAdventurer_idle.png"),
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
