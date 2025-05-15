use bevy_egui::{
    egui,
    EguiContexts,
};

use bevy::prelude::*;
use egui_file::{DialogType, FileDialog};
use ciborium::{
    from_reader,
    into_writer,
};

use std::fs::File;

use crate::grid::*;

#[derive(Resource, Default)]
pub struct FilePicker(Option<FileDialog>);

pub fn draw_ui(mut ctx: EguiContexts, mut file_dialog: ResMut<FilePicker>, mut grid: ResMut<Grid>) {
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
        todo!()
    }

    if let FilePicker(Some(dialog)) = &mut (*file_dialog) {
        dialog.show(ctx.ctx_mut());
        if let Some(path) = dialog.path() {
            match dialog.dialog_type() {
                DialogType::OpenFile if dialog.selected() => {
                    let file =  File::open(path).expect("Failed to open file!");
                    *grid = from_reader(file).expect("Failed deserialize grid!");
                }
                DialogType::SaveFile if dialog.selected() => {
                    let file =  File::create(path).expect("Failed to save file!");
                    into_writer( &(*grid), file).expect("Failed to serialize grid!");
                }
                _ => {}
            }
        }
    }
}