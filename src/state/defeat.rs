use notan::prelude::*;

use crate::state::{Stage, State};

#[derive(Debug)]
pub struct Explosion {
    pub pos: (usize, usize),
    pub delay: u32,
}

#[derive(Debug)]
pub struct DefeatState {
    pub explosions: Vec<Explosion>,
    pub elapsed_milisec: u32,
}

impl DefeatState {
    pub fn update(&mut self, app: &App) {
        let delta = app.timer.delta().subsec_millis();
        self.elapsed_milisec += delta;
    }
}

pub fn update(app: &mut App, state: &mut State) {
    state.hover = None;

    if app.keyboard.was_pressed(KeyCode::Space) {
        state.stage = Stage::Playing;
        state.run_timer_milisec = 0;
        state.board.reset();
    }
}
