use notan::prelude::*;

use crate::state::{Stage, State};

pub fn update(app: &mut App, state: &mut State) {
    state.hover = None;

    if app.keyboard.was_pressed(KeyCode::Space) {
        state.stage = Stage::Playing;
        state.run_timer_milisec = 0;
        state.board.reset();
    }
}
