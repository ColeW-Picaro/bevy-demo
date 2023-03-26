use bevy::prelude::*;

pub mod components;
pub mod systems; 

use systems::*;

pub struct DebugPlugin; 

impl Plugin for DebugPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(update_pos_text);
  }
}