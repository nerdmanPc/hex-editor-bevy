use bevy::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TerrainCell {
    //entity: Option<Entity>,
    height: i32,
}

impl Default for TerrainCell {
    fn default() -> Self {
        Self {
            //entity: None,
            height: 0,
        }
    }
}

impl TerrainCell {

    pub fn add_height(&mut self, increment: i32) {
        self.height += increment;
        if self.height < 0 { self.height = 0 }
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

pub enum ReliefType {
    Flat,
    Hill,
    Cliff,
}