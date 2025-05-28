use std::path::Path;
use crate::grid::*;
use bevy::prelude::*;
use std::fs::File;
use ciborium::{
    from_reader,
    into_writer,
};

use crate::components::*;
use crate::picking_systems::*;

pub fn new_grid(radius: u16, commands: &mut Commands, mesh_handle: &Handle<Mesh>, default_material: &Handle<StandardMaterial>, cell_components: &Query<(Entity, &CellComponent)>) -> Grid {
    let mut grid = Grid::default();
    grid.make_hex([0, 0], radius);
    delete_cells(commands, &cell_components);
    create_cells(commands, &mut grid, mesh_handle, default_material);
    grid
}

pub fn save_grid(path: &Path, grid: &Grid) {
    let file =  File::create(path).expect("Failed to save file!");
    into_writer( &(*grid), file).expect("Failed to serialize grid!");
}

pub fn load_grid(path: &Path, commands: &mut Commands, cell_components: &Query<(Entity, &CellComponent)>, mesh_handle: &Handle<Mesh>, default_material: &Handle<StandardMaterial>) -> Grid {
    let file =  File::open(path).expect("Failed to open file!");
    let grid = from_reader(file).expect("Failed deserialize grid!");
    delete_cells(commands, cell_components);
    create_cells(commands, &grid, mesh_handle, default_material);
    grid
}

pub fn create_cells(commands: &mut Commands, grid: &Grid, mesh_handle: &Handle<Mesh>, default_material: &Handle<StandardMaterial>) {
    let cell_keys = grid.cell_keys();
    for cell_key in cell_keys {
        let world_coord = grid.hex_to_point(cell_key);
        let transform = Transform::from_xyz(world_coord.x as f32, grid.world_cell_height(cell_key), world_coord.y as f32);
        let cell_component = CellComponent::with_coords(cell_key);
        let mut spawn_commands = commands
            .spawn((transform, cell_component, MeshMaterial3d(default_material.clone()), Mesh3d(mesh_handle.clone())));
        spawn_commands.observe(paint_grid).observe(highlight_cells);
    }
}

fn delete_cells(commands: &mut Commands, cell_components: &Query<(Entity, &CellComponent)>) {
    //let cell_keys = grid.cell_keys();
    for (entity, _cell_component) in cell_components.iter() {
        commands.entity(entity).despawn();
    }

}

pub fn update_cells<'a, 'b>(grid: &Grid, mut cells: &mut Query<(&mut  CellComponent, &mut Transform)>) {
    for (cell_component, mut transform) in cells {
        let cell_key = cell_component.hex_coords();
        transform.translation.y = grid.world_cell_height(cell_key);
    }
}