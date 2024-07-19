use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::screens::battle::{BattlePlayerContext, BattleRenderable};
use crate::utils::render_utils::render_text;

pub struct Hud{
	pub health: (usize, usize),
	pub ability_primary: String,
	pub ability_secondary: String,
}

impl Hud{
	pub fn from_player(&player: &BattlePlayerContext) -> Hud{
		Hud{
			health: (100,100),
			ability_primary: player.ability_primary.get_hud_text(),
			ability_secondary: player.ability_secondary.get_hud_text(),
		}
	}
}

impl BattleRenderable for Hud {
	fn render(&self, canvas: &mut sdl2::render::WindowCanvas, _background_texture: &sdl2::render::Texture, _ctx: &crate::screens::battle::BattleContext) {
		//render a health bar
		canvas.set_draw_color(Color::RGB(64, 64, 64));
		let health_bar_width = 300;
		canvas.fill_rect(Rect::new(10, 10, health_bar_width, 30)).unwrap();
		let fill_width = ((self.health.0 as f32 / self.health.1 as f32) * health_bar_width as f32) as u32;
		canvas.set_draw_color(Color::RED);
		canvas.fill_rect(Rect::new(10, 10, fill_width, 30)).unwrap();

		let ttf_context = sdl2::ttf::init().unwrap();
		if self.ability_primary.len()>0{
			render_text(canvas, &ttf_context, &self.ability_primary, 32, Color::WHITE, Rect::new(10, 50, 200, 50));
		}

		if self.ability_secondary.len()>0{
			render_text(canvas, &ttf_context, &self.ability_secondary, 32, Color::WHITE, Rect::new(10, 110, 200, 50));
		}
	}
}