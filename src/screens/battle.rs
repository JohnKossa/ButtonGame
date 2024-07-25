use std::collections::HashSet;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use crate::game_context::{GameContext, GameObject};
use crate::input::{get_player_intent_vector, InputState};
use crate::sound_manager::SoundManager;
use crate::battle_objects::button::Button;
use crate::battle_objects::other_player::OtherPlayer;
use crate::battle_objects::buildables::{Wall, Window};
use crate::battle_objects::enemy::Enemy;
use crate::battle_objects::projectiles::FriendlyProjectile;
use crate::battle_objects::coordinates::{Direction, GameCoord, GridCoord};
use crate::battle_objects::ability_plots::AbilityPlot;
use crate::battle_objects::battle_player::{BattlePlayerContext, PlayerState};
use crate::battle_objects::camera::CameraState;
use crate::battle_objects::hud::Hud;
use crate::battle_objects::battle_player::Ability::{Armor, Blank, Build, ButtonPress, Heal, MeleeAttack, RangeAttack, Repair, Vision};
use crate::utils::collisions::line_to_line_intersect;

#[derive(Clone)]
pub enum BattleState{
	Starting,
	Live,
	Finished
}

pub struct BattleContext{
	pub state: BattleState,
	pub player: BattlePlayerContext,
	pub round_time: u32, //frame count for the battle context
	pub camera_state: CameraState,
	pub button: Button,
	pub other_players: Vec<OtherPlayer>,
	pub walls:  Vec<Wall>,
	pub windows: Vec<Window>,
	pub friendly_projectiles: Vec<FriendlyProjectile>,
	pub enemies: Vec<Enemy>,
	pub ability_plots : Vec<AbilityPlot>
}

impl BattleContext{
	pub fn new() -> BattleContext{
		BattleContext{
			state: BattleState::Starting,//TODO change this to starting once we have state transitions
			round_time: 0,
			player: BattlePlayerContext{
				facing_vector: 0.0,
				state: PlayerState::Standing,
				base_vision_range: 5,
				ability_primary: Blank,
				ability_secondary: Blank,
				game_coord: GameCoord{ x:150, y:-150 },
				snapped_facing_vector: Direction::East
			},
			button: Button::new(),
			camera_state: CameraState::new(),
			walls: Vec::new(),
			windows: Vec::new(),
			other_players: Vec::new(),
			friendly_projectiles: Vec::new(),
			enemies: Vec::new(),
			ability_plots: vec![
				AbilityPlot{pos:GridCoord{x:-1, y:-2}, ability:MeleeAttack},
				AbilityPlot{pos:GridCoord{x:1,  y:-2}, ability:Armor},
				AbilityPlot{pos:GridCoord{x:2,  y:-1}, ability:RangeAttack},
				AbilityPlot{pos:GridCoord{x:2,  y:1}, ability:Vision},
				AbilityPlot{pos:GridCoord{x:1,  y:2}, ability:Build},
				AbilityPlot{pos:GridCoord{x:-1, y:2}, ability:Repair},
				AbilityPlot{pos:GridCoord{x:-2, y:-1}, ability:ButtonPress},
				AbilityPlot{pos:GridCoord{x:-2, y:1}, ability:Heal}
			],
		}
	}
	pub fn from_game_object(_game_object: &GameObject) -> BattleContext{
		BattleContext::new()
		// TODO hydrate from save state
	}

