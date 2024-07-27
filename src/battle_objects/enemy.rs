use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::battle_objects::coordinates::{Direction, GameCoord, GridCoord};
use crate::screens::battle::{BattleContext, BattleRenderable};

#[derive(Clone)]
pub struct Enemy{
	pub pos: GameCoord,
	pub snapped_facing_vector: Direction,
	pub health: (u32, u32),
	pub behavior: EnemyBehavior
}

impl Enemy {
	pub fn speed() -> f32 {
		1.5
	}
}

impl Enemy{
	const fn width() -> u32 {
		16
	}
}

impl BattleRenderable for Enemy{
	fn render(&self, canvas: &mut WindowCanvas, _background_texture: &Texture, ctx: &BattleContext){
		let camera_pos = ctx.camera_state.pos;
		let camera_scale = ctx.camera_state.scale;
		let enemy_rect = Rect::from_center(
			self.pos.to_display_coord(
				camera_pos,
				camera_scale,
				canvas.output_size().unwrap()
			),
			(camera_scale*Enemy::width() as f32) as u32,
			(camera_scale*Enemy::width() as f32) as u32
		);
		canvas.set_draw_color(Color::RGB(128, 0, 128));
		canvas.fill_rect(enemy_rect).unwrap();
	}
}

#[derive(Clone)]
pub enum EnemyBehavior{
	Idle,
	WalkToButton(u32, u32, Vec<GridCoord>), //path to the button
	TargetPlayer(u32, u32, Vec<GridCoord>), //path to the nearest player
	AttackWalls(u32, u32, Vec<GridCoord>),
}