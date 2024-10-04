use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

mod startup; use startup::*;
mod grid; use grid::*;
mod cell_component; use cell_component::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EditorPlugin,
            DefaultPickingPlugins,
        )).run();
}

#[derive(Resource)]
struct CellTemplates {
    pub empty_material: Handle<StandardMaterial>,
    pub filled_material: Handle<StandardMaterial>,
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        let mut empty_grid = Grid::default();
        empty_grid.make_hex([0, 0], 8); 
        app.insert_resource(empty_grid)
            .add_systems(Startup, (
                spawn_cells,
                spawn_light,
                spawn_camera,
            ))
            .add_systems(Update, (paint_grid, rotate_camera));
    }
}

fn paint_grid(mut grid: ResMut<Grid>, mut query: Query<(&mut CellComponent,  &mut Handle<StandardMaterial>, &mut Transform)>) {
    for (mut cell_component, mut material, mut transform) in &mut query {
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

fn rotate_camera(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Camera3d>>) {
    let rotate_left = input.just_pressed(KeyCode::ArrowLeft) as i32 as f32;
    let rotate_right = input.just_pressed(KeyCode::ArrowRight) as i32 as f32;
    let angle = (rotate_right- rotate_left) * 1./3. * PI;
    for mut transform in &mut query {
        transform.rotate_around(Vec3 {x: 0.0, y: 0.0, z: 0.0}, Quat::from_rotation_y(angle as f32));
    }
}