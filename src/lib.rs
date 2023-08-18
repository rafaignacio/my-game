use bevy::{
    prelude::{Component, Deref, DerefMut},
    time::Timer,
};

pub mod plugins;

pub type Position = bevy::prelude::Vec3;

#[derive(Component, Default, Clone, Copy)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub trait Creature {
    fn get_current_life_points(&self) -> u32;
    fn get_current_position(&self) -> Position;
}
