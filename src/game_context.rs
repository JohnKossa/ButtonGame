use sdl2::render::{WindowCanvas, Texture};
use crate::game_context::GameContext::{Battle, StartScreen};
use crate::screens::start::render_start_screen;
use crate::screens::battle::{BattleContext, render_battle};
use crate::screens::start::StartScreenContext;
use crate::input::{InputState, get_player_intent_vector};
use crate::sound_manager::SoundManager;

pub struct GameObject{
    pub phase: GameContext,
    pub player: Option<Player>,
}

impl GameObject{
    pub fn handle_tick(&mut self, input_state: &InputState, my_sound_manager: &mut SoundManager){
        match &self.phase{
            Battle(_battle_context) =>{
                BattleContext::handle_tick(self, input_state, my_sound_manager);
            },
            StartScreen(_start_context) =>{
                StartScreenContext::handle_tick(self, input_state, my_sound_manager);
            }
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, background_texture: &Texture){
        match &self.phase{
            Battle(battle) => render_battle(canvas, background_texture, &battle),
            StartScreen(ctx) => render_start_screen(canvas, background_texture, &ctx),
            _ => todo!("Implement render for other phases")
        }
    }
}

pub enum GameContext{
    StartScreen(StartScreenContext),
    Battle(BattleContext),
}

#[derive(Clone, Copy)]
pub struct Player{
    //player stat things go here
}