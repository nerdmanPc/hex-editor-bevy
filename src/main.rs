use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
//use bevy_mod_picking::prelude::*;

mod startup_systems; 
use startup_systems::*;
mod update_systems; use update_systems::*;
mod grid; use grid::*;

mod components;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            PanOrbitCameraPlugin,
            EditorPlugin,
        )).run();
}

#[derive(Resource)]
struct CellTemplates {
    pub default_material: Handle<StandardMaterial>,
    pub hovered_material: Handle<StandardMaterial>,
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        let mut empty_grid = Grid::default();
        empty_grid.make_hex([0, 0], 3); 
        app.insert_resource(empty_grid)
            .add_systems(Startup, (
                spawn_cells,
                spawn_light,
                spawn_camera,
            ));
            //.add_systems(Update,  rotate_camera);
    }
}
