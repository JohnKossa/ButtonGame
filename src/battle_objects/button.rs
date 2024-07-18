use sdl2::image::LoadTexture;
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
		Button {pos: GridCoord{x:0, y:0}, state: ButtonState::NeverPressed}
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
	fn render(&self, canvas: &mut WindowCanvas, _background_texture: &Texture, ctx: &BattleContext) {
		let texture_creator = canvas.texture_creator();
		let button_texture = match self.state {
			ButtonState::NeverPressed => texture_creator.load_texture("assets/images/hotel_bell_gray.png").unwrap(),
			ButtonState::Unpressed(_, _) => texture_creator.load_texture("assets/images/hotel_bell_gray.png").unwrap(),
			ButtonState::Pressed(_, _) => texture_creator.load_texture("assets/images/hotel_bell_yellow.png").unwrap(),
		};
		let camera = &ctx.camera_state;
		let display_rect_center = self.pos.center().to_display_coord(camera.pos, camera.scale, canvas.output_size().unwrap());
		let button_rect = Rect::from_center(
			display_rect_center,
			(camera.scale * 16.0) as u32,
			(camera.scale * 16.0) as u32);
		canvas.copy(&button_texture, None, Some(button_rect)).unwrap();
	}
}