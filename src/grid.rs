use std::{
    collections::HashMap, iter::Map, vec::IntoIter
};
use bevy::prelude::*;

mod hex_utils; pub use hex_utils::*;

//Stores layout and adjacency information
#[derive(Resource)]
pub struct Grid {
    layout: Layout,
    data: HashMap<Hex, Option<Entity>>,
    //rotation: [f32; 2],
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
                    self.data.insert(key, None);
                }
            }
        }
        self
    }

    /*pub fn rotate(&mut self, amount: impl Into<[f32;2]>) {
        let amount: [f32; 2] = amount.into();
        self.rotation[0] += amount[0];
        self.rotation[1] += amount[1];
    }*/

    pub fn set_entity(&mut self, cell: impl Into<Hex>, entity: Entity) {
        self.data.insert(cell.into(), Some(entity));
    }

    pub fn delete_cell(&mut self, cell: impl Into<Hex>) {
        self.data.remove(&cell.into());
    }

    pub fn sample_cell(&self, pos: impl Into<Point>) -> Hex {

        let fractional_coord = LayoutTool::pixel_to_hex(self.layout, pos.into());
        fractional_coord.round()
    }

    pub fn build_mesh(&self) -> Vec<[f32;2]> {
        let cells = self.data.iter().map(|(hex, _entity)|{

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
            size: Point { x:0.1, y:0.1 }, 
            origin: Point { x: 0.0, y: 0.0 },
        };
        let data = HashMap::new();
        Self {
            layout,
            data,
        }
    }
}
