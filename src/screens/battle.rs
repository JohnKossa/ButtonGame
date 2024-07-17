use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

use crate::game_context::{GameContext, GameObject};
use crate::input::{InputState, get_player_intent_vector};
use crate::sound_manager::SoundManager;
use crate::battle_objects::button::Button;
use crate::battle_objects::other_player::OtherPlayer;
use crate::battle_objects::buildables::{Wall, Window};
use crate::battle_objects::enemy::Enemy;
use crate::battle_objects::projectiles::FriendlyProjectile;
use crate::battle_objects::coordinates::{GridCoord, GameCoord, Direction};
use crate::battle_objects::ability_plots::AbilityPlot;
use crate::screens::battle::Ability::Blank;

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
                vision_range: 5,
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
                AbilityPlot{pos:GridCoord{x:-1, y:-2, grid_size:20}, ability:Ability::MeleeAttack},
                AbilityPlot{pos:GridCoord{x:1,  y:-2, grid_size:20}, ability:Ability::Armor},
                AbilityPlot{pos:GridCoord{x:2,  y:-1, grid_size:20}, ability:Ability::RangeAttack},
                AbilityPlot{pos:GridCoord{x:2,  y:1,  grid_size:20}, ability:Ability::Vision},
                AbilityPlot{pos:GridCoord{x:1,  y:2,  grid_size:20}, ability:Ability::Build},
                AbilityPlot{pos:GridCoord{x:-1, y:2,  grid_size:20}, ability:Ability::Repair},
                AbilityPlot{pos:GridCoord{x:-2, y:-1, grid_size:20}, ability:Ability::ButtonPress},
                AbilityPlot{pos:GridCoord{x:-2, y:1,  grid_size:20}, ability:Ability::Heal}
            ],
        }
    }
    pub fn from_game_object(game_object: &GameObject) -> BattleContext{
        BattleContext::new()
        // TODO hydrate from save state
    }

    pub fn get_visible_squares(&self) -> Vec<GridCoord>{
        //visible squares are a cone of squares projecting from the player and all other players as well as all squares directly adjacent to the player
        //if a square crosses a wall object, visibility stops projecting from that point
        let mut to_return = Vec::new();
        //player square is visible
        to_return.push(self.player.game_coord.to_grid_coord());
        //if there's no wall in the direction the player is facing
        //for the direction the player is facing, add each square in that direction until either a wall is found or the vision range is reached
        //

        to_return.push(self.player.game_coord.to_grid_coord());

        todo!("not implemented");
        Vec::new()
    }

    pub fn get_learning_time(&self) -> u32{
        30
    }
    pub fn handle_tick(game_obj: &mut GameObject, /*game_obj: &mut GameObject,*/ input_state: &InputState, my_sound_manager: &mut SoundManager){
        match game_obj.phase {
            GameContext::Battle(ref mut battle_context) =>{
                let learning_timer = battle_context.get_learning_time();
                let battle_player = &mut battle_context.player;
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
                        match (&battle_player.state, get_player_intent_vector(input_state), &input_state.btn_down, &input_state.btn_right){
                            (PlayerState::Standing, None, false, false) => (),
                            (PlayerState::Standing, Some(x), false, false) => {
                                battle_player.facing_vector = x;
                                battle_player.snapped_facing_vector = Direction::from_facing_vector(x);
                                const RUNNING_SPEED: f32 = 2.0;
                                battle_player.game_coord.x += (battle_player.facing_vector.cos() * RUNNING_SPEED) as i32;
                                battle_player.game_coord.y -= (battle_player.facing_vector.sin() * RUNNING_SPEED) as i32;
                                battle_player.state = PlayerState::Running;
                            },
                            (PlayerState::Standing,_, true, false) =>{
                                battle_player.state = PlayerState::Learning(ActionButton::Primary, 0, learning_timer)
                            },
                            (PlayerState::Standing,_, false, true) =>{
                                battle_player.state = PlayerState::Learning(ActionButton::Secondary, 0, learning_timer)
                            }
                            (PlayerState::Standing, _, _, _) => todo!("Button combo for standing not implemented"),
                            (PlayerState::Running, Some(x), false, false) => {
                                //still running
                                battle_player.facing_vector = x;
                                battle_player.snapped_facing_vector = Direction::from_facing_vector(x);
                                //println!("{:?} {:?}", player.snapped_facing_vector, player.facing_vector);
                                const RUNNING_SPEED: f32 = 2.0;
                                battle_player.game_coord.x += (battle_player.facing_vector.cos() * RUNNING_SPEED) as i32;
                                battle_player.game_coord.y -= (battle_player.facing_vector.sin() * RUNNING_SPEED) as i32;
                            },
                            (PlayerState::Running, None, f_, _) =>{
                                battle_player.state = PlayerState::Standing;
                            },
                            (PlayerState::Running, _,_,_) =>todo!("Button combo for running not implemented"),
                            (PlayerState::Learning(_,_,_),_,false,false) =>{
                                battle_player.state = PlayerState::Standing;
                            },
                            (PlayerState::Learning(ActionButton::Primary, curr, max), _, true, false) if curr < max => {
                                //if player is standing in a learning zone
                                let player_grid = battle_player.game_coord.to_grid_coord();
                                battle_player.state = PlayerState::Learning(ActionButton::Primary, curr+1, *max);
                                //else, player goes back to standing
                            },
                            (PlayerState::Learning(ActionButton::Primary, curr, max), _, true, false) if curr >= max => {
                                battle_player.state = PlayerState::Standing;

                                //TODO
                            },
                            (PlayerState::Learning(ActionButton::Secondary, curr, max), _, false, true) if curr < max => {
                                battle_player.state = PlayerState::Learning(ActionButton::Secondary, curr+1, *max)
                            }
                            (PlayerState::Learning(ActionButton::Secondary, curr, max), _, true, false) if curr >= max => {
                                battle_player.state = PlayerState::Standing;
                                //TODO
                            },
                            _=>todo!("Not implemented"),
                        }
                        update_battle_player(battle_player, &input_state, my_sound_manager, learning_timer);
                        //TODO broadcast moves
                    },
                    BattleState::Finished => (),
                }
            },
            _ => unreachable!("Should not be able to call handle_tick from start screen while not in battle phase.")
        };
    }
}

