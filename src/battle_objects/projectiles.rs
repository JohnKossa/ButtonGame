use crate::battle_objects::coordinates::GameCoord;

#[derive(Clone)]
pub struct FriendlyProjectile{
	pub source_pos: GameCoord,
	pub target_pos: GameCoord,
	pub speed: f32,
	pub damage: i32,
}