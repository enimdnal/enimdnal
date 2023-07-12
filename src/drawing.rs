use notan::draw::*;
use notan::math::{Mat3, Vec2};
use notan::prelude::*;

use crate::minefield::{Cover, Mark, Object, Params};
use crate::state::{Stage, State};

pub const TILE_SIZE: f32 = 40.;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.;
pub const UI_WIDTH: f32 = 300.;

const OUTLINE_COLOR: Color = Color::from_rgb(0., 0.8, 0.7);
const WIN_COLOR: Color = Color::GREEN;

const MINE_COLOR: Color = Color::BLACK;
const BLANK_COLOR: Color = Color::WHITE;
const HINT_COLOR: Color = Color::PINK;

const COVER_COLOR: Color = Color::GRAY;
const FLAG_COLOR: Color = Color::RED;
const UNSURE_COLOR: Color = Color::BLUE;

pub fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::BLACK);

    draw_ui(&mut draw, state);

    draw_board(&mut draw, state);

    gfx.render(&draw);
}

pub fn board_dims(params: Params) -> (f32, f32) {
    (
        params.width as f32 * TILE_SIZE,
        params.height as f32 * TILE_SIZE,
    )
}

fn draw_board(draw: &mut Draw, state: &State) {
    let (cols, rows) = state.board().dims();

    for y in 0..rows {
        for x in 0..cols {
            draw_tile(draw, state, x, y);
        }
    }
}

fn draw_tile(draw: &mut Draw, state: &State, x: usize, y: usize) {
    const DIMS: (f32, f32) = (TILE_SIZE, TILE_SIZE);
    const STROKE: f32 = 3.;

    let screen_x = x as f32 * TILE_SIZE;
    let screen_y = y as f32 * TILE_SIZE;
    let pos = (screen_x, screen_y);

    let tile = state.board().tile(x, y);
    let cover = tile.cover();
    let object = tile.object();

    let mut fill_color = match (cover, object) {
        (Cover::Up(Mark::None), _) => COVER_COLOR,
        (Cover::Up(Mark::Flag), _) => FLAG_COLOR,
        (Cover::Up(Mark::Unsure), _) => UNSURE_COLOR,
        (Cover::Down, Object::Blank) => BLANK_COLOR,
        (Cover::Down, Object::Hint(_)) => HINT_COLOR,
        (Cover::Down, Object::Mine) => MINE_COLOR,
    };

    match state.stage() {
        Stage::Defeat => {
            if let Object::Mine = object {
                fill_color = MINE_COLOR;
            }
        }
        Stage::Victory => {
            if let Cover::Up(_) = cover {
                fill_color = WIN_COLOR;
            }
        }
        _ => (),
    }

    if let Some(hover_coords) = state.hover_index() {
        if (x, y) == hover_coords && matches!(cover, Cover::Up(_)) {
            hover_color(&mut fill_color);
        }
    }

    draw.rect(pos, DIMS).color(fill_color);
    draw.rect(pos, DIMS).color(OUTLINE_COLOR).stroke(STROKE);

    if let (Cover::Down, Object::Hint(n)) = (cover, object) {
        draw.text(state.font(), &n.to_string())
            .color(Color::BLACK)
            .size(26.0)
            .position(screen_x + HALF_TILE_SIZE, screen_y + HALF_TILE_SIZE)
            .h_align_center()
            .v_align_middle();
    }
}

fn hover_color(color: &mut Color) {
    let Color { r, g, b, .. } = color;

    *r *= 0.8;
    *g *= 0.8;
    *b *= 0.8;
}

fn draw_ui(draw: &mut Draw, state: &State) {
    let (cols, _) = state.board().dims();

    draw.transform().push(Mat3::from_translation(Vec2::new(
        cols as f32 * TILE_SIZE,
        0.,
    )));

    let elapsed = state.run_timer_milisec();

    let milis = elapsed % 1000;
    let secs = (elapsed / 1000) % 60;
    let mins = elapsed / 60_000;

    let time = format!("{:02}:{:02}.{:03}", mins, secs, milis);

    draw.text(state.font_mono(), &time)
        .color(Color::WHITE)
        .size(30.)
        .position(UI_WIDTH / 2., TILE_SIZE)
        .h_align_center()
        .v_align_middle();

    let flags = state.board().flags();
    let mines = state.board().mines();

    let flag_counter = format!("{:03} / {:03}", flags, mines);

    draw.text(state.font_mono(), &flag_counter)
        .color(Color::WHITE)
        .size(30.)
        .position(UI_WIDTH / 2., TILE_SIZE * 3.)
        .h_align_center()
        .v_align_middle();

    draw.transform().pop();
}