#[derive(Clone)]
pub struct CameraState{
    pub pos: GameCoord,
    pub scale: f32,
}

impl CameraState{
    pub fn new() -> CameraState{
        CameraState{pos: GameCoord{x:0, y:0}, scale:1.2}
    }

    pub fn smooth_scroll(&mut self, target: &GameCoord){
        let dx = target.x - self.pos.x;
        let dy = target.y - self.pos.y;
        self.pos.x = self.pos.x + (0.1 * dx as f32) as i32;
        self.pos.y = self.pos.y + (0.1 * dy as f32) as i32;
    }
}

#[derive(Clone, Copy)]
pub struct BattlePlayerContext{
    pub game_coord: GameCoord,
    pub facing_vector: f32,
    pub vision_range: u8,
    pub ability_primary: Ability,
    pub ability_secondary: Ability,
    pub snapped_facing_vector: Direction,
    pub state: PlayerState,
}

impl BattlePlayerContext{
    fn edge_coords(&self, width: u32, scale_factor: f32, center_point: GameCoord, window_dimensions: (u32, u32)) -> (Point, Point){
        let top_left = GameCoord{x: self.game_coord.x - width as i32/2, y: self.game_coord.y - width as i32/2};
        let top_right = GameCoord{x: self.game_coord.x + width as i32/2, y: self.game_coord.y - width as i32/2};
        let bottom_left = GameCoord{x: self.game_coord.x - width as i32/2, y: self.game_coord.y + width as i32/2};
        let bottom_right = GameCoord{x: self.game_coord.x + width as i32/2, y: self.game_coord.y + width as i32/2};
        match self.snapped_facing_vector {
            Direction::North => (
                top_left.to_display_coord(center_point, scale_factor, window_dimensions),
                top_right.to_display_coord(center_point, scale_factor, window_dimensions)
            ),
            Direction::South => (
                bottom_left.to_display_coord(center_point, scale_factor, window_dimensions),
                bottom_right.to_display_coord(center_point, scale_factor, window_dimensions)
            ),
            Direction::West => (
                top_left.to_display_coord(center_point, scale_factor, window_dimensions),
                bottom_left.to_display_coord(center_point, scale_factor, window_dimensions)
            ),
            Direction::East => (
                top_right.to_display_coord(center_point, scale_factor, window_dimensions),
                bottom_right.to_display_coord(center_point, scale_factor, window_dimensions)
            ),
        }
    }
}

