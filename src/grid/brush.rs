use bevy::prelude::*;

#[derive(Resource, Default, Clone, Copy)]
pub struct Brush {
    pub radius: u8,
    pub delta_height: i8,
}