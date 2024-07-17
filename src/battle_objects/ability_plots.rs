use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use crate::battle_objects::coordinates::GridCoord;
use crate::screens::battle::{Ability, BattleContext, BattleRenderable};

pub struct AbilityPlot {
	pub(crate) pos: GridCoord,
	pub(crate) ability: Ability
}

impl BattleRenderable for AbilityPlot{
	fn render(
		&self,
		canvas: &mut WindowCanvas,
		background_texture: &Texture,
		ctx: &BattleContext
	){
		let camera_pos = ctx.camera_state.pos;
		let camera_scale = ctx.camera_state.scale;
		let plot_rect = Rect::from_center(
			self.pos.center().to_display_coord(
				camera_pos,
				camera_scale,
				canvas.output_size().unwrap()
			),
			(camera_scale*16.0) as u32,
			(camera_scale*16.0) as u32
		);
		canvas.set_draw_color(Color::RGBA(0, 128, 0, 64));
		canvas.fill_rect(plot_rect).unwrap();
	}
}