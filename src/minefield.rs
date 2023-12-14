use std::collections::HashSet;

use crate::random::IteratorRandom;

pub const BEGINNER: Params = Params {
    width: 8,
    height: 8,
    mines: 10,
};
pub const INTERMEDIATE: Params = Params {
    width: 16,
    height: 16,
    mines: 40,
};
pub const EXPERT: Params = Params {
    width: 30,
    height: 16,
    mines: 99,
};

#[derive(Debug, Clone, Copy)]
pub struct Params {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mark {
    /// Mine flag, indicates 100% player certainty of a mine,
    /// and disables uncovering the marked field, for safety.
    Flag,

    /// "Danger, probably" marker, for fields that are sorta suspicious,
    /// but not yet worthy of The [Mark::Flag].
    Unsure,
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Cover {
    Up(Mark),
    Down,
}

#[derive(Debug, Clone, Copy)]
pub enum Object {
    Mine,
    Hint(u8),
    Blank,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    cover: Cover,
    object: Object,
}

#[derive(Debug)]
pub struct Board {
    tiles: Vec<Tile>,
    covered: usize,
    flags: usize,
    params: Params,
    placed: bool,
    defeat: bool,
}

impl Mark {
    fn cycle(&mut self) {
        *self = match self {
            Self::None => Self::Flag,
            Self::Flag => Self::Unsure,
            Self::Unsure => Self::None,
        };
    }
}

impl Tile {
    fn new() -> Self {
        Self {
            cover: Cover::Up(Mark::None),
            object: Object::Blank,
        }
    }

    pub fn cover(&self) -> Cover {
        self.cover
    }

    pub fn object(&self) -> Object {
        self.object
    }

    pub fn is_uncoverable(&self) -> bool {
        matches!(self.cover, Cover::Up(mark) if mark != Mark::Flag)
    }

    pub fn is_mine(&self) -> bool {
        matches!(self.object, Object::Mine)
    }

    pub fn is_hint(&self) -> bool {
        matches!(self.object, Object::Hint(_))
    }

    pub fn is_blank(&self) -> bool {
        matches!(self.object, Object::Blank)
    }

    pub fn is_flag(&self) -> bool {
        matches!(self.cover, Cover::Up(Mark::Flag))
    }
}

impl Board {
    pub fn new(params: Params) -> Self {
        let size = params.width * params.height;
        Self {
            tiles: vec![Tile::new(); size],
            covered: size,
            flags: 0,
            placed: false,
            defeat: false,
            params,
        }
    }

    pub fn beginner() -> Self {
        Self::new(BEGINNER)
    }

    pub fn intermediate() -> Self {
        Self::new(INTERMEDIATE)
    }

    pub fn expert() -> Self {
        Self::new(EXPERT)
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.params.width, self.params.height)
    }

    pub fn mines(&self) -> usize {
        self.params.mines
    }

    pub fn tile(&self, x: usize, y: usize) -> Tile {
        let index = self.coords_to_index(x, y);
        self.tiles[index]
    }

    pub fn flags(&self) -> usize {
        self.flags
    }

    pub fn is_victory(&self) -> bool {
        self.covered == self.params.mines
    }

    pub fn is_defeat(&self) -> bool {
        self.defeat
    }

    pub fn is_initialized(&self) -> bool {
        self.placed
    }

    /// Primary interface for acting on a minefield.
    ///
    /// Corresponds to one of the primary actions on a tile:
    ///
    /// - uncovering a covered tile
    /// - doing an "explore" action around a hint tile
    ///
    /// Uncovering every non-mine tile is the win condition.
    /// Note that the mine tiles are **not** required to be flagged (looking at you, speedrunners).
    pub fn handle_primary_action(&mut self, x: usize, y: usize) {
        if !self.placed {
            self.place_mines_and_hints(x, y);
            self.placed = true;
        }

        let tile_idx = self.coords_to_index(x, y);
        let tile = &mut self.tiles[tile_idx];

        if let (Cover::Down, Object::Hint(hint)) = (tile.cover, tile.object) {
            self.explore_around(hint, x, y);
            return;
        }

        if !tile.is_uncoverable() {
            return;
        }

        self.uncover(x, y);
    }

