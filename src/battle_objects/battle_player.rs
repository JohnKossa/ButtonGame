use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use crate::battle_objects::buildables::Wall;
use crate::battle_objects::coordinates::{Direction, GameCoord};
use crate::screens::battle::{ActionButton, BattleContext, BattleRenderable};
use crate::utils::collisions::line_to_square_intersect;
use crate::utils::render_utils::render_progress_bar;

#[derive(Clone, Copy)]
pub struct BattlePlayerContext{
	pub game_coord: GameCoord,
	pub facing_vector: f32,
	pub base_vision_range: u8,
	pub ability_primary: Ability,
	pub ability_secondary: Ability,
	pub snapped_facing_vector: Direction,
	pub state: PlayerState,
}

impl BattlePlayerContext{
	fn width() -> u32{
		16
	}

	fn display_corners(&self, width: u32, scale_factor: f32, center_point: GameCoord, window_dimensions:(u32, u32)) -> (Point, Point, Point, Point){
		let top_left = GameCoord{x: self.game_coord.x - width as i32/2, y: self.game_coord.y - width as i32/2};
		let top_right = GameCoord{x: self.game_coord.x + width as i32/2, y: self.game_coord.y - width as i32/2};
		let bottom_left = GameCoord{x: self.game_coord.x - width as i32/2, y: self.game_coord.y + width as i32/2};
		let bottom_right = GameCoord{x: self.game_coord.x + width as i32/2, y: self.game_coord.y + width as i32/2};
		(
			top_left.to_display_coord(center_point, scale_factor, window_dimensions),
			top_right.to_display_coord(center_point, scale_factor, window_dimensions),
			bottom_left.to_display_coord(center_point, scale_factor, window_dimensions),
			bottom_right.to_display_coord(center_point, scale_factor, window_dimensions)
		)
	}

	fn edge_coords(&self, width: u32, scale_factor: f32, center_point: GameCoord, window_dimensions: (u32, u32)) -> (Point, Point){
		let corners = self.display_corners(width, scale_factor, center_point, window_dimensions);
		match self.snapped_facing_vector {
			Direction::North => (corners.0, corners.1),
			Direction::South => (corners.2, corners.3),
			Direction::West => (corners.0, corners.2),
			Direction::East => (corners.1, corners.3),
		}
	}

	pub(crate) fn get_vision_range(&self) -> u8{
		match (self.ability_primary, self.ability_secondary) {
			(Ability::Vision, _) => 2*self.base_vision_range,
			(_, Ability::Vision) => 2*self.base_vision_range,
			(_, _) => self.base_vision_range
		}
	}

	pub(crate) fn get_collisions(&self, top_wall: Option<&Wall>, right_wall: Option<&Wall>, bottom_wall: Option<&Wall>, left_wall: Option<&Wall>) -> (bool, bool, bool, bool){
		let mut collisions = (false, false, false, false);
		let player_square = ||-> ((i32, i32),u32){
			(self.game_coord.into(), BattlePlayerContext::width())
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

impl BattleRenderable for BattlePlayerContext{
	fn render(&self, canvas: &mut WindowCanvas, _background_texture: &Texture, ctx: &BattleContext){
		let player = &ctx.player;
		let camera = &ctx.camera_state;
		let canvas_size = canvas.output_size().unwrap();
		let player_rect = Rect::from_center(
			player.game_coord.to_display_coord(
				camera.pos,
				camera.scale,
				canvas.output_size().unwrap()),
			(camera.scale * BattlePlayerContext::width() as f32) as u32,
			(camera.scale * BattlePlayerContext::width() as f32) as u32);
		let player_facing_indicator_points = player.edge_coords(BattlePlayerContext::width(), camera.scale, camera.pos, canvas_size);
		let player_color = match player.state{
			PlayerState::Standing => Color::RED,
			PlayerState::Running => Color::YELLOW,
			PlayerState::Learning(_,_,_) => Color::RGB(255, 127, 0),
			PlayerState::BuildPlacing(_,_) => Color::RGB(255, 127,0),
			PlayerState::MeleeAttacking(_, _) => Color::RGB(255, 127, 0),
			PlayerState::RangeTargeting => Color::RGB(255, 127, 0),
			PlayerState::RangeAttacking(_,_) => Color::RGB(255, 127, 0),
			PlayerState::ButtonPressing(_,_) => Color::RGB(255, 127, 0),
			PlayerState::BuildChoosing => Color::RGB(255, 127, 0),
			PlayerState::Repairing(_,_) => Color::RGB(255, 127, 0),
			PlayerState::Healing(_,_) => Color::RGB(255, 127, 0),
		};
		canvas.set_draw_color(player_color);
		canvas.fill_rect(player_rect).unwrap();
		let mut render_progress = |cur, max| {
			render_progress_bar(
				canvas,
				player_rect.x(),
				player_rect.y(),
				player_rect.width(),
				player_rect.height(),
				(cur as usize, max as usize)
			);
		};
		match player.state {
			PlayerState::Learning(_, cur, max) => {
				render_progress(cur, max);
			},
			PlayerState::Standing => {}
			PlayerState::Running => {}
			PlayerState::MeleeAttacking(cur, max) => {
				render_progress(cur, max);
			}
			PlayerState::RangeTargeting => {}
			PlayerState::RangeAttacking(cur, max) => {
				render_progress(cur, max);
			}
			PlayerState::ButtonPressing(_,_) => {}
			PlayerState::BuildChoosing => {}
			PlayerState::BuildPlacing(cur, max) => {
				render_progress(cur, max);
			}
			PlayerState::Repairing(cur, max) => {
				render_progress(cur, max);
			}
			PlayerState::Healing(cur, max) => {
				render_progress(cur, max);
			}
		}
		canvas.set_draw_color(Color::MAGENTA);
		canvas.draw_line(player_facing_indicator_points.0, player_facing_indicator_points.1).unwrap();
	}
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerState{
	Standing,
	Running,
	Learning(ActionButton, u32, u32),
	MeleeAttacking(u32, u32),
	RangeTargeting,
	RangeAttacking(u32, u32),
	ButtonPressing(u32, u32),
	BuildChoosing,
	BuildPlacing(u32, u32),
	Repairing(u32, u32),
	Healing(u32, u32)
}

#[derive(Clone, Copy)]
pub enum Ability{
	Blank,
	MeleeAttack,
	Armor,
	RangeAttack,
	Vision,
	Build,
	Repair,
	ButtonPress,
	Heal
}

impl Ability{
	pub fn get_hud_text(&self) -> String{
		match self{
			Ability::Blank => String::from(""),
			Ability::MeleeAttack => String::from("Melee Attack"),
			Ability::Armor => String::from("Armor"),
			Ability::RangeAttack => String::from("Range Attack"),
			Ability::Vision => String::from("Vision"),
			Ability::Build => String::from("Build"),
			Ability::Repair => String::from("Repair"),
			Ability::ButtonPress => String::from("Press Button"),
			Ability::Heal => String::from("Heal"),
		}
	}
}