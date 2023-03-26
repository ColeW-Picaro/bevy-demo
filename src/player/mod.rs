use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin  {
  fn build(&self, app: &mut App) {
    app.add_system(move_to_final_position)
        .add_system(click_set_final_position);
  }
}