    /// Primary interface for acting on a minefield.
    ///
    /// Corresponds to the action of cycling through
    /// available covered-tile marks (the [Mark] type).
    pub fn handle_secondary_action(&mut self, x: usize, y: usize) {
        let tile_idx = self.coords_to_index(x, y);
        let Cover::Up(mark) = &mut self.tiles[tile_idx].cover else {
            return;
        };

        mark.cycle();
        match mark {
            Mark::Flag => self.flags += 1,
            Mark::Unsure => self.flags -= 1,
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        self.tiles.fill(Tile::new());
        self.placed = false;
        self.defeat = false;
        self.covered = self.tiles.len();
        self.flags = 0;
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        // copy these values to avoid borrowing `self` in the closure
        let width = self.params.width;
        let height = self.params.height;
        offsets.into_iter().filter_map(move |(off_x, off_y)| {
            let new_x = x as i32 + off_x;
            let new_y = y as i32 + off_y;
            let x_in_bounds = new_x >= 0 && new_x < width as i32;
            let y_in_bounds = new_y >= 0 && new_y < height as i32;

            if !x_in_bounds || !y_in_bounds {
                return None;
            }

            Some((new_x as _, new_y as _))
        })
    }

    /// A flood-fill-style uncovering procedure,
    /// where the uncovering "spills" over a surrounding area
    /// bounded by hint tiles (inclusive).
    ///
    /// Algorithmically, this is equivalent to a DFS/BFS traversal
    /// starting from a player-uncovered tile
    /// and stopping on already uncovered tiles and hint tiles.
    fn flood_uncover(&mut self, x: usize, y: usize) {
        let mut flooded: Vec<_> = self.neighbors(x, y).collect();
        let mut visited = HashSet::new();

        while let Some((current_x, current_y)) = flooded.pop() {
            let t_idx = self.coords_to_index(current_x, current_y);
            let tile = &mut self.tiles[t_idx];

            if !tile.is_uncoverable() || !visited.insert((current_x, current_y)) {
                continue;
            }

            tile.cover = Cover::Down;
            self.covered -= 1;

            if tile.is_blank() {
                let n = self.neighbors(current_x, current_y);
                flooded.extend(n);
            }
        }
    }

    fn uncover(&mut self, x: usize, y: usize) {
        let tile_idx = self.coords_to_index(x, y);
        let tile = &mut self.tiles[tile_idx];

        tile.cover = Cover::Down;
        self.covered -= 1;

        match tile.object {
            Object::Mine => self.defeat = true,
            Object::Blank => self.flood_uncover(x, y),
            Object::Hint(_) => (),
        }
    }

    /// Clicking on a hint tile if there are exactly as many flags around it as hinted
    /// causes the remaining covered tiles to be uncovered automatically.
    ///
    /// Beware: if the flags are misplaced, this is an instant defeat!
    fn explore_around(&mut self, hinted: u8, x: usize, y: usize) {
        let neighbors: Vec<_> = self.neighbors(x, y).collect();
        let n_flags = neighbors
            .iter()
            .filter(|&&(xx, yy)| self.tiles[self.coords_to_index(xx, yy)].is_flag())
            .count();

        if hinted as usize != n_flags {
            return;
        }

        let explored: Vec<_> = neighbors
            .iter()
            .copied()
            .filter(|&(xx, yy)| self.tiles[self.coords_to_index(xx, yy)].is_uncoverable())
            .collect();

        for (current_x, current_y) in explored {
            self.uncover(current_x, current_y);
        }
    }

    /// Get single dimension index of 2D tile in tiles array
    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        y * self.params.width + x
    }

    fn place_mines_and_hints(&mut self, x: usize, y: usize) {
        let skip: Vec<_> = self
            .neighbors(x, y)
            .chain([(x, y)])
            .map(|(xx, yy)| self.coords_to_index(xx, yy))
            .collect();
        self.place_mines(&skip);
        self.place_hints();
    }

    /// Place mines on the field.
    ///
    /// The `skip` argument contains board indices
    /// that shall not have a mine placed in.
    fn place_mines(&mut self, skip: &[usize]) {
        let mut rng = nanorand::WyRand::new();
        let mines = (0..self.tiles.len())
            .filter(|i| !skip.contains(i))
            .choose_multiple(&mut rng, self.params.mines);

        for mine in mines {
            self.tiles[mine].object = Object::Mine;
        }
    }

    fn place_hints(&mut self) {
        for x in 0..self.params.width {
            for y in 0..self.params.height {
                let idx = self.coords_to_index(x, y);
                if self.tiles[idx].is_mine() {
                    continue;
                }
                let mine_count = self
                    .neighbors(x, y)
                    .filter(|&(xx, yy)| self.tiles[self.coords_to_index(xx, yy)].is_mine())
                    .count();
                if mine_count > 0 {
                    self.tiles[idx].object = Object::Hint(mine_count as _);
                }
            }
        }
    }
}
