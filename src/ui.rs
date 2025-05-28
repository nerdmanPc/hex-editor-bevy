use bevy_egui::{
    egui,
    EguiContexts,
};

use bevy::{prelude::*};
use egui_file::{DialogType, FileDialog};

use crate::{
    commands::*, common_resources::*, components::*, grid::*,
};

#[derive(Resource, Default)]
pub struct FileDialogWrapper(Option<FileDialog>);

#[derive(Resource, Default)]
pub struct CreationForm{
    pub open: bool,
    pub radius: u16,
}

pub fn draw_ui(
    mut commands: Commands, 
    cell_template: Res<CellTemplates>,
    mut grid: ResMut<Grid>,
    mut brush: ResMut<Brush>,
    cell_components: Query<(Entity, &CellComponent)>,
    mut file_dialog: ResMut<FileDialogWrapper>, 
    mut create_dialog: ResMut<CreationForm>,
    mut ctx: EguiContexts, 
){
    draw_side_panel(&mut ctx, &mut brush);
    draw_top_panel(&mut ctx, &mut file_dialog, &mut create_dialog);
    draw_create_dialog(&mut commands, &cell_template, &mut grid, cell_components, &mut ctx, create_dialog);
    draw_load_save_dialog(commands, cell_template, grid, cell_components, ctx, file_dialog);
}

fn draw_side_panel(ctx: &mut EguiContexts, brush: &mut ResMut<Brush>) {
    egui::SidePanel::right("side_panel")
        .default_width(128.0)
        .show(ctx.ctx_mut(), |ui| {
            ui.label("Brush Size");
            ui.add(egui::Slider::new(&mut brush.radius, 0..=3));
            ui.separator();
            ui.label("Controls:");
            ui.label("Left Click: Raise Height");
            ui.label("Right Click: Lower Height");
        });
}

fn draw_top_panel(ctx: &mut EguiContexts, file_dialog: &mut ResMut<FileDialogWrapper>, create_dialog: &mut ResMut<CreationForm>) {
    let mut clicked_new = false;
    let mut clicked_load = false;
    let mut clicked_save = false;
    egui::TopBottomPanel::top("top_panel")
        .default_height(128.0)
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
        **file_dialog = FileDialogWrapper(Some(inner_dialog));
    } else if clicked_save {
        let mut inner_dialog = FileDialog::save_file(None);
        inner_dialog.open();
        **file_dialog = FileDialogWrapper(Some(inner_dialog));
    } else if clicked_new {
        create_dialog.open = true;
    }
}

fn draw_load_save_dialog(
    mut commands: Commands, 
    cell_template: Res<CellTemplates>, 
    mut grid: ResMut<Grid>, 
    cell_components: Query<(Entity, &CellComponent)>, 
    mut ctx: EguiContexts, 
    mut file_dialog: ResMut<FileDialogWrapper>
) {
    if let FileDialogWrapper(Some(inner_dialog)) = &mut (*file_dialog) {
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

fn draw_create_dialog(
    commands: &mut Commands, 
    cell_template: &Res<CellTemplates>, 
    grid: &mut ResMut<Grid>, 
    cell_components: Query<(Entity, &CellComponent)>, 
    ctx: &mut EguiContexts, 
    mut create_dialog: ResMut<CreationForm>
) {
    if create_dialog.open {
        egui::Window::new("New Grid").show(ctx.ctx_mut(), |ui| {
            ui.label("Grid Radius");
            ui.add(egui::Slider::new(&mut create_dialog.radius, 0..=16));
            if ui.button("Create").clicked() {
                let grid_radius = create_dialog.radius;
                **grid = new_grid(grid_radius, commands, &cell_template.mesh, &cell_template.default_material, &cell_components);
                create_dialog.open = false;
            }
        });
    }
}