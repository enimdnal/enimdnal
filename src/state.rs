pub(crate) mod game;

use notan::prelude::*;

use game::{defeat, playing, victory, Stage};

#[derive(AppState)]
pub struct State {
    stage: Stage,
}

impl State {
    pub fn new() -> Self {
        Self {
            stage: Stage::Playing,
        }
    }

    pub fn stage(&self) -> Stage {
        self.stage
    }
}

pub fn update(app: &mut App, state: &mut State) {
    match state.stage {
        Stage::Playing => playing::update(app, state),
        Stage::Victory => victory::update(app, state),
        Stage::Defeat => defeat::update(app, state),
    }
}
