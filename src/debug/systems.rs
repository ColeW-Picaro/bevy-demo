use bevy::prelude::*;
use super::components::*;
use crate::player::components::*;

pub fn update_pos_text(
    player_query: Query<&Transform, With<Player>>,
    mut text_query: Query<&mut Text, With<PosText>>,
) {
    let transform = player_query.get_single().unwrap();
    let mut text = text_query.get_single_mut().unwrap();
    text.sections[0].value = format!("{t}", t = transform.translation);
}
