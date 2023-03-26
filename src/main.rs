pub mod player;
pub mod debug;

use bevy::{prelude::*, window::PrimaryWindow};
use debug::DebugPlugin;
use player::PlayerPlugin;
use player::components::*;
use debug::components::*;
// use player::systems::*; 
// use player::components::*; 
// use debug::components::*;
// use debug::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .run();
}

// TODO
/*
Create an event reading system to scroll the camera accoring to the player's position
Create the concept of rooms
Implement an inventory interface
Implement a verb interface
Implement a verb + item interaction system
Create an inventory system
Modify the player moving system to not allow the player to scroll past the room's boundaries
 */

pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let starting_position = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);
    commands.spawn((
        Player {},
        Move {
            final_position: starting_position.translation.truncate(),
        },
        SpriteBundle {
            transform: starting_position,
            texture: asset_server.load("sprites\\map\\cactus.png"),
            ..default()
        },
    ));

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([TextSection::new(
            "FPS: ",
            TextStyle {
                font: asset_server.load("fonts/Iosevka.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
        )]),
        PosText {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Easel,
}

