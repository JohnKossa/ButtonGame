use crate::screens::battle::{PlayerState};
use crate::battle_objects::coordinates::GridCoord;

pub struct OtherPlayer{
	pub grid_coord: GridCoord,
	pub facing_vector: f32,
	pub state: PlayerState,
	pub health: (u32, u32)
}