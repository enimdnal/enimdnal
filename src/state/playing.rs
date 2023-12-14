use itertools::Itertools;
use notan::prelude::*;

use crate::state::defeat::{DefeatState, Explosion};
use crate::state::{Stage, State};

pub fn update(app: &mut App, state: &mut State) {
    if state.board().is_initialized() {
        let delta = app.timer.delta().subsec_millis();
        state.run_timer_milisec += delta;
    }

    let (mouse_x, mouse_y) = app.mouse.position();
    let board_coords = state.mouse_to_board_coords(mouse_x, mouse_y);

    state.hover = board_coords;

    if let Some((x, y)) = board_coords {
        if app.mouse.left_was_pressed() {
            state.board.handle_primary_action(x, y);
        } else if app.mouse.right_was_pressed() {
            state.board.handle_secondary_action(x, y);
        }
    }

    if state.board.is_defeat() {
        let triggered_pos = board_coords
            .expect("Failed to obtain board coords when transitioning playing -> defeat");
        transition_defeat(state, triggered_pos);
    } else if state.board.is_victory() {
        state.stage = Stage::Victory;
    }

    if app.keyboard.was_pressed(KeyCode::Return) {
        state.stage = Stage::Paused;
    }
}

fn transition_defeat(state: &mut State, triggered_pos: (usize, usize)) {
    const EXPLOSION_RING_DELAY: u32 = 80;

    let (width, height) = state.board().dims();
    let mut explosions = vec![];

    for y in 0..height {
        for x in 0..width {
            let tile = state.board().tile(x, y);

            if !tile.is_mine() {
                continue;
            }

            let explosion = Explosion {
                pos: (x, y),
                delay: 0,
            };
            explosions.push(explosion);
        }
    }

    explosions.sort_by_key(|expl| distance(triggered_pos, expl.pos));

    let rings = explosions
        .iter_mut()
        .group_by(|expl| distance(triggered_pos, expl.pos));
    let mut current_delay = 0;
    for (_, ring) in &rings {
        for expl in ring {
            expl.delay = current_delay;
        }

        current_delay += EXPLOSION_RING_DELAY;
    }

    state.stage = Stage::Defeat(DefeatState {
        explosions,
        elapsed_milisec: 0,
    });
}

fn distance((from_x, from_y): (usize, usize), (p_x, p_y): (usize, usize)) -> usize {
    let x_projection = usize::abs_diff(from_x, p_x);
    let y_projection = usize::abs_diff(from_y, p_y);

    std::cmp::max(x_projection, y_projection)
}
