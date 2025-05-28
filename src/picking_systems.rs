use std::collections::HashSet;

use bevy::prelude::*;

use crate::commands::update_cells;
use crate::grid::*;
use crate::components::*;
use crate::common_resources::*;

pub fn paint_grid(click: Trigger<Pointer<Click>>, mut grid: ResMut<Grid>, brush: Res<Brush>, mut query: Query<(&mut CellComponent, &mut Transform)>) {
    let query_result = query.get_mut(click.target());
    let (grid_cell, _transform) = query_result.unwrap();
    let mut brush = *brush;
    match click.button {
        PointerButton::Primary => {
            brush.delta_height = 1;
         }
        PointerButton::Secondary => { 
            brush.delta_height = -1;
         }
        _ => {}
    }
    grid.apply_brush(*grid_cell, &brush);
    update_cells(&grid, &mut query);
}

pub fn highlight_cells(hover_event: Trigger<Pointer<Over>>, materials: Res<CellTemplates>, brush: Res<Brush>, mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &CellComponent)>) {

    let query_result = query.get(hover_event.target());
    let (_material, center_cell) = query_result.unwrap();
    let cell_keys: HashSet<Hex> = Grid::cells_in_hexagon(center_cell.hex_coords(), brush.radius).collect();
    for (mut material, cell) in  query.iter_mut() {
        let cell_key = cell.hex_coords();
        if cell_keys.contains(&cell_key) {
            *material = MeshMaterial3d(materials.hovered_material.clone());
        } else {
            *material = MeshMaterial3d(materials.default_material.clone());
        }
    }
}

pub fn un_highlight_cells(hover_event: Trigger<Pointer<Out>>, materials: Res<CellTemplates>, mut query: Query<&mut MeshMaterial3d<StandardMaterial>>) {
    let query_result = query.get_mut(hover_event.target());
    let mut material = query_result.unwrap();
    *material = MeshMaterial3d(materials.default_material.clone());
}

/*pub fn _rotate_camera(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Camera3d>>) {
    let rotate_left = input.just_pressed(KeyCode::ArrowLeft) as i32 as f32;
    let rotate_right = input.just_pressed(KeyCode::ArrowRight) as i32 as f32;
    let angle = (rotate_right- rotate_left) * 1./3. * PI;
    for mut transform in &mut query {
        transform.rotate_around(Vec3 {x: 0.0, y: 0.0, z: 0.0}, Quat::from_rotation_y(angle as f32));
    }
}*/