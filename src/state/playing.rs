use notan::prelude::*;

use crate::state::{Stage, State};

pub fn update(app: &mut App, state: &mut State) {
    if state.board().is_initialized() {
        let delta = app.timer.delta().subsec_millis();
        state.elapsed_milisec += delta;
    }

    let (mouse_x, mouse_y) = app.mouse.position();
    let board_coords = state.mouse_to_board_coords(mouse_x, mouse_y);

    state.hover = board_coords;

    if let Some((x, y)) = board_coords {
        if app.mouse.left_was_pressed() {
            state.board.handle_uncover(x, y);
        } else if app.mouse.right_was_pressed() {
            state.board.handle_mark(x, y);
        }
    }

    if state.board.is_defeat() {
        state.stage = Stage::Defeat;
    } else if state.board.is_victory() {
        state.stage = Stage::Victory;
    }

    if app.keyboard.was_pressed(KeyCode::Return) {
        state.stage = Stage::Paused;
    }
}