impl BattleRenderable for BattlePlayerContext{
    fn render(&self, canvas: &mut WindowCanvas, background_texture: &Texture, ctx: &BattleContext){
        let player = &ctx.player;
        let camera = &ctx.camera_state;
        let canvas_size = canvas.output_size().unwrap();
        let player_rect = Rect::from_center(
            player.game_coord.to_display_coord(
                camera.pos,
                camera.scale,
                canvas.output_size().unwrap()),
            (camera.scale * 16.0) as u32,
            (camera.scale * 16.0) as u32);
        let player_facing_indicator_points = player.edge_coords(16, camera.scale, camera.pos, canvas_size);
        let player_color = match player.state{
            PlayerState::Standing => Color::RGB(0,255,0),
            PlayerState::Running => Color::RGB(255, 255, 0),
            _ => todo!()
        };
        canvas.set_draw_color(player_color);
        canvas.fill_rect(player_rect).unwrap();
        canvas.set_draw_color(Color::RGB(255,0,255));
        canvas.draw_line(player_facing_indicator_points.0, player_facing_indicator_points.1).unwrap();
    }
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

#[derive(Clone, Copy)]
pub enum ActionButton {
    Primary,
    Secondary
}

#[derive(Clone, Copy)]
pub enum PlayerState{
    Standing,
    Running,
    Learning(ActionButton, u32, u32),
    MeleeAttacking,
    RangeTargeting,
    RangeAttacking,
    ButtonPressing,
    BuildChoosing,
    BuildPlacing,
    Building,
    Healing
}

fn update_battle_player(player: &mut BattlePlayerContext, input: &InputState, sound_manager: &mut SoundManager, learning_time: u32){
    match (&player.state, get_player_intent_vector(input), &input.btn_down, &input.btn_right){
        (PlayerState::Standing, None, false, false) => (),
        (PlayerState::Standing, Some(x), false, false) => {
            player.facing_vector = x;
            player.snapped_facing_vector = Direction::from_facing_vector(x);
            const RUNNING_SPEED: f32 = 2.0;
            player.game_coord.x += (player.facing_vector.cos() * RUNNING_SPEED) as i32;
            player.game_coord.y -= (player.facing_vector.sin() * RUNNING_SPEED) as i32;
            player.state = PlayerState::Running;
        },
        (PlayerState::Standing,_, true, false) =>{
            player.state = PlayerState::Learning(ActionButton::Primary, 0, learning_time)
        },
        (PlayerState::Standing,_, false, true) =>{
            player.state = PlayerState::Learning(ActionButton::Secondary, 0, learning_time)
        }
        (PlayerState::Standing, _, _, _) => todo!("Button combo for standing not implemented"),
        (PlayerState::Running, Some(x), false, false) => {
            //still running
            player.facing_vector = x;
            player.snapped_facing_vector = Direction::from_facing_vector(x);
            //println!("{:?} {:?}", player.snapped_facing_vector, player.facing_vector);
            const RUNNING_SPEED: f32 = 2.0;
            player.game_coord.x += (player.facing_vector.cos() * RUNNING_SPEED) as i32;
            player.game_coord.y -= (player.facing_vector.sin() * RUNNING_SPEED) as i32;
        },
        (PlayerState::Running, None, f_, _) =>{
            player.state = PlayerState::Standing;
        },
        (PlayerState::Running, _,_,_) =>todo!("Button combo for running not implemented"),
        (PlayerState::Learning(_,_,_),_,false,false) =>{
            player.state = PlayerState::Standing;
        },
        (PlayerState::Learning(ActionButton::Primary, curr, max), _, true, false) if curr < max => {
            //if player is standing in a learning zone
            let player_grid = player.game_coord.to_grid_coord();
            player.state = PlayerState::Learning(ActionButton::Primary, curr+1, *max);
            //else, player goes back to standing
        },
        (PlayerState::Learning(ActionButton::Primary, curr, max), _, true, false) if curr >= max => {
            player.state = PlayerState::Standing;

            //TODO
        },
        (PlayerState::Learning(ActionButton::Secondary, curr, max), _, false, true) if curr < max => {
            player.state = PlayerState::Learning(ActionButton::Secondary, curr+1, *max)
        }
        (PlayerState::Learning(ActionButton::Secondary, curr, max), _, true, false) if curr >= max => {
            player.state = PlayerState::Standing;
            //TODO
        },
        _=>todo!("Not implemented"),

    }
    //println!("grid {:?}, game {:?}, gridcenter {:?}, gridtopleft {:?}", player.game_coord.to_grid_coord(), player.game_coord, player.game_coord.to_grid_coord().center(), player.game_coord.to_grid_coord().top_left())
}

pub fn draw_grid(canvas: &mut WindowCanvas, background_texture: &Texture, ctx: &BattleContext){
    //starting from the camera position, get the grid square, get the top left corner, keep drawin vertical lines to the left and right until we've drawn 3/4 the width of the screen each direction
    //keep drawing horizontal lines to the top and botton until we've drawn 3/4 of the height of the screen
    let camera = &ctx.camera_state;
    let canvas_dimensions = canvas.output_size().unwrap();
    canvas.set_draw_color(Color::RGB(64,64,64));
    //let start_point = camera.pos.to_grid_coord().top_left().to_grid_coord();
    let start_point = camera.pos.to_grid_coord().top_left().to_display_coord(camera.pos, camera.scale, canvas_dimensions);
    let grid_width = camera.pos.to_grid_coord().grid_size;
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
    ctx.player.render(canvas, background_texture, ctx);
    for ability_plot in &ctx.ability_plots{
        ability_plot.render(canvas, background_texture, ctx);
    }
    canvas.present();
}