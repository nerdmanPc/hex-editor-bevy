
use std::f32::consts::PI;
use bevy::prelude::*;

use crate::grid::*;
use crate::components::*;
use crate::CellTemplates;

pub fn paint_grid(click: Trigger<Pointer<Click>>, mut grid: ResMut<Grid>, mut query: Query<(&mut CellComponent, &mut Transform)>) {
    let query_result = query.get_mut(click.entity());
    let (grid_cell, mut transform) = query_result.unwrap();
    match click.button {
        PointerButton::Primary => { 
            grid.increment_height(*grid_cell, 1);
            transform.translation.y = grid.world_cell_height(*grid_cell) as f32;
         }
        PointerButton::Secondary => { 
            grid.increment_height(*grid_cell, -1);
            transform.translation.y = grid.world_cell_height(*grid_cell) as f32;
         }
        _ => {}
    }
}

pub fn highlight_cells(hover_event: Trigger<Pointer<Over>>, materials: Res<CellTemplates>, mut query: Query<&mut MeshMaterial3d<StandardMaterial>>) {
    let query_result = query.get_mut(hover_event.entity());
    let mut material = query_result.unwrap();
    *material = MeshMaterial3d(materials.hovered_material.clone());
}

pub fn un_highlight_cells(hover_event: Trigger<Pointer<Out>>, materials: Res<CellTemplates>, mut query: Query<&mut MeshMaterial3d<StandardMaterial>>) {
    let query_result = query.get_mut(hover_event.entity());
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