	pub fn get_visible_squares(&self) -> HashSet<GridCoord>{
		let player = &self.player;
		let player_square = player.game_coord.to_grid_coord();
		let center_square = GridCoord{x:0, y:0};
		let mut visible_squares = vec![center_square];
		for y in 1..=player.get_vision_range() as i32{
			visible_squares.push(center_square.to_north(y));
			for x in 1..=y{
				visible_squares.push(center_square.offset((-x, -y)));
				visible_squares.push(center_square.offset((x, -y)));
			}
		}

		//add all the other squares directly adjacent of diagonal to the center square
		visible_squares.push(center_square.to_west(1));
		visible_squares.push(center_square.to_east(1));
		visible_squares.push(center_square.to_south(1));
		visible_squares.push(center_square.offset((-1, 1)));
		visible_squares.push(center_square.offset((1, 1)));

		match player.snapped_facing_vector{
			Direction::North => (),
			Direction::South => {
				//flip all the y values
				visible_squares = visible_squares.iter().map(|coord| GridCoord{x: coord.x, y: -coord.y}).collect();
			},
			Direction::West => {
				//y becomes x, x becomes y
				visible_squares = visible_squares.iter().map(|coord| GridCoord{x: coord.y, y: coord.x}).collect();
			},
			Direction::East => {
				//y becomes -x, x becomes y
				visible_squares = visible_squares.iter().map(|coord| GridCoord{x: -coord.y, y: coord.x}).collect();
			}
		}
		//add player grid coordinates to all the visible squares
		visible_squares =  visible_squares
				.iter()
				.map(|coord| GridCoord{x: coord.x + player_square.x, y: coord.y + player_square.y})
				.collect();

		//get all walls that border the visible squares
		let all_corners = visible_squares
				.iter()
				.flat_map(|coord| vec![
					coord.top_left(),
					coord.top_right(),
					coord.bottom_left(),
					coord.bottom_right()
				])
				.collect::<HashSet<GameCoord>>();
		let relevant_walls = self.walls.iter().filter(|wall| {
			let wall_corners = vec![wall.endpoints.0, wall.endpoints.1];
			wall_corners.iter().any(|corner| all_corners.contains(corner))
		});
		let start_point:(i32, i32) = match player.snapped_facing_vector{
			Direction::North => (player_square.center().x, player_square.center().y - GridCoord::grid_size()*0.4 as i32),
			Direction::South => (player_square.center().x, player_square.center().y + GridCoord::grid_size()*0.4 as i32),
			Direction::West => (player_square.center().x - GridCoord::grid_size()*0.4 as i32, player_square.center().y),
			Direction::East => (player_square.center().x + GridCoord::grid_size()*0.4 as i32, player_square.center().y),
		};
		let to_return: HashSet<GridCoord> =  visible_squares
			.into_iter()
			.filter(|square|{
				for wall in relevant_walls.clone() {
					let wall_endpoints = ((wall.endpoints.0.x, wall.endpoints.0.y),(wall.endpoints.1.x, wall.endpoints.1.y));
					let square_coords = (square.center().x, square.center().y);
					if line_to_line_intersect((start_point, square_coords), wall_endpoints){
						return false;
					}
				}
				return true;
			}).collect();
		to_return
	}

	pub fn get_learning_time(&self) -> u32{
		30
	}

