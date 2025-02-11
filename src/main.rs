use bevy::prelude::*;
//use bevy_mod_picking::prelude::*;

mod startup_systems; use startup_systems::*;
mod update_systems; use update_systems::*;
mod grid; use grid::*;

mod components;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            EditorPlugin,
        )).run();
}

#[derive(Resource)]
struct CellTemplates {
    pub empty_material: Handle<StandardMaterial>,
    pub filled_material: Handle<StandardMaterial>,
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
            ).chain())
            .add_systems(Update, (paint_grid, rotate_camera));
    }
}
