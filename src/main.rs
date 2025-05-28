use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
//use bevy_mod_picking::prelude::*;

mod startup_systems; 
use startup_systems::*;
mod ui; use ui::*;
mod grid; use grid::*;
mod common_resources; use common_resources::*;

mod picking_systems;
mod components;
mod commands;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            PanOrbitCameraPlugin,
            HexEditorPlugin,
            EguiPlugin {enable_multipass_for_primary_context: false},
        )).run();
}

pub struct HexEditorPlugin;

impl Plugin for HexEditorPlugin {
    fn build(&self, app: &mut App) {
        let mut empty_grid = Grid::default();
        empty_grid.make_hex([0, 0], 3); 
        app.insert_resource(empty_grid)
            .insert_resource(FileDialogWrapper::default())
            .insert_resource(CreationForm::default())
            .insert_resource(Brush::default())
            .add_systems(Startup, (
                init_cells,
                init_light,
                init_camera,
            ))
            .add_systems(Update,  draw_ui);
    }
}