	pub fn handle_tick(game_obj: &mut GameObject, input_state: &InputState, my_sound_manager: &mut SoundManager){
		let GameContext::Battle(battle_context) = &mut game_obj.phase else {unreachable!("Game object is not in Battle phase")};
		battle_context.round_time += 1;
		let learning_timer = battle_context.get_learning_time();
		let battle_player = &mut battle_context.player;

		//check collisions
		let player_grid_corner_coords = [
			battle_player.game_coord.to_grid_coord().top_left(),
			battle_player.game_coord.to_grid_coord().top_right(),
			battle_player.game_coord.to_grid_coord().bottom_right(),
			battle_player.game_coord.to_grid_coord().bottom_left()
		];
		//find wall above player if one exists
		let top_wall: Option<&Wall> = battle_context.walls.iter().find(|wall| {
			let wall_corners = (wall.endpoints.0, wall.endpoints.1);
			return (player_grid_corner_coords[0] == wall_corners.0 && player_grid_corner_coords[1] == wall_corners.1) || (player_grid_corner_coords[0] == wall_corners.1 && player_grid_corner_coords[1] == wall_corners.0);
		});
		let right_wall: Option<&Wall> = battle_context.walls.iter().find(|wall| {
			let wall_corners = (wall.endpoints.0, wall.endpoints.1);
			return (player_grid_corner_coords[1] == wall_corners.0 && player_grid_corner_coords[2] == wall_corners.1) || (player_grid_corner_coords[1] == wall_corners.1 && player_grid_corner_coords[2] == wall_corners.0);
		});
		let bottom_wall: Option<&Wall> = battle_context.walls.iter().find(|wall| {
			let wall_corners = (wall.endpoints.0, wall.endpoints.1);
			return (player_grid_corner_coords[2] == wall_corners.0 && player_grid_corner_coords[3] == wall_corners.1) || (player_grid_corner_coords[2] == wall_corners.1 && player_grid_corner_coords[3] == wall_corners.0);
		});
		let left_wall: Option<&Wall> = battle_context.walls.iter().find(|wall| {
			let wall_corners = (wall.endpoints.0, wall.endpoints.1);
			return (player_grid_corner_coords[3] == wall_corners.0 && player_grid_corner_coords[0] == wall_corners.1) || (player_grid_corner_coords[3] == wall_corners.1 && player_grid_corner_coords[0] == wall_corners.0);
		});
		let player_collisions = battle_player.get_collisions(top_wall, right_wall, bottom_wall, left_wall);
		if player_collisions.0{
			//snap player to bottom of top wall
			battle_player.game_coord.y = top_wall.unwrap().endpoints.0.y + GridCoord::grid_size()/2;
		}
		if player_collisions.1{
			//snap player to left of right wall
			battle_player.game_coord.x = right_wall.unwrap().endpoints.0.x - GridCoord::grid_size()/2;
		}
		if player_collisions.2{
			//snap player to top of bottom wall
			battle_player.game_coord.y = bottom_wall.unwrap().endpoints.0.y - GridCoord::grid_size()/2;
		}
		if player_collisions.3{
			//snap player to right of left wall
			battle_player.game_coord.x = left_wall.unwrap().endpoints.0.x + GridCoord::grid_size()/2;
		}

		match battle_context.state {
			BattleState::Starting => {
				battle_context.state = BattleState::Live;
				//load and start the music loop
				my_sound_manager.register_file("battle-bg", String::from("assets/sounds/Cool-Adventure-Intro.mp3"));
				my_sound_manager.play_registered_looping("bg", "battle-bg").set_volume(0.2);
			},
			BattleState::Live => {
				//TODO check for received moves
				//TODO update world
				battle_context.camera_state.smooth_scroll(&battle_player.game_coord);
				battle_context.button.update();
				match (&battle_player.state, get_player_intent_vector(input_state), &input_state.btn_down, &input_state.btn_right){
					(PlayerState::Standing, None, false, false) => (),
					(PlayerState::Standing, Some(x), false, false) => {
						battle_player.facing_vector = x;
						battle_player.snapped_facing_vector = Direction::from_facing_vector(x);
						const RUNNING_SPEED: f32 = BattlePlayerContext::running_speed();
						battle_player.game_coord.y -= (battle_player.facing_vector.sin() * RUNNING_SPEED) as i32;
						battle_player.game_coord.x += (battle_player.facing_vector.cos() * RUNNING_SPEED) as i32;
						battle_player.state = PlayerState::Running;
					},
					(PlayerState::Standing,_, true, false) =>{
						//if players are standing in a learning zone, switch to learning state
						let player_grid = battle_player.game_coord.to_grid_coord();
						let player_in_plot = battle_context.ability_plots.iter().find(|plot| plot.pos == player_grid);
						if let Some(_) = player_in_plot {
							battle_player.state = PlayerState::Learning(ActionButton::Primary, 0, learning_timer)
						}else {
							//otherwise, activate the ability assigned to primary
							battle_player.state = match battle_player.ability_primary {
								Blank => PlayerState::Standing,
								MeleeAttack => PlayerState::MeleeAttacking(0, 25),
								Armor => PlayerState::Standing,
								RangeAttack => PlayerState::RangeTargeting,
								Vision => PlayerState::Standing,
								Build => PlayerState::BuildPlacing(0, 25),
								Repair => PlayerState::Repairing(0, 25),
								ButtonPress => PlayerState::ButtonPressing(0, 25),
								Heal => PlayerState::Healing(0, 25)
							}
						}
					},
					(PlayerState::Standing,_, false, true) =>{
						//if players are standing in a learning zone, switch to learning state
						let player_grid = battle_player.game_coord.to_grid_coord();
						let player_in_plot = battle_context.ability_plots.iter().find(|plot| plot.pos == player_grid);
						if let Some(_) = player_in_plot {
							battle_player.state = PlayerState::Learning(ActionButton::Secondary, 0, learning_timer)
						}else{
							//otherwise, activate the ability assigned to primary
							battle_player.state = match battle_player.ability_secondary {
								Blank => PlayerState::Standing,
								MeleeAttack => PlayerState::MeleeAttacking(0,25),
								Armor => PlayerState::Standing,
								RangeAttack => PlayerState::RangeTargeting,
								Vision => PlayerState::Standing,
								Build => PlayerState::BuildPlacing(0, 25),
								Repair => PlayerState::Repairing(0, 25),
								ButtonPress => PlayerState::ButtonPressing(0, 25),
								Heal => PlayerState::Healing(0, 25)
							}
						}
					},
					(PlayerState::Standing, facing, primary, secondary) => {
						println!("Player state: {:?}, direction: {:?}, primary: {}, secondary: {}", battle_player.state, facing, primary, secondary);
						todo!("Button combo for standing not implemented")
					},
					(PlayerState::Running, Some(x), false, false) => {
						//still running
						battle_player.facing_vector = x;
						battle_player.snapped_facing_vector = Direction::from_facing_vector(x);
						const RUNNING_SPEED: f32 = BattlePlayerContext::running_speed();
						battle_player.game_coord.x += (battle_player.facing_vector.cos() * RUNNING_SPEED) as i32;
						battle_player.game_coord.y -= (battle_player.facing_vector.sin() * RUNNING_SPEED) as i32;
					},
					(PlayerState::Running, _, true, false) =>{
						battle_player.state = PlayerState::Learning(ActionButton::Primary, 0, learning_timer)
					},
					(PlayerState::Running, _, false, true) =>{
						battle_player.state = PlayerState::Learning(ActionButton::Secondary, 0, learning_timer)
					},
					(PlayerState::Running, None, _, _) =>{
						battle_player.state = PlayerState::Standing;
					},
					(PlayerState::Running, facing, primary, secondary) =>{
						println!("Player state: {:?}, direction: {:?}, primary: {}, secondary: {}", battle_player.state, facing, primary, secondary);
						todo!("Button combo for running not implemented")
					},
					(PlayerState::Learning(_,_,_),_,false,false) =>{
						battle_player.state = PlayerState::Standing;
					},
					(PlayerState::Learning(ActionButton::Primary, curr, max), _, true, false) if curr < max => {
						//if player is standing in a learning zone
						let player_grid = battle_player.game_coord.to_grid_coord();
						let player_in_plot = battle_context.ability_plots.iter().find(|plot| plot.pos == player_grid);
						if let Some(_) = player_in_plot {
							battle_player.state = PlayerState::Learning(ActionButton::Primary, curr+1, *max);
						}else{
							battle_player.state = PlayerState::Standing;
						}
					},
					(PlayerState::Learning(ActionButton::Primary, curr, max), _, true, false) if curr >= max => {
						let player_grid_square = battle_player.game_coord.to_grid_coord();
						let active_plot = battle_context.ability_plots.iter()
								.find(|plot| plot.pos == player_grid_square);
						if let Some(plot) = active_plot {
							battle_player.ability_primary = plot.ability;
						}
						battle_player.state = PlayerState::Standing;
					},
					(PlayerState::Learning(ActionButton::Secondary, curr, max), _, false, true) if curr < max => {
						let player_grid = battle_player.game_coord.to_grid_coord();
						let player_in_plot = battle_context.ability_plots.iter().find(|plot| plot.pos == player_grid);
						if let Some(_) = player_in_plot {
							battle_player.state = PlayerState::Learning(ActionButton::Secondary, curr+1, *max);
						}else{
							battle_player.state = PlayerState::Standing;
						}
					}
					(PlayerState::Learning(ActionButton::Secondary, curr, max), _, false, true) if curr >= max => {
						let player_grid_square = battle_player.game_coord.to_grid_coord();
						let active_plot = battle_context.ability_plots.iter()
								.find(|plot| plot.pos == player_grid_square);
						if let Some(plot) = active_plot {
							battle_player.ability_secondary = plot.ability;
						}
						battle_player.state = PlayerState::Standing;
					},
					(PlayerState::Learning(_,_,_),facing, primary, secondary) => {
						println!("Player state: {:?}, direction: {:?}, primary: {}, secondary: {}", battle_player.state, facing, primary, secondary);
						todo!("Button combo for learning not implemented")
					},
					(PlayerState::BuildPlacing(_,_), None, false, false) => {
						battle_player.state = PlayerState::Standing;
					},
					(PlayerState::BuildPlacing(_,_), Some(angle), false, false) => {
						battle_player.state = PlayerState::Running;
						battle_player.facing_vector = angle;
						battle_player.snapped_facing_vector = Direction::from_facing_vector(angle);
					},
					(PlayerState::BuildPlacing(curr, max), facing, true, false) if curr < max => {
						if let Some(x) = facing {
							battle_player.snapped_facing_vector = Direction::from_facing_vector(x);
							battle_player.facing_vector = x;
						}
						battle_player.state = match battle_player.ability_primary {
							Build => PlayerState::BuildPlacing(curr+1, *max),
							_ => PlayerState::Standing
						};
					},
					(PlayerState::BuildPlacing(curr, max), _, true, false) if curr >= max => {
						if let Build = battle_player.ability_primary {
							let build_endpoints = match battle_player.snapped_facing_vector{
								Direction::North => (battle_player.game_coord.to_grid_coord().top_left(), battle_player.game_coord.to_grid_coord().top_right()),
								Direction::South => (battle_player.game_coord.to_grid_coord().bottom_left(), battle_player.game_coord.to_grid_coord().bottom_right()),
								Direction::West => (battle_player.game_coord.to_grid_coord().top_left(), battle_player.game_coord.to_grid_coord().bottom_left()),
								Direction::East => (battle_player.game_coord.to_grid_coord().top_right(), battle_player.game_coord.to_grid_coord().bottom_right())
							};
							let new_wall = Wall { endpoints: build_endpoints, health: (100, 100) };
							if battle_context.walls.iter().all(|wall| wall.endpoints != new_wall.endpoints){
								battle_context.walls.push(new_wall);
							}
						}
						battle_player.state = PlayerState::Standing;
					},
					(PlayerState::BuildPlacing(curr, max), facing, false, true) if curr < max => {
						if let Some(x) = facing {
							battle_player.snapped_facing_vector = Direction::from_facing_vector(x);
							battle_player.facing_vector = x;
						}
						battle_player.state = match battle_player.ability_secondary {
							Build => PlayerState::BuildPlacing(curr+1, *max),
							_ => PlayerState::Standing
						};
					},
					(PlayerState::BuildPlacing(curr, max), _, false, true) if curr >= max => {
						if let Build = battle_player.ability_secondary {
							let build_endpoints = match battle_player.snapped_facing_vector{
								Direction::North => (battle_player.game_coord.to_grid_coord().top_left(), battle_player.game_coord.to_grid_coord().top_right()),
								Direction::South => (battle_player.game_coord.to_grid_coord().bottom_left(), battle_player.game_coord.to_grid_coord().bottom_right()),
								Direction::West => (battle_player.game_coord.to_grid_coord().top_left(), battle_player.game_coord.to_grid_coord().bottom_left()),
								Direction::East => (battle_player.game_coord.to_grid_coord().top_right(), battle_player.game_coord.to_grid_coord().bottom_right())
							};
							let new_wall = Wall { endpoints: build_endpoints, health: (100, 100) };
							if battle_context.walls.iter().all(|wall| wall.endpoints != new_wall.endpoints){
								battle_context.walls.push(new_wall);
							}
						}
						battle_player.state = PlayerState::Standing;
					},
					(PlayerState::BuildPlacing(_, _), facing, primary, secondary) => {
						println!("Player state: {:?}, direction: {:?}, primary: {}, secondary: {}", battle_player.state, facing, primary, secondary);
						todo!("Button combo for build placing not implemented")
					},
					(PlayerState::MeleeAttacking(_,_), _, _, _) => {
						battle_player.state = PlayerState::Standing;
						//TODO implement
					},
					(PlayerState::RangeTargeting, _, _, _) =>{
						battle_player.state = PlayerState::Standing;
						//TODO implement
					},
					(PlayerState::Healing(_,_), _, _, _) =>{
						battle_player.state = PlayerState::Standing;
						//TODO implement
					},
					(PlayerState::Repairing(_,_), _, _, _) =>{
						battle_player.state = PlayerState::Standing;
						//TODO implement
					},
					(PlayerState::ButtonPressing(_, _), _, _, _) =>{
						battle_player.state = PlayerState::Standing;
						//TODO implement
					},
					(s,d,a,b)=>{
						println!("Not Implemented: Player state: {:?}, direction: {:?}, a: {}, b: {}", s, d, a, b);
						//todo!("Not implemented")
					},
				}
				//TODO broadcast moves
			},
			BattleState::Finished => (),
		};

	}
}

