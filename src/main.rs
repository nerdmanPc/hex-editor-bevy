use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
//use bevy_mod_picking::prelude::*;

mod startup_systems; 
use startup_systems::*;
mod ui; use ui::*;
mod grid; use grid::*;

mod picking_systems;
mod components;
mod commands;
mod common_resources;


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

pub struct HexEditorPlugin;

impl Plugin for HexEditorPlugin {
    fn build(&self, app: &mut App) {
        let mut empty_grid = Grid::default();
        empty_grid.make_hex([0, 0], 3); 
        app.insert_resource(empty_grid)
            .insert_resource(FilePicker::default())
            .add_systems(Startup, (
                init_cells,
                init_light,
                init_camera,
            ))
            .add_systems(Update,  draw_ui);
    }
}
