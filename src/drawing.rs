use notan::draw::*;
use notan::math::{Mat3, Vec2};
use notan::prelude::*;
use notan::random::rand;

use crate::minefield::{Cover, Mark, Object, Params};
use crate::state::{Stage, State};

pub const TILE_SIZE: f32 = 40.;
pub const HALF_TILE_SIZE: f32 = 40.;
pub const STROKE: f32 = 3.;
pub const UI_WIDTH: f32 = 300.;

const OUTLINE_COLOR: Color = Color::from_rgb(0., 0.8, 0.7);
const WIN_COLOR: Color = Color::GREEN;

const MINE_COLOR: Color = Color::BLACK;
const BLANK_COLOR: Color = Color::WHITE;
const HINT_COLOR: Color = Color::PINK;

const COVER_COLOR: Color = Color::GRAY;
const FLAG_COLOR: Color = Color::RED;
const UNSURE_COLOR: Color = Color::BLUE;

pub(crate) struct Particle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    direction: f32,
}

pub struct Explosion {
    start_time: f32,
    particles: Vec<Particle>,
}

impl Explosion {
    pub(crate) fn new(start_time: f32, square_size: f32, square_position: (f32, f32)) -> Self {
        let mut rectangles: Vec<Particle> = Vec::new();
        let p = Particle {
            color: Color::RED,
            x: square_position.0,
            y: square_position.1,
            width: square_size,
            height: square_size,
            direction: 1.,
        };
        rectangles.push(p);
        Self {
            start_time: start_time,
            particles: rectangles,
        }
    }

    pub(crate) fn pos(&self) -> (f32, f32) {
        (
            self.particles.first().unwrap().x,
            self.particles.first().unwrap().y,
        )
    }

    pub(crate) fn delay(&mut self, time: f32) {
        self.start_time += time;
    }
}

pub struct ConfettiExplosion {
    start_time: f32,
    particles: Vec<Particle>,
}

impl ConfettiExplosion {
    pub(crate) fn new(
        start_time: f32,
        square_size: f32,
        square_position: (f32, f32),
        num_rectangles: i32,
    ) -> Self {
        let rectangles = break_rectangle(square_position, num_rectangles, square_size);
        Self {
            start_time: start_time + rand::random::<f32>() * 2000.,
            particles: rectangles,
        }
    }
}

fn break_rectangle(
    square_position: (f32, f32),
    num_rectangles: i32,
    square_size: f32,
) -> Vec<Particle> {
    let mut rectangles = Vec::new();

    let square_x = square_position.0;
    let square_y = square_position.1;

    let squashing_ratio = (num_rectangles as f32).sqrt();
    let rectangle_size = square_size / squashing_ratio;

    for i in 0..squashing_ratio as i32 {
        for j in 0..squashing_ratio as i32 {
            let size_ratio = rand::random::<f32>();
            let rectangle = Particle {
                x: square_x + i as f32 * rectangle_size,
                y: square_y + j as f32 * rectangle_size,
                width: rectangle_size * size_ratio,
                height: rectangle_size * size_ratio,
                color: Color::from_rgb(
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                    rand::random::<f32>(),
                ),
                direction: rand::random::<f32>() * 2. - 1.,
            };
            rectangles.push(rectangle);
        }
    }
    rectangles
}

pub fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::BLACK);

    draw_ui(&mut draw, state);

    if state.stage() == Stage::Paused {
        draw_paused(&mut draw, state);
    } else {
        draw_board(&mut draw, state);
    }

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

    state.explosions().iter().for_each(|e| {
        draw_explosion_tile(draw, state.global_milisec(), e.start_time, &e.particles)
    });

    state.confetti_explosions().iter().for_each(|e| {
        draw_confetti_explosion_tile(draw, state.global_milisec(), e.start_time, &e.particles)
    });
}

