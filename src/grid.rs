use std::{
    collections::HashMap, iter::Map, vec::IntoIter, collections::hash_map::Keys, iter::Cloned,
};
use bevy::prelude::*;

mod hex_utils; pub use hex_utils::*;
mod cell_entry; pub use cell_entry::*;
use hexagon::FractionalHex;

//Stores layout and adjacency information
#[derive(Resource, Clone, Debug)]
pub struct Grid {
    layout: Layout,
    terrain: HashMap<Hex, TerrainCell>,
    //tiles: HashMap<FractionalHex, TerrainTile>
}

impl Grid {

    /*pub fn _make_rhombus(min: impl Into<Hex>, max: impl Into<Hex>) -> Self {
        let (min, max): (Hex, Hex) = (min.into(), max.into());
        let mut instance = Self::default();
        for q in min.q() ..= max.q() {
            for r in min.r() ..= max.r() {
                let key = Hex::new(q, r);
                instance.data.insert(key, None);
            }
        }
        instance
    }

    pub fn _make_triangle(min: impl Into<Hex>, size: i32) -> Self {
        let min: Hex = min.into();
        let mut instance = Self::default();
        for q in min.q() ..=  min.q() + size {
            for r in min.r() ..= min.r() + size - q {
                let key = Hex::new(q, r);
                instance.data.insert(key, None);
            }
        }
        instance
    }*/

    pub fn make_hex(&mut self, center: impl Into<Hex>, size: i32) -> &mut Self {
        let center: Hex = center.into();
        //let mut instance = Self::default();
        for q in -size ..= size {
            for r in -size ..= size {
                let s = -q-r;
                if (-size <= s) && (s <= size) {
                    let key = center.add(Hex::new(q, r));
                    self.terrain.insert(key, TerrainCell::default());
                }
            }
        }
        //TODO init 
        self
    }

    pub fn set_entity(&mut self, cell_id: impl Into<Hex> + Clone, entity: Entity) {
        let opt_cell = self.terrain.get_mut(&cell_id.clone().into());
        if let Some(cell) = opt_cell {
            cell.set_entity(entity);
            return;
        }
        //let entity = Some(entity);
        //self.data.insert(cell_id.into(), Cell::with_entity(entity));
    }

    pub fn _delete_cell(&mut self, cell: impl Into<Hex>) {
        self.terrain.remove(&cell.into());
    }

    pub fn _sample_cell(&self, pos: impl Into<Point>) -> Hex {

        let fractional_coord = LayoutTool::pixel_to_hex(self.layout, pos.into());
        fractional_coord.round()
    }

    pub fn cell_coords<'a>(&'a self) -> Cloned<Keys<'_, Hex, TerrainCell>>  {
        self.terrain.keys().cloned()
    }

    pub fn world_coords_from_hex<'a>(&'a self, hex_coords: impl Into<Hex>) -> Point {
        LayoutTool::hex_to_pixel(self.layout, hex_coords.into())
    }

    pub fn world_cell_height(&self, cell_id: impl Into<Hex>) -> f64 {
        let cell_id = cell_id.into();
        self.terrain.get(&cell_id).expect("This is a bug!").height() as f64 * self.layout.height
    }

    pub fn increment_height(&mut self, cell_id: impl Into<Hex>, delta_height: i32) {
        let cell_id = cell_id.into();
        let cell = self.terrain.get_mut(&cell_id).expect("This is a bug!");
        //print!("Cell height before: {}\n", cell.height);
        cell.add_height(delta_height);
        //print!("Cell height after: {}\n", cell.height);
    }

    pub fn _build_mesh(&self) -> Vec<[f32;2]> {
        let cells = self.terrain.iter().map(|(hex, _entity)|{

            let mut points = LayoutTool::polygon_corners(self.layout, *hex);
            points.push(points[0]);
            let midpoint = points.iter().fold(Point{ x: 0.0, y: 0.0}, |acc, elem| { acc + *elem }) / points.len() as f64;
            points.insert(0, midpoint);
            let points: Vec<_> = points
                .iter()
                .map(|Point{x, y}| {[*x as f32, *y as f32]})
                .collect();
            points
        });
        let mesh = cells.reduce(|mut acc, mut item| {
            acc.append(&mut item);
            acc
        });
        if let Some(result) = mesh {
            return result;
        } else {
            return Vec::new();
        }
    }

    fn _polygon_corners(&self, key: Hex) -> Map<IntoIter<Point>, fn(Point)->[f32; 2]>{

        let convert_point: fn(Point) -> [f32; 2] = |point: Point| {
            [point.x as f32, point.y as f32]
        };
        LayoutTool::polygon_corners(self.layout, key).into_iter().map(convert_point)
    }
}


impl Default for Grid {
    fn default() -> Self {
        let layout = Layout {
            orientation: LAYOUT_ORIENTATION_POINTY,
            size: Point { x:1.0, y:1.0 }, 
            origin: Point { x: 0.0, y: 0.0 },
            height: 0.25,
        };
        let terrain = HashMap::new();
        //let tiles = HashMap::new();
        Self {
            layout,
            terrain,
            //tiles,
        }
    }
}
