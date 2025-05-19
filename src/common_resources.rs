use bevy::prelude::*;

#[derive(Resource)]
pub struct CellTemplates {
    pub default_material: Handle<StandardMaterial>,
    pub hovered_material: Handle<StandardMaterial>,
    pub mesh: Handle<Mesh>
}