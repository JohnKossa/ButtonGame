use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::battle_objects::coordinates::GridCoord;
use crate::screens::battle::{BattleContext, BattleRenderable};

pub enum ButtonState{
	NeverPressed,
	Pressed(usize, usize),
	Unpressed(usize, usize),
}

pub struct Button{
	pub pos: GridCoord,
	pub state: ButtonState
}

impl Button {
	pub fn new() -> Button {
		Button {pos: GridCoord{x:0, y:0, grid_size: 20}, state: ButtonState::NeverPressed}
	}
	pub fn update(&mut self){
		self.state = match self.state{
			ButtonState::NeverPressed => ButtonState::NeverPressed,
			ButtonState::Pressed(curr, max) => ButtonState::Pressed(curr+1, max),
			ButtonState::Unpressed(curr, max) => ButtonState::Unpressed(curr+1, max),
		}
	}
}

impl BattleRenderable for Button{
	fn render(&self, canvas: &mut WindowCanvas, background_texture: &Texture, ctx: &BattleContext) {
		let camera = &ctx.camera_state;
		let button_rect = Rect::from_center(
			self.pos.center().to_display_coord(
				camera.pos,
				camera.scale,
				canvas.output_size().unwrap()),
			(camera.scale * 16.0) as u32,
			(camera.scale * 16.0) as u32);
		let button_color = Color::RGB(255, 128, 0);
		canvas.set_draw_color(button_color);
		canvas.fill_rect(button_rect).unwrap();
	}
}