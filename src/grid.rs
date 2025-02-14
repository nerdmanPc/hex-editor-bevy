use std::{
    collections::HashMap, collections::hash_map::Keys, iter::Cloned,
};
use bevy::prelude::Resource;
pub use hexx::{
    Vec2,
    Hex,
    MeshInfo,
};
use hexx::*;
mod cell_entry; pub use cell_entry::*;

//Stores layout and adjacency information
#[derive(Resource, Clone, Debug)]
pub struct Grid {
    layout: HexLayout,
    height: f32,
    cells: HashMap<Hex, TerrainCell>,
}

impl Grid {

    pub fn _make_rhombus(min: impl Into<Hex>, max: impl Into<Hex>) -> Self {
        let (min, max): (Hex, Hex) = (min.into(), max.into());
        let mut instance = Self::default();
        for q in min.x ..= max.x {
            for r in min.y ..= max.y {
                let key = Hex::new(q, r);
                instance.cells.insert(key, TerrainCell::default());
            }
        }
        instance
    }

    pub fn _make_triangle(min: impl Into<Hex>, size: i32) -> Self {
        let min: Hex = min.into();
        let mut instance = Self::default();
        for q in min.x ..=  min.x + size {
            for r in min.y ..= min.y + size - q {
                let key = Hex::new(q, r);
                instance.cells.insert(key, TerrainCell::default());
            }
        }
        instance
    }

    pub fn make_hex(&mut self, center: impl Into<Hex>, size: i32) -> &mut Self {
        let center: Hex = center.into();
        //let mut instance = Self::default();
        for q in -size ..= size {
            for r in -size ..= size {
                let s = -q-r;
                if (-size <= s) && (s <= size) {
                    let key = center + Hex::new(q, r);
                    self.cells.insert(key, TerrainCell::default());
                }
            }
        }
        self
    }

    pub fn _delete_cell(&mut self, cell: impl Into<Hex>) {
        self.cells.remove(&cell.into());
    }

    pub fn cell_keys<'a>(&'a self) -> Cloned<Keys<'a, Hex, TerrainCell>>  {
        self.cells.keys().cloned()
    }

    pub fn hex_to_point<'a>(&'a self, hex_coords: impl Into<Hex>) -> Vec2 {
        self.layout.hex_to_world_pos(hex_coords.into())
    }

    pub fn world_cell_height(&self, cell_id: impl Into<Hex>) -> f32 {
        let cell_id = cell_id.into();
        self.cells.get(&cell_id).expect("This is a bug!").height() as f32 * self.height
    }

    pub fn increment_height(&mut self, cell_id: impl Into<Hex>, delta_height: i32) {
        let cell_id = cell_id.into();
        let cell = self.cells.get_mut(&cell_id).expect("This is a bug!");
        //print!("Cell height before: {}\n", cell.height);
        cell.add_height(delta_height);
        //print!("Cell height after: {}\n", cell.height);
    }

    pub fn hex_adjacent(hex: impl Into<Hex>, neighbor_id: u8) -> Hex {
        if neighbor_id > 5 { panic!("Invalid hex neighbor!") }
        let hex = hex.into();
        hex + Self::hex_direction(neighbor_id)
        //HexDirection::neighbor(hex, neighbor_id as i32)
    }

    pub fn hex_direction(direction_id: u8) -> Hex {
        if direction_id > 5 { panic!("Invalid direction!") }
        EdgeDirection::ALL_DIRECTIONS[direction_id as usize].into()
        //HexDirection::direction(direction_id as i32)
    }

    pub fn has_neighbor(&self, hex: impl Into<Hex>, neighbor_id: u8) -> bool {
        if neighbor_id > 5 { panic!("Invalid hex neighbor!") }
        let hex = hex.into();
        let adjacent_key = Self::hex_adjacent(hex, neighbor_id);//HexDirection::neighbor(hex, neighbor_id as i32);
        self.cells.contains_key(&adjacent_key)
    }

    pub fn cell_mesh(&self) -> MeshInfo {
        ColumnMeshBuilder::new(&self.layout, 32.0 * self.height)
            .without_bottom_face()
            .with_offset(Vec3 { x: 0.0, y: -32.0 * self.height, z: 0.0 })
            .build()
    }

    /*pub fn tile_points(&self) -> [Vec2; 4] {
        let hexes = [
            Hex::new(0, 0),
            Self::hex_direction(0),
            Self::hex_direction(1),
            Self::hex_direction(2),
        ];
        let points = [
            self.hex_to_point(hexes[0]),
            self.hex_to_point(hexes[1]),
            self.hex_to_point(hexes[2]),
            self.hex_to_point(hexes[3]),
        ];
        points
    }*/
}


impl Default for Grid {
    fn default() -> Self {
        let layout = HexLayout {
            orientation: HexOrientation::Pointy,
            scale: Vec2::new(1.0, 1.0), 
            ..Default::default()
        };
        let terrain = HashMap::new();
        Self {
            layout,
            height: 0.5,
            cells: terrain,
        }
    }
}
