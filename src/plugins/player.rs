use bevy::prelude::*;

use crate::{AnimationIndices, AnimationTimer, Creature, Position};

const PLAYER_KEYS_DOWN: [KeyCode; 2] = [KeyCode::Down, KeyCode::S];
const PLAYER_KEYS_UP: [KeyCode; 2] = [KeyCode::Up, KeyCode::W];
const PLAYER_KEYS_RIGHT: [KeyCode; 2] = [KeyCode::Right, KeyCode::D];
const PLAYER_KEYS_LEFT: [KeyCode; 2] = [KeyCode::Left, KeyCode::A];

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Reflect)]
pub enum FacingDirection {
    North,
    #[default]
    South,
    East,
    West,
}
#[derive(Component, Default, Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Reflect), reflect(Component))]
pub struct PlayerAnimationIndices {
    pub idle_south: AnimationIndices,
    pub walking_south: AnimationIndices,
    pub idle_north: AnimationIndices,
    pub walking_north: AnimationIndices,
    pub idle_east: AnimationIndices,
    pub walking_east: AnimationIndices,
    pub idle_west: AnimationIndices,
    pub walking_west: AnimationIndices,
}

#[derive(Component, Default)]
#[cfg_attr(feature = "debug", derive(Reflect), reflect(Component))]
pub struct Player {
    pub level: u16,
    pub speed: f32,
    pub facing_direction: FacingDirection,
    pub life_points: u32,
    pub position: Position,
    pub animations: PlayerAnimationIndices,
    pub is_walking: bool,
    pub current_animation_index: AnimationIndices,
    pub keyboard_last_action: Timer,
}

impl Creature for Player {
    fn get_current_life_points(&self) -> u32 {
        self.life_points
    }

    fn get_current_position(&self) -> Position {
        self.position
    }
}

impl Player {
    fn new() -> Player {
        let mut player = Player {
            speed: 10.,
            level: 1,
            facing_direction: FacingDirection::South,
            life_points: 100,
            position: Vec3::splat(0.),
            animations: PlayerAnimationIndices::default(),
            is_walking: false,
            current_animation_index: AnimationIndices::default(),
            keyboard_last_action: Timer::from_seconds(1., TimerMode::Once),
        };
        player.setup_animations();

        player.current_animation_index = player.animations.idle_south;

        player
    }

    fn setup_animations(&mut self) {
        self.animations = PlayerAnimationIndices {
            idle_south: AnimationIndices { first: 0, last: 11 },
            walking_south: AnimationIndices {
                first: 12,
                last: 19,
            },
            idle_north: AnimationIndices {
                first: 20,
                last: 31,
            },
            walking_north: AnimationIndices {
                first: 32,
                last: 39,
            },
            idle_east: AnimationIndices {
                first: 40,
                last: 51,
            },
            walking_east: AnimationIndices {
                first: 52,
                last: 59,
            },
            idle_west: AnimationIndices {
                first: 60,
                last: 71,
            },
            walking_west: AnimationIndices {
                first: 71,
                last: 79,
            },
        }
    }

    fn walk(&mut self, facing_direction: FacingDirection, is_walking: bool) -> AnimationIndices {
        self.facing_direction = facing_direction;
        self.is_walking = is_walking;

        self.current_animation_index = match (facing_direction, is_walking) {
            (FacingDirection::East, true) => {
                self.position.x += 1.;
                self.animations.walking_east
            }
            (FacingDirection::East, false) => self.animations.idle_east,
            (FacingDirection::West, true) => {
                self.position.x -= 1.;
                self.animations.walking_west
            }
            (FacingDirection::West, false) => self.animations.idle_west,
            (FacingDirection::North, true) => {
                self.position.y += 1.;
                self.animations.walking_north
            }
            (FacingDirection::North, false) => self.animations.idle_north,
            (FacingDirection::South, true) => {
                self.position.y -= 1.;
                self.animations.walking_south
            }
            (FacingDirection::South, false) => self.animations.idle_south,
        };

        self.current_animation_index
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, animate_player);
        #[cfg(feature = "debug")]
        app.register_type::<Player>();
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    let atlas_texture = asset_server.load("sprites/base_model-Sheet.png");
    let atlas = TextureAtlas::from_grid(atlas_texture, Vec2::new(64., 80.), 20, 4, None, None);
    let atlas_handle = texture_atlas.add(atlas);
    let player = Player::new();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        player.current_animation_index,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        player,
    ));
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
) {
    query.for_each_mut(|(indices, mut timer, mut sprites)| {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprites.index = if sprites.index == indices.last {
                indices.first
            } else {
                sprites.index + 1
            };
        }
    });
}

fn move_player(
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite, &mut AnimationIndices)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player, mut sprites, mut indices) = player_query.single_mut();
    player.keyboard_last_action.tick(time.delta());

    if input.any_just_released(PLAYER_KEYS_DOWN)
        || input.any_just_released(PLAYER_KEYS_LEFT)
        || input.any_just_released(PLAYER_KEYS_RIGHT)
        || input.any_just_released(PLAYER_KEYS_UP)
    {
        let f = player.facing_direction;
        *indices = player.walk(f, false);
        sprites.index = player.current_animation_index.first;
    }

    if player.is_walking && !player.keyboard_last_action.finished() {
        return;
    }

    player.keyboard_last_action.reset();

    if input.any_pressed(PLAYER_KEYS_DOWN) {
        *indices = player.walk(FacingDirection::South, true);
        sprites.index = player.current_animation_index.first;
    }

    if input.any_pressed(PLAYER_KEYS_UP) {
        *indices = player.walk(FacingDirection::North, true);
        sprites.index = player.current_animation_index.first;
    }

    if input.any_pressed(PLAYER_KEYS_RIGHT) {
        *indices = player.walk(FacingDirection::East, true);
        sprites.index = player.current_animation_index.first;
    }

    if input.any_pressed(PLAYER_KEYS_LEFT) {
        *indices = player.walk(FacingDirection::West, true);
        sprites.index = player.current_animation_index.first;
    }
}
