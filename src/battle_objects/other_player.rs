use crate::battle_objects::battle_player::PlayerState;
use crate::battle_objects::coordinates::GridCoord;

#[derive(Clone)]
pub struct OtherPlayer{
	pub grid_coord: GridCoord,
	pub facing_vector: f32,
	pub state: PlayerState,
	pub health: (u32, u32)
}