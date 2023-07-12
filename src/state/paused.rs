use notan::prelude::*;

use crate::state::State;

use super::Stage;

pub fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Return) {
        state.stage = Stage::Playing;
    }
}
