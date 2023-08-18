use bevy::{prelude::*, time::Timer};

pub mod plugins;

pub type Position = bevy::prelude::Vec3;

#[derive(Component, Default, Clone, Copy)]
#[cfg_attr(feature = "debug", derive(Reflect))]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
#[cfg_attr(feature = "debug", derive(Reflect))]
pub struct AnimationTimer(Timer);

pub trait Creature {
    fn get_current_life_points(&self) -> u32;
    fn get_current_position(&self) -> Position;
}
