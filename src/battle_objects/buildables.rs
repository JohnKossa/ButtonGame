use sdl2::pixels::Color;
use crate::battle_objects::coordinates::{GameCoord};
use crate::screens::battle::BattleRenderable;

#[derive(Clone, Copy)]
pub struct Wall{
	//walls are corner-aligned. To convert them to game coordinates, default to the top left
	pub endpoints: (GameCoord, GameCoord),
	pub health: (usize, usize),
}

#[derive(Clone, Copy)]
pub struct Window{
	//windows are corner-aligned. To convert them to game coordinates, default to the top left
	pub endpoints: (GameCoord, GameCoord),
	pub health: (usize, usize),
}

impl BattleRenderable for Wall{
	fn render(&self, canvas: &mut sdl2::render::WindowCanvas, _background_texture: &sdl2::render::Texture, ctx: &crate::screens::battle::BattleContext){
		//set color to green if health is full, red if health is 0, yellow if health is in between
		let camera_coord = ctx.camera_state.pos;
		let camera_scale = ctx.camera_state.scale;
		let draw_color = match self.health.0 as f32 / self.health.1 as f32{
			x if x > 0.00 && x<=0.25 => Color::RED,
			x if x > 0.25 && x<=0.75 => Color::YELLOW,
			x if x > 0.75 && x<=1.0 => Color::GREEN,
			_ => unreachable!("Health ratio outside of 0 to 1 range")
		};
		canvas.set_draw_color(draw_color);
		canvas.draw_line(
			self.endpoints.0.to_display_coord(camera_coord, camera_scale ,canvas.output_size().unwrap()),
			self.endpoints.1.to_display_coord(camera_coord, camera_scale, canvas.output_size().unwrap())
		).unwrap();
	}
}

impl BattleRenderable for Window{
	fn render(&self, _canvas: &mut sdl2::render::WindowCanvas, _background_texture: &sdl2::render::Texture, _ctx: &crate::screens::battle::BattleContext){
		todo!("Implement window rendering")
	}
}