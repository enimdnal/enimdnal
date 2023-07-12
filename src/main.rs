#![allow(clippy::main_recursion)]

pub(crate) mod drawing;
pub(crate) mod minefield;
pub(crate) mod state;

use notan::draw::*;
use notan::prelude::*;

use drawing::UI_WIDTH;

#[notan_main]
fn main() -> Result<(), String> {
    let difficulty = minefield::EXPERT;
    let (width, height) = drawing::board_dims(difficulty);
    let win = WindowConfig::default()
        .title("Enimdnal")
        .size(width as i32 + UI_WIDTH as i32, height as _);
    notan::init_with(state::setup)
        .update(state::update)
        .draw(drawing::draw)
        .add_config(win)
        .add_config(DrawConfig)
        .build()
}
