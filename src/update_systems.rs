
use std::f32::consts::PI;
use bevy::prelude::*;

use crate::grid::*;
use crate::components::*;

pub fn paint_grid(mut grid: ResMut<Grid>, mut query: Query<(&mut CellComponent, &mut Transform)>) {
    for (mut cell_component, mut transform) in &mut query {
        if cell_component.get_painted() {
            grid.increment_height(*cell_component, 1);
            transform.translation.y = grid.world_cell_height(*cell_component) as f32;
            //*material = cell_templates.filled_material.clone();
            //grid.set_entity(*cell, entity);
        }
        if cell_component.get_erased() {
            grid.increment_height(*cell_component, -1);
            transform.translation.y = grid.world_cell_height(*cell_component) as f32;
            //*material = cell_templates.empty_material.clone();
            //grid.delete_cell(*cell);
        }
    }
}

pub fn on_click_cell(click: Trigger<Pointer<Click>>, mut query: Query<&mut CellComponent>) {
    let query_result = query.get_mut(click.entity());
    if query_result.is_err() {
        return;
    }
    let mut grid_cell = query_result.unwrap();
    match click.button {
        PointerButton::Primary => { grid_cell.on_click(); }
        PointerButton::Secondary => { grid_cell.on_right_click(); }
        _ => {}
    }
}

pub fn rotate_camera(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Camera3d>>) {
    let rotate_left = input.just_pressed(KeyCode::ArrowLeft) as i32 as f32;
    let rotate_right = input.just_pressed(KeyCode::ArrowRight) as i32 as f32;
    let angle = (rotate_right- rotate_left) * 1./3. * PI;
    for mut transform in &mut query {
        transform.rotate_around(Vec3 {x: 0.0, y: 0.0, z: 0.0}, Quat::from_rotation_y(angle as f32));
    }
}