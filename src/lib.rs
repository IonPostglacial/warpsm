extern crate alloc;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{FreeListAllocator, LockedAllocator};

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/warpsm.mjs")]
extern "C" {
    fn canvas_set_fill_style(color: u32);
    fn canvas_fill_rect(x: usize, y: usize, width: usize, height: usize);
    fn canvas_fill();
    fn canvas_draw_image(sx: usize, sy: usize, sw: usize, sh: usize, dx: usize, dy: usize, dw: usize, dh: usize);
    fn game_over();
}

const COLOR_BACKGROUND: u32 = 0x00000000;
const CELL_SIZE: usize = 32;
const GRID_WIDTH: usize = 12;
const GRID_HEIGHT: usize = 12;

fn paint_background() {
    canvas_set_fill_style(COLOR_BACKGROUND);
    canvas_fill_rect(0, 0, GRID_WIDTH * CELL_SIZE, GRID_HEIGHT * CELL_SIZE);
}

struct Rectangle {
    x: usize, 
    y: usize, 
    width:usize, 
    height: usize,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Grass,
    Sand,
    Water,
    Ground,
    Lava,
}

pub struct TileProperties {
    pub tile: Tile,
    pub traversible: bool,
}

impl Tile {
    pub fn elements() -> &'static [TileProperties] {
        static ELEMENTS: [TileProperties; 5] = [
            TileProperties { tile: Tile::Grass, traversible: true },
            TileProperties { tile: Tile::Sand, traversible: true },
            TileProperties { tile: Tile::Water, traversible: false },
            TileProperties { tile: Tile::Ground, traversible: true },
            TileProperties { tile: Tile::Grass, traversible: true },
        ];
        &ELEMENTS
    }

    pub fn properties(&self) -> &'static TileProperties {
        &Self::elements()[*self as usize]
    }
}

#[wasm_bindgen]
pub struct GameState {
    step_period: i32,
    tiles: [Rectangle; 5],
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        let mut game_state = GameState {
            step_period: 300,
            tiles: [
                Rectangle {x: 0, y: 0, width: CELL_SIZE, height: CELL_SIZE},
                Rectangle {x: CELL_SIZE, y: 0, width: CELL_SIZE, height: CELL_SIZE},
                Rectangle {x: 2 * CELL_SIZE, y: 0, width: CELL_SIZE, height: CELL_SIZE},
                Rectangle {x: 3 * CELL_SIZE, y: 0, width: CELL_SIZE, height: CELL_SIZE},
                Rectangle {x: 4 * CELL_SIZE, y: 0, width: CELL_SIZE, height: CELL_SIZE},
            ],
        };
        game_state
    }

    pub fn on_key_down(&mut self, code: u32) {

    }

    pub fn step(&mut self, _timestamp: i32) {
        self.repaint();
    }

    fn draw_tile(&self, tile: Tile, x: usize, y: usize) {
        let src = &self.tiles[tile as usize];
        canvas_draw_image(src.x, src.y, src.width, src.height, x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    }

    fn repaint(&self) {
        paint_background();
        self.draw_tile(Tile::Water, 0, 0);
        self.draw_tile(Tile::Grass, 1, 1);
        canvas_fill();
    }
}
