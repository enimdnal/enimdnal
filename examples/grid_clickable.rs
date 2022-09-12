use notan::draw::*;
use notan::prelude::*;

const COLS: usize = 30;
const ROWS: usize = 16;
const TILES: usize = COLS * ROWS;
const TILE_SIZE: f32 = 40.;
const BOARD_WIDTH: f32 = COLS as f32 * TILE_SIZE;
const BOARD_HEIGHT: f32 = ROWS as f32 * TILE_SIZE;

const IDLE_COLOR: Color = Color::WHITE;
const HOVER_COLOR: Color = Color::from_rgb(0.8, 0.8, 0.8);
const CLICKED_COLOR: Color = Color::RED;
const HOVER_CLICKED_COLOR: Color = Color::from_rgb(0.8, 0., 0.);

const OUTLINE_COLOR: Color = Color::from_rgb(0., 0.8, 0.7);

#[derive(AppState)]
struct State {
    board: [Tile; TILES],
    hover_index: Option<usize>,
}

#[derive(Clone, Copy)]
struct Tile {
    clicked: bool,
}

#[notan_main]
fn main() -> Result<(), String> {
    let win = WindowConfig::default()
        .title("Clickable Grid")
        .size(BOARD_WIDTH as _, BOARD_HEIGHT as _)
        .resizable(false);
    notan::init_with(setup)
        .update(update)
        .draw(draw)
        .add_config(win)
        .add_config(DrawConfig)
        .build()
}

fn setup() -> State {
    State {
        board: [Tile { clicked: false }; TILES],
        hover_index: None,
    }
}

fn update(app: &mut App, state: &mut State) {
    let (x, y) = app.mouse.position();
    let maybe_index = mouse_pos_to_index(x, y);
    state.hover_index = maybe_index;

    if app.mouse.left_was_pressed() {
        if let Some(index) = maybe_index {
            let Tile { clicked } = &mut state.board[index];
            *clicked = !*clicked;
        }
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::BLACK);

    for y in 0..ROWS {
        for x in 0..COLS {
            let index = y * COLS + x;
            let Tile { clicked } = state.board[index];
            let hover = state.hover_index == Some(index);

            draw_tile(&mut draw, x, y, hover, clicked);
        }
    }

    gfx.render(&draw);
}

fn draw_tile(draw: &mut Draw, x: usize, y: usize, hover: bool, clicked: bool) {
    const DIMS: (f32, f32) = (TILE_SIZE, TILE_SIZE);
    const STROKE: f32 = 3.;

    let screen_x = x as f32 * TILE_SIZE;
    let screen_y = y as f32 * TILE_SIZE;
    let pos = (screen_x, screen_y);

    let fill_color = match (hover, clicked) {
        (false, false) => IDLE_COLOR,
        (true, false) => HOVER_COLOR,
        (false, true) => CLICKED_COLOR,
        (true, true) => HOVER_CLICKED_COLOR,
    };

    draw.rect(pos, DIMS).color(fill_color);
    draw.rect(pos, DIMS).color(OUTLINE_COLOR).stroke(STROKE);
}

fn mouse_pos_to_index(x: f32, y: f32) -> Option<usize> {
    let in_x_bounds = x > 0. && x < BOARD_WIDTH;
    let in_y_bounds = y > 0. && y < BOARD_HEIGHT;

    if !in_x_bounds || !in_y_bounds {
        return None;
    }

    let tile_x = f32::floor(x / TILE_SIZE) as usize;
    let tile_y = f32::floor(y / TILE_SIZE) as usize;

    Some(tile_y * COLS + tile_x)
}
