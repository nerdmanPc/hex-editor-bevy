use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct TerrainCell {
    entity: Option<Entity>,
    height: i32,
}

impl Default for TerrainCell {
    fn default() -> Self {
        Self {
            entity: None,
            height: 0,
        }
    }
}

impl TerrainCell {
    pub fn _with_entity(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            height: 0,
        }
    }

    pub fn add_height(&mut self, increment: i32) {
        self.height += increment;
        if self.height < 0 { self.height = 0 }
    }

    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
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