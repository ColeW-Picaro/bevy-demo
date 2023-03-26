use bevy::prelude::*;
use bevy::window::*;
use super::components::*;

pub const PLAYER_SPEED: f32 = 400.00;

pub fn click_set_final_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<Input<MouseButton>>,
    mut player_move_query: Query<&mut Move, With<Player>>,
) {
    let window = window_query.get_single().unwrap();
    let mut player = player_move_query.get_single_mut().unwrap();
    if mouse_input.just_pressed(MouseButton::Left) {
        player.final_position = window.cursor_position().unwrap();
    }
}

pub fn move_to_final_position(
    mut player_transform_query: Query<(&mut Transform, &Move), With<Player>>,
    time: Res<Time>
) {
    let (mut player_transform, mut player_move) = player_transform_query.get_single_mut().unwrap();
    let direction = player_move.final_position.extend(0.0) - player_transform.translation;
    let distance = direction.length();

    if distance > (PLAYER_SPEED * time.delta_seconds()) {
        // If the player has not yet reached the destination, move towards it
        let velocity = direction.normalize() * PLAYER_SPEED * time.delta_seconds();
        player_transform.translation += velocity;
    } else {
        // If the player has reached the destination, snap to it
        player_transform.translation = player_move.final_position.extend(0.0);
    }
}