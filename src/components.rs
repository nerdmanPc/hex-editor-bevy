use crate::grid::*;
use  bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub struct CellComponent{
    hex_coord: Hex,
}

impl Default for CellComponent {
    fn default() -> Self {
        let hex_coord = [0, 0].into();
        Self {
            hex_coord,
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
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct TileComponent{
    hex_coord: Hex,
}

impl Into<Hex> for TileComponent {
    fn into(self) -> Hex {
        self.hex_coord
    }
}

impl TileComponent {
    pub fn new(coords: impl Into<Hex>) -> Self {
        let position = coords.into();
        Self {
            hex_coord: position,
        }
    }
}