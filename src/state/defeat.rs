use std::cmp::Ordering;

use notan::prelude::*;

use crate::{
    drawing::{self, TILE_SIZE},
    minefield::Object,
    state::{Stage, State},
};

use drawing::Explosion;

pub fn update(app: &mut App, state: &mut State) {
    state.hover = None;

    if app.keyboard.was_pressed(KeyCode::Space) {
        state.stage = Stage::Playing;
        state.elapsed_milisec = 0;
        state.board.reset();
        state.explosions.clear();
    }

    if state.explosions.is_empty() {
        let elapsed = state.global_milisec();
        let (width, height) = state.board().dims();

        for x in 0..width {
            for y in 0..height {
                let tile = state.board().tile(x, y);

                match tile.object() {
                    Object::Mine => {
                        // println!("Mine nr=({x};{y}) pos=({};{})", x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
                        let explosion = Explosion::new(
                            elapsed,
                            TILE_SIZE,
                            (x as f32 * TILE_SIZE, y as f32 * TILE_SIZE),
                        );
                        state.explosions.push(explosion);
                    }
                    Object::Hint(_) => {}
                    Object::Blank => {}
                }
            }
        }
        let (x0, y0) = (state.last_coords.0 as f32 * TILE_SIZE, state.last_coords.1 as f32 * TILE_SIZE);
        state.explosions.sort_by(|el_l, el_r| {
            let dist_a = distance(x0, y0, el_l.pos().0, el_l.pos().1);
            let dist_b = distance(x0, y0, el_r.pos().0, el_r.pos().1);
            dist_a.partial_cmp(&dist_b).unwrap_or(Ordering::Equal)
        });
        let mut delay = 0 as f32;
        state.explosions.iter_mut().for_each(|e| {
            e.delay(delay);
            delay += 15.;
        });
    }
}

fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
}