fn draw_tile(draw: &mut Draw, state: &State, x: usize, y: usize) {
    const DIMS: (f32, f32) = (TILE_SIZE, TILE_SIZE);

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

    // adjust color for mouse hover
    if let Some(hover_coords) = state.hover_index() {
        if (x, y) == hover_coords && matches!(cover, Cover::Up(_)) {
            hover_color(&mut fill_color);
        }
    }

    draw.rect(pos, DIMS).color(fill_color);
    draw.rect(pos, DIMS).color(OUTLINE_COLOR).stroke(STROKE);

    // draw hint number
    if let (Cover::Down, Object::Hint(n)) = (cover, object) {
        draw.text(state.font(), &n.to_string())
            .color(Color::BLACK)
            .size(26.0)
            .position(screen_x + TILE_SIZE * 0.5, screen_y + TILE_SIZE * 0.5)
            .h_align_center()
            .v_align_middle();
    }
}

fn draw_confetti_explosion_tile(
    draw: &mut Draw,
    curr_time: f32,
    start_time: f32,
    particles: &Vec<Particle>,
) {
    const ANIMATION_TIME: f32 = 1000.;

    let time_diff = (curr_time - start_time) as f32;

    let progress: f32 = time_diff / ANIMATION_TIME;
    let velocity = 4. * HALF_TILE_SIZE;
    let acceleration: f32 = -2. * velocity;
    let time = progress as f32;
    let vertical_displacement = velocity * time + 0.5 * acceleration * time.powf(2.);

    particles.iter().for_each(|p| {
        let direction = if p.direction >= 0. {
            p.direction + 1.
        } else {
            p.direction - 1.
        };
        let explosion_pos = (
            p.x + progress * HALF_TILE_SIZE * direction,
            p.y - vertical_displacement * direction.abs(),
        );
        // let alpha = 1. - progress.powf(2.);
        let alpha = 1.;
        let angle = progress.log2() * p.direction;
        draw.rect(explosion_pos, (p.width, p.height))
            .color(p.color)
            .alpha(alpha)
            .rotate_from(
                (
                    explosion_pos.0 + p.width / 2.,
                    explosion_pos.1 + p.height / 2.,
                ),
                angle,
            );
    });
}

fn draw_explosion_tile(
    draw: &mut Draw,
    curr_time: f32,
    start_time: f32,
    particles: &Vec<Particle>,
) {
    const ANIMATION_TIME: f32 = 500.;

    let time_diff = (curr_time - start_time) as f32;
    let progress: f32 = time_diff / ANIMATION_TIME;
    if time_diff < 0. {
        return;
    }

    particles.iter().for_each(|p| {
        // let angle = progress.log2() * p.direction;
        let ratio = gauss(progress, 0.5, 0., 1.);
        let shift = HALF_TILE_SIZE * ratio;
        let exploded_size = (p.width + shift, p.height + shift);
        draw.rect((p.x - shift / 2., p.y - shift / 2.), exploded_size)
            .color(p.color);
        draw.rect((p.x - shift / 2., p.y - shift / 2.), exploded_size)
            .color(OUTLINE_COLOR)
            .stroke(STROKE);
    });
    // }
}

fn gauss(number: f32, a: f32, b: f32, c: f32) -> f32 {
    a * (-1. * (number - b).powf(2.) / (2. * c * c)).exp()
}

fn hover_color(color: &mut Color) {
    let Color { r, g, b, .. } = color;

    *r *= 0.8;
    *g *= 0.8;
    *b *= 0.8;
}

fn draw_paused(draw: &mut Draw, state: &State) {
    let (cols, rows) = state.board().dims();
    let size @ (width, height) = (cols as f32 * TILE_SIZE, rows as f32 * TILE_SIZE);

    draw.rect((0., 0.), size).color(COVER_COLOR);
    draw.rect((0., 0.), size).color(OUTLINE_COLOR).stroke(3.);

    draw.text(state.font(), "PAUSED")
        .color(Color::WHITE)
        .size(40.)
        .position(width / 2., height / 2.)
        .h_align_center()
        .v_align_middle();
}

fn draw_ui(draw: &mut Draw, state: &State) {
    let (cols, _) = state.board().dims();

    draw.transform().push(Mat3::from_translation(Vec2::new(
        cols as f32 * TILE_SIZE,
        0.,
    )));

    let elapsed = state.elapsed_milisec();

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
