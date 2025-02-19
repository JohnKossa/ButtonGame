use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::render::{WindowCanvas, Texture, BlendMode};
use crate::game_context::{GameContext, GameObject};
use crate::input::{InputState};
use crate::screens::battle::BattleContext;
use crate::sound_manager::SoundManager;
use crate::utils::render_utils::render_text;

#[derive(Clone, Copy)]
pub enum StartScreenState{
	FadeIn(usize, usize),
	Waiting,
	FadeOut(usize, usize)
}

#[derive(Clone, Copy)]
pub struct StartScreenContext{
	pub state: StartScreenState
}

impl StartScreenContext{
	pub fn new() -> StartScreenContext{
		StartScreenContext{
			state: StartScreenState::FadeIn(0,30)
		}
	}

	pub fn handle_tick(game_obj: &mut GameObject, input_state: &InputState, _sound_manager: &mut SoundManager){
		let GameContext::StartScreen(start_screen_context) = game_obj.phase else { unreachable!("Game object is not in StartScreen phase") };
		match start_screen_context.state {
			StartScreenState::FadeIn(a, b) if a == b => {
				game_obj.phase = GameContext::StartScreen(StartScreenContext{state: StartScreenState::Waiting})
			},
			StartScreenState::FadeIn(a, b) if a > b => unreachable!("Frame count above maximum"),
			StartScreenState::FadeIn(a,b) => {
				game_obj.phase = GameContext::StartScreen(StartScreenContext{state: StartScreenState::FadeIn(a+1, b)})
			},
			StartScreenState::Waiting => match input_state.btn_start{
				true => {
					game_obj.phase = GameContext::StartScreen(StartScreenContext{state: StartScreenState::FadeOut(0,30)})
				},
				false => (),
			},
			StartScreenState::FadeOut(a,b) if a==b => {
				game_obj.phase = GameContext::Battle(BattleContext::from_game_object(game_obj))
			},
			StartScreenState::FadeOut(a, b) if a > b => unreachable!("Frame count above maximum"),
			StartScreenState::FadeOut(a, b) => {
				game_obj.phase = GameContext::StartScreen(StartScreenContext{state: StartScreenState::FadeOut(a+1,b)})
			}
		}
	}
}

pub fn render_start_screen(canvas: &mut WindowCanvas, background_texture: &Texture, context: &StartScreenContext){
	let (width, height) = canvas.output_size().unwrap();
	canvas.clear();
	canvas.copy(background_texture, None, None).expect("Couldn't draw background texture.");

	let ttf_context = sdl2::ttf::init().unwrap();

	render_text(
		canvas,
		&ttf_context,
		"Button Game",
		128,
		Color::YELLOW,
		Rect::new(50, 100, (width as f32 *0.8) as u32, (height as f32 *0.25) as u32)
	);

	// Render the text into a surface.
	render_text(
		canvas,
		&ttf_context,
		"Survival",
		64,
		Color::YELLOW,
		Rect::new(50, 250, 400, 100)
	);

	render_text(
		canvas,
		&ttf_context,
		"Press Start",
		64,
		Color::WHITE,
		Rect::new(((width / 2)-100) as i32, ((height/2)+50) as i32, 200, 100)
	);

	match context.state{
		StartScreenState::Waiting => (),
		StartScreenState::FadeIn(frame_num, frame_max) =>{
			canvas.set_blend_mode(BlendMode::Blend);
			let opacity: u8 = (255 * (frame_max - frame_num) / frame_max) as u8;
			canvas.set_draw_color((0, 0, 0, opacity));
			canvas.fill_rect(Rect::new(0, 0, width, height)).expect("Failed to draw a rectangle");
			canvas.set_blend_mode(BlendMode::None);//put the blend mode back to normal
		},
		StartScreenState::FadeOut(frame_num, frame_max) =>{
			canvas.set_blend_mode(BlendMode::Blend);
			let opacity: u8 = (255 * frame_num / frame_max) as u8;
			canvas.set_draw_color((255,255,255,opacity));
			canvas.fill_rect(Rect::new(0, 0, width, height)).expect("Failed to draw a rectangle");
			canvas.set_blend_mode(BlendMode::None);//put the blend mode back to normal
		},
	}

	canvas.set_blend_mode(BlendMode::None);//put the blend mode back to normal
	canvas.present();
}