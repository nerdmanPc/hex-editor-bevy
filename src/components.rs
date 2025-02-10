use crate::grid::*;
use  bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub struct CellComponent{
    hex_coord: Hex,
    painted: bool,
    erased: bool,
}

impl Default for CellComponent {
    fn default() -> Self {
        let hex_coord = [0, 0].into();
        Self {
            hex_coord,
            painted: false,
            erased: false,
        }
    }
}

impl Into<Hex> for CellComponent {
    fn into(self) -> Hex {
        self.hex_coord
    }
}

impl CellComponent {
    pub fn with_coords(coords: impl Into<Hex>) -> Self {
        let position = coords.into();
        Self {
            hex_coord: position,
            painted: false,
            erased: false,
        }
    }

    pub fn on_click(&mut self) {
        self.painted = true
    }
    pub fn on_right_click(&mut self) {
        self.erased = true
    }

    pub fn get_painted(&mut self) -> bool {
        let painted = self.painted;
        self.painted = false;
        painted
    }

    pub fn get_erased(&mut self) -> bool {
        let erased = self.erased;
        self.erased = false;
        erased
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct TileComponent{
    hex_coord: Hex,
    tile_id: u8,
}

impl Into<Hex> for TileComponent {
    fn into(self) -> Hex {
        self.hex_coord
    }
}

impl TileComponent {
    pub fn new(coords: impl Into<Hex>, tile_id: i32) -> Self {
        let position = coords.into();
        Self {
            hex_coord: position,
            tile_id: (tile_id % 2) as u8
        }
    }
}