#[derive(Clone, Copy, Debug)]
pub enum ActionButton {
	Primary,
	Secondary
}

pub fn draw_grid(canvas: &mut WindowCanvas, _background_texture: &Texture, ctx: &BattleContext){
	//starting from the camera position, get the grid square, get the top left corner, keep drawin vertical lines to the left and right until we've drawn 3/4 the width of the screen each direction
	//keep drawing horizontal lines to the top and botton until we've drawn 3/4 of the height of the screen
	let camera = &ctx.camera_state;
	let canvas_dimensions = canvas.output_size().unwrap();
	canvas.set_draw_color(Color::RGB(32,32,32));
	let start_point = camera.pos.to_grid_coord().top_left().to_display_coord(camera.pos, camera.scale, canvas_dimensions);
	let grid_width = GridCoord::grid_size();
	let mut x_offset = 0;
	while x_offset < (2 * canvas_dimensions.0 ){
		//draw a vertical line that's 120% of the height
		canvas.draw_line(Point::new(start_point.x + x_offset as i32, 0), Point::new(start_point.x + x_offset as i32, canvas_dimensions.1 as i32)).unwrap();
		canvas.draw_line(Point::new(start_point.x - x_offset as i32, 0), Point::new(start_point.x - x_offset as i32, canvas_dimensions.1 as i32)).unwrap();
		x_offset = x_offset + (grid_width as f32 * camera.scale) as u32;
	}
	let mut y_offset = 0;
	while y_offset < (2 * canvas_dimensions.1){
		canvas.draw_line(Point::new(0, start_point.y + y_offset as i32), Point::new(canvas_dimensions.0 as i32, start_point.y + y_offset as i32)).unwrap();
		canvas.draw_line(Point::new(0, start_point.y - y_offset as i32), Point::new(canvas_dimensions.0 as i32, start_point.y - y_offset as i32)).unwrap();
		y_offset = y_offset + (grid_width as f32 * camera.scale) as u32;
	}
}

