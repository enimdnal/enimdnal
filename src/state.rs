mod defeat;
mod paused;
mod playing;
mod victory;

use notan::draw::*;
use notan::prelude::*;

use crate::drawing::ConfettiExplosion;
use crate::drawing::Explosion;
use crate::drawing::TILE_SIZE;
use crate::minefield::Board;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Stage {
    Playing,
    Paused,
    Victory,
    Defeat,
}

#[derive(AppState)]
pub struct State {
    stage: Stage,
    board: Board,
    last_coords: (usize, usize),
    hover: Option<(usize, usize)>,
    elapsed_milisec: u32,
    global_milisec: f32,
    font: Font,
    font_mono: Font,
    explosions: Vec<Explosion>,
    confetti_explosions: Vec<ConfettiExplosion>,

}

impl State {
    pub fn new(font: Font, font_mono: Font) -> Self {
        Self {
            stage: Stage::Playing,
            board: Board::expert(),
            hover: None,
            last_coords: (0, 0),
            elapsed_milisec: 0,
            global_milisec: 0.,
            font,
            font_mono,
            explosions:Vec::new(),
            confetti_explosions: Vec::new(),
        }
    }

    pub fn mouse_to_board_coords(&self, mouse_x: f32, mouse_y: f32) -> Option<(usize, usize)> {
        let (width, height) = self.board.dims();
        let screen_width = width as f32 * TILE_SIZE;
        let screen_height = height as f32 * TILE_SIZE;

        let x_in_bounds = mouse_x >= 0. && mouse_x <= screen_width;
        let y_in_bounds = mouse_y >= 0. && mouse_y <= screen_height;

        if !x_in_bounds || !y_in_bounds {
            return None;
        }

        let board_x = f32::floor(mouse_x / TILE_SIZE) as usize;
        let board_y = f32::floor(mouse_y / TILE_SIZE) as usize;

        Some((board_x, board_y))
    }

    pub fn stage(&self) -> Stage {
        self.stage
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn hover_index(&self) -> Option<(usize, usize)> {
        self.hover
    }

    pub fn elapsed_milisec(&self) -> u32 {
        self.elapsed_milisec
    }

    pub fn global_milisec(&self) -> f32 {
        self.global_milisec
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn font_mono(&self) -> &Font {
        &self.font_mono
    }

    pub fn explosions(&self) -> &Vec<Explosion> {
        &self.explosions
    }
    pub fn confetti_explosions(&self) -> &Vec<ConfettiExplosion> {
        &self.confetti_explosions
    }
}

pub fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("../assets/OpenSauceTwo-Bold.ttf"))
        .unwrap();
    let font_mono = gfx
        .create_font(include_bytes!(
            "../assets/martian-mono-latin-400-normal.ttf"
        ))
        .unwrap();

    State::new(font, font_mono)
}

pub fn update(app: &mut App, state: &mut State) {
    match state.stage {
        Stage::Playing => playing::update(app, state),
        Stage::Paused => paused::update(app, state),
        Stage::Defeat => defeat::update(app, state),
        Stage::Victory => victory::update(app, state),
    }
    state.global_milisec += app.timer.delta().subsec_millis() as f32;
}
