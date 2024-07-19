extern crate sdl2;
extern crate gl;
mod input;
mod screens;
mod sound_manager;
mod game_context;
mod battle_objects;
mod utils;

use std::time::{Duration, Instant};
use sdl2::image::{LoadTexture};

use input::{InputState, ControllerSettings, read_input_event};
use sound_manager::SoundManager;
use game_context::{GameContext, GameObject, Player};
use crate::screens::start::StartScreenContext;

static TICK_RATE: u32 = 30;

fn main() {
	let sdl_context = sdl2::init().expect("Unable to create sdl context");
	let controller_subsystem = sdl_context
		.game_controller()
		.expect("Unable to create game controller subsystem");
	let video_subsystem = sdl_context.video().expect("Unable to initialize sdl video context");
	let mut my_sound_manager = SoundManager::new();
	let _controller = controller_subsystem.open(0);
	let controller_settings = ControllerSettings::new();
	let window = video_subsystem.window("Button Game", 1080, 720)
		.position_centered()
		.build()
		.expect("Failed to create window");
	let mut canvas = window.into_canvas()
		.software()
		.build()
		.expect("Failed to create canvas from window");

	let texture_creator = canvas.texture_creator();

	let background_texture = texture_creator.load_texture("assets/images/button_game_splash_art.png")
		.expect("Unable to create background texture.");

	let mut events = sdl_context.event_pump()
		.expect("Unable to initialize sdl event pump");
	let target_frame_duration = Duration::from_millis(1000 / TICK_RATE as u64);
	let mut input_state = InputState::new();

	let mut game_obj = GameObject{
		phase: GameContext::StartScreen(StartScreenContext::new()),
		player: Some(Player{})

	};

	'mainloop: loop {
		let frame_start = Instant::now();
		for event in events.poll_iter() {
			read_input_event(&mut input_state, &controller_settings, &event);
		}
		//println!("{:?}", input_state);
		if input_state.shutdown{
			break 'mainloop;
		}

		game_obj.handle_tick(&input_state, &mut my_sound_manager);
		game_obj.render(&mut canvas, &background_texture);

		// Sleep if we finished this frame early, so we lock to the desired framerate
		let frame_duration = frame_start.elapsed();
		if let Some(remaining_duration) = target_frame_duration.checked_sub(frame_duration) {
			std::thread::sleep(remaining_duration);
		} else {
			println!("Dropped framerate. Frame duration: {:?}, Target: {:?}", frame_duration, target_frame_duration);
		}
	}
}
