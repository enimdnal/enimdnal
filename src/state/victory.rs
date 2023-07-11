use notan::prelude::*;

use crate::{
    drawing::{ConfettiExplosion, TILE_SIZE},
    minefield::Object,
    state::{Stage, State},
};

pub fn update(app: &mut App, state: &mut State) {
    state.hover = None;

    if app.keyboard.was_pressed(KeyCode::Space) {
        state.stage = Stage::Playing;
        state.elapsed_milisec = 0;
        state.board.reset();
        state.confetti_explosions.clear();
    }

    if state.confetti_explosions.is_empty() {
        let elapsed = state.global_milisec();
        let (width, height) = state.board().dims();
        for x in 0..width {
            for y in 0..height {
                let tile = state.board().tile(x, y);

                match tile.object() {
                    Object::Mine => {
                        // println!("Mine nr=({x};{y}) pos=({};{})", x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
                        let explosion = ConfettiExplosion::new(
                            elapsed,
                            TILE_SIZE,
                            (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE),
                            50,
                        );
                        state.confetti_explosions.push(explosion);
                    }
                    Object::Hint(_) => {}
                    Object::Blank => {}
                }
            }
        }
    }
}
