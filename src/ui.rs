use bevy_egui::{
    egui,
    EguiContexts,
};

use bevy::{prelude::*};
use egui_file::{DialogType, FileDialog};
use ciborium::{
    from_reader,
    into_writer,
};

use std::fs::File;

use crate::{
    commands::*, common_resources::CellTemplates, components::*, grid::*
};

#[derive(Resource, Default)]
pub struct FilePicker(Option<FileDialog>);

pub fn draw_ui(
    mut commands: Commands, 
    cell_template: Res<CellTemplates>,
    mut grid: ResMut<Grid>, 
    cell_components: Query<(Entity, &CellComponent)>,
    mut ctx: EguiContexts, 
    mut file_dialog: ResMut<FilePicker>, 
){
    let mut clicked_new = false;
    let mut clicked_load = false;
    let mut clicked_save = false;
    egui::TopBottomPanel::top("top_panel")
        .default_height(64.0)
        .show(ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                clicked_new = ui.button("New").clicked();
                clicked_load = ui.button("Load").clicked();
                clicked_save = ui.button("Save").clicked();
            })
        });
    if clicked_load {
        let mut dialog = FileDialog::open_file(None);
        dialog.open();
        *file_dialog = FilePicker(Some(dialog));
    } else if clicked_save {
        let mut dialog = FileDialog::save_file(None);
        dialog.open();
        *file_dialog = FilePicker(Some(dialog));
    } else if clicked_new {
        let grid_radius = 16;
        let mesh_handle = &cell_template.mesh;
        let default_material = &cell_template.default_material;
        *grid = new_grid(grid_radius, &mut commands, mesh_handle, default_material, &cell_components)
    }

    if let FilePicker(Some(dialog)) = &mut (*file_dialog) {
        dialog.show(ctx.ctx_mut());
        if let Some(path) = dialog.path() {
            match dialog.dialog_type() {
                DialogType::OpenFile if dialog.selected() => {
                    *grid = load_grid(path, &mut commands, &cell_components, &cell_template.mesh, &cell_template.default_material);
                }
                DialogType::SaveFile if dialog.selected() => {
                    save_grid(path, &grid);
                }
                _ => {}
            }
        }
    }
}