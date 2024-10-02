use bevy::prelude::*;


#[derive(Clone, Copy, Debug)]
pub struct Cell {
    entity: Option<Entity>,
    height: i32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            entity: None,
            height: 0,
        }
    }
}

impl Cell {
    pub fn _with_entity(entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            height: 0,
        }
    }

    pub fn add_height(&mut self, increment: i32) {
        self.height += increment;
    }

    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}