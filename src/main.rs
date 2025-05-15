use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
//use bevy_mod_picking::prelude::*;

mod startup_systems; 
use startup_systems::*;
mod picking_systems;
mod ui; use ui::*;
mod grid; use grid::*;

mod components;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            PanOrbitCameraPlugin,
            HexEditorPlugin,
            EguiPlugin,
        )).run();
}

#[derive(Resource)]
struct CellTemplates {
    pub default_material: Handle<StandardMaterial>,
    pub hovered_material: Handle<StandardMaterial>,
}

pub struct HexEditorPlugin;

impl Plugin for HexEditorPlugin {
    fn build(&self, app: &mut App) {
        let mut empty_grid = Grid::default();
        empty_grid.make_hex([0, 0], 3); 
        app.insert_resource(empty_grid)
            .insert_resource(FilePicker::default())
            .add_systems(Startup, (
                spawn_cells,
                spawn_light,
                spawn_camera,
            ))
            .add_systems(Update,  draw_ui);
    }
}