pub trait BattleRenderable{
	fn render(&self, canvas: &mut WindowCanvas, background_texture: &Texture, ctx: &BattleContext);
}

pub fn render_battle(canvas: &mut WindowCanvas, background_texture: &Texture, ctx: &BattleContext){
	let canvas_size = canvas.output_size().unwrap();
	canvas.clear();
	canvas.set_draw_color(Color::RGB(0,0,16));
	canvas.fill_rect(Rect::new(0,0, canvas_size.0, canvas_size.1)).unwrap();
	//canvas.copy(background_texture, None, None).expect("Couldn't draw background texture.");
	draw_grid(canvas, background_texture, ctx);
	ctx.button.render(canvas, background_texture, ctx);
	for ability_plot in &ctx.ability_plots{
		ability_plot.render(canvas, background_texture, ctx);
	}
	for wall in &ctx.walls{
		wall.render(canvas, background_texture, ctx);
	}
	ctx.player.render(canvas, background_texture, ctx);
	for visible_grid_square in ctx.get_visible_squares(){
		canvas.set_draw_color(Color::RGB(32, 32, 32));
		canvas.fill_rect(
			Rect::from_center(
				visible_grid_square.center().to_display_coord(
					ctx.camera_state.pos,
					ctx.camera_state.scale,
					canvas_size
				),
				(GridCoord::grid_size() as f32 * 0.6) as u32,
				(GridCoord::grid_size() as f32 * 0.6) as u32
			)
		).unwrap();
	}
	Hud::from_player(&ctx.player).render(canvas, background_texture, ctx);
	canvas.present();
}