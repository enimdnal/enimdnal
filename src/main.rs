#![allow(clippy::main_recursion)]

pub(crate) mod minefield;
pub(crate) mod state;

use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().build()
}
