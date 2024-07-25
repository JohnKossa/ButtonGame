use crate::battle_objects::coordinates::GameCoord;

#[derive(Clone)]
pub struct CameraState{
	pub pos: GameCoord,
	pub scale: f32,
}

impl CameraState{
	pub fn new() -> CameraState{
		CameraState{pos: GameCoord{x:0, y:0}, scale:1.1}
	}

	pub fn smooth_scroll(&mut self, target: &GameCoord){
		let dx = target.x - self.pos.x;
		let dy = target.y - self.pos.y;
		self.pos.x = self.pos.x + (0.1 * dx as f32) as i32;
		self.pos.y = self.pos.y + (0.1 * dy as f32) as i32;
	}
}