use bevy_egui::{
    egui,
    EguiContexts,
};

use bevy::{prelude::*};
use egui_file::{DialogType, FileDialog};

use crate::{
    commands::*, common_resources::CellTemplates, components::*, grid::*,
};

#[derive(Resource, Default)]
pub struct FilePicker(Option<FileDialog>);

#[derive(Resource, Default)]
pub struct CreationForm{
    pub open: bool,
    pub radius: u16,
}

pub fn draw_ui(
    mut commands: Commands, 
    cell_template: Res<CellTemplates>,
    mut grid: ResMut<Grid>, 
    cell_components: Query<(Entity, &CellComponent)>,
    mut ctx: EguiContexts, 
    mut file_dialog: ResMut<FilePicker>, 
    mut create_dialog: ResMut<CreationForm>
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
        let mut inner_dialog = FileDialog::open_file(None);
        inner_dialog.open();
        *file_dialog = FilePicker(Some(inner_dialog));
    } else if clicked_save {
        let mut inner_dialog = FileDialog::save_file(None);
        inner_dialog.open();
        *file_dialog = FilePicker(Some(inner_dialog));
    } else if clicked_new {
        create_dialog.open = true;
    }
    if create_dialog.open {
        egui::Window::new("New Grid").show(ctx.ctx_mut(), |ui| {
            ui.label("Grid Radius");
            ui.add(egui::Slider::new(&mut create_dialog.radius, 1..=32));
            if ui.button("Create").clicked() {
                let grid_radius = create_dialog.radius;
                *grid = new_grid(grid_radius, &mut commands, &cell_template.mesh, &cell_template.default_material, &cell_components);
                create_dialog.open = false;
            }
        });
    }

    if let FilePicker(Some(inner_dialog)) = &mut (*file_dialog) {
        inner_dialog.show(ctx.ctx_mut());
        if let Some(path) = inner_dialog.path() {
            match inner_dialog.dialog_type() {
                DialogType::OpenFile if inner_dialog.selected() => {
                    *grid = load_grid(path, &mut commands, &cell_components, &cell_template.mesh, &cell_template.default_material);
                }
                DialogType::SaveFile if inner_dialog.selected() => {
                    save_grid(path, &grid);
                }
                _ => {}
            }
        }
    }

}