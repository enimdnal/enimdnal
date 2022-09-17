pub(crate) mod defeat;
pub(crate) mod playing;
pub(crate) mod victory;

#[derive(Clone, Copy)]
pub enum Stage {
    Playing,
    Victory,
    Defeat,
}
