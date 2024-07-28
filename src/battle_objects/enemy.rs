use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::battle_objects::buildables::Wall;
use crate::battle_objects::coordinates::{Direction, GameCoord, GridCoord};
use crate::screens::battle::{BattleContext, BattleRenderable};
use crate::utils::collisions::line_to_square_intersect;

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
	const fn width() -> u32 {
		16
	}

	pub fn attack_power() -> u32 {
		12
	}

	pub fn get_collisions(&self, top_wall: Option<&Wall>, right_wall: Option<&Wall>,
	                      bottom_wall: Option<&Wall>, left_wall: Option<&Wall>
	) -> (bool, bool, bool, bool){
		let mut collisions = (false, false, false, false);
		let player_square = ||-> ((i32, i32),u32){
			(self.pos.into(), Enemy::width())
		};
		if let Some(wall) = top_wall {
			if line_to_square_intersect((wall.endpoints.0.into(), wall.endpoints.1.into()), player_square()){
				collisions.0 = true;
			}
		}
		if let Some(wall) = right_wall {
			if line_to_square_intersect((wall.endpoints.0.into(), wall.endpoints.1.into()), player_square()){
				collisions.1 = true;
			}
		}
		if let Some(wall) = bottom_wall {
			if line_to_square_intersect((wall.endpoints.0.into(), wall.endpoints.1.into()), player_square()){
				collisions.2 = true;
			}
		}
		if let Some(wall) = left_wall {
			if line_to_square_intersect((wall.endpoints.0.into(), wall.endpoints.1.into()), player_square()){
				collisions.3 = true;
			}
		}
		collisions
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