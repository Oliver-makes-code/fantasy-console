use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::{color::Color, math::Fixed};

static STATE: OnceLock<Mutex<TileState>> = OnceLock::new();

/// 1 nibble per pixel, 16*16 pixels, 128 bytes
#[derive(Debug, Clone, Copy)]
pub struct Tile(pub [u8; 128]);

#[derive(Debug, Clone, Copy)]
pub struct TileMap {
    pub palettes: [u8; 4096],
    pub tiles: [u8; 4096],
    pub pre_offset: (i16, i16),
    pub post_offset: (i16, i16),
    pub matrix: ((Fixed, Fixed), (Fixed, Fixed)),
}

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub palette: u8,
    pub tile: u8,
    pub position: (i16, i16)
}


#[derive(Debug, Clone, Copy)]
pub struct TileState {
    pub palette: [Color; 256],
    pub tiles: [Tile; 256],
    pub background_color: u8,
    pub backgrounds: [TileMap; 8],
}
impl Tile {
    pub fn get_color(&self, idx: usize) -> u8 {
        let b = self.0[idx / 2];
        if idx % 2 == 0 {
            ((b >> 4) % 16) as u8
        } else {
            (b % 16) as u8
        }
    }
}

impl TileMap {
    pub fn get_color_offset(&self, tile_state: &TileState, px: usize, py: usize) -> (u8, u8) {
        let tile_loc = self.get_tile_index(px, py);
        let tile_idx = self.tiles[tile_loc];
        let tile = tile_state.tiles[tile_idx as usize];
        let tile_offset = self.get_tile_offset(px, py);
        (tile.get_color(tile_offset), self.palettes[tile_loc])
    }

    fn transform_coords(&self, px: isize, py: isize) -> (usize, usize) {
        let tx = px + self.pre_offset.0 as isize;
        let ty = py + self.pre_offset.1 as isize;

        let mut x = tx * self.matrix.0 .0 + ty * self.matrix.0 .1;
        let mut y = tx * self.matrix.1 .0 + ty * self.matrix.1 .1;

        x += self.post_offset.0 as isize;

        y += self.post_offset.1 as isize;

        x = (x % 1024 + 1024) % 1024;

        y = (y % 1024 + 1024) % 1024;

        (x as usize, y as usize)
    }

    fn get_tile_index(&self, px: usize, py: usize) -> usize {
        let (mut x, mut y) = self.transform_coords(px as isize, py as isize);

        x %= 1024;
        x /= 16;

        y %= 1024;
        y /= 16;
        y *= 64;

        x | y
    }

    fn get_tile_offset(&self, px: usize, py: usize) -> usize {
        let (mut x, mut y) = self.transform_coords(px as isize, py as isize);

        x %= 16;

        y %= 16;
        y *= 16;

        x | y
    }
}

impl TileState {
    pub fn get() -> MutexGuard<'static, Self> {
        STATE
            .get_or_init(|| Mutex::new(TileState::new()))
            .lock()
            .unwrap()
    }

    fn new() -> Self {
        Self {
            background_color: 0,
            palette: [(255, 0, 0).into(); 256],
            tiles: [Tile([0; 128]); 256],
            backgrounds: [TileMap {
                palettes: [0; 4096],
                tiles: [0; 4096],
                pre_offset: (0, 0),
                post_offset: (0, 0),
                matrix: (
                    (Fixed::from(256), Fixed::from(0)),
                    (Fixed::from(0), Fixed::from(256)),
                ),
            }; 8],
        }
    }
}
