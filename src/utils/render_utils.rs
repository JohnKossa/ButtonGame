use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::ttf::Sdl2TtfContext;

pub fn render_progress_bar(canvas: &mut sdl2::render::WindowCanvas, x: i32, y: i32, width: u32, height: u32, progress: (usize, usize)){
	let progress_pct = progress.0 as f32 / progress.1 as f32;
	let progress_pixels = (height as f32 * progress_pct) as u32;
	let progress_rectangle = Rect::new(
		x,
		y + height as i32 - progress_pixels as i32,
		width,
		progress_pixels
	);
	canvas.set_draw_color(Color::WHITE);
	canvas.fill_rect(progress_rectangle).unwrap();
}

pub fn render_text(canvas: &mut sdl2::render::WindowCanvas, ttf_context: &Sdl2TtfContext, text: &str, font_size: u16, color: Color, target: Rect){
	let font = ttf_context.load_font("assets/fonts/The_Frontman.ttf", font_size).unwrap();
	let surface = font.render(text)
		.blended(color)
		.unwrap();
	let texture_creator = canvas.texture_creator();
	let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
	canvas.copy(&texture, None, Some(target)).expect("Couldn't write start screen text.");
}