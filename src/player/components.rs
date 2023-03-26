use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Move {
    pub final_position: Vec2,
}