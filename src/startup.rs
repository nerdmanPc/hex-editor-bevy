use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::grid::*;
use crate::cell_component::CellComponent;
use crate::CellTemplates;

pub fn spawn_cells(mut commands: Commands, mut grid: ResMut<Grid>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    let cell_coords: Vec<Hex> = grid.cell_coords().collect();
    let mesh_handle = meshes.add(Sphere::new(0.5));
    let empty_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(1.0, 1.0, 1.0, 0.0),
        alpha_mode: AlphaMode::AlphaToCoverage,
        ..default()
    });
    let filled_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands.insert_resource(CellTemplates{
        empty_material: empty_material.clone(),
        filled_material: filled_material.clone(),
    });

    for cell_coord in cell_coords {
        let world_coord = grid.world_coords_from_hex(cell_coord);
        let mut spawn_commands = commands
            .spawn((PbrBundle {
                mesh: mesh_handle.clone(),
                material: filled_material.clone(),
                transform: Transform::from_xyz(world_coord.x as f32, 0.0, world_coord.y as f32),
                ..default()
            },
            CellComponent::with_coords(cell_coord),
            //PickableBundle::default(),
        ));

        let spawn_commands = spawn_commands
            .insert(On::<Pointer<Click>>::target_component_mut::<CellComponent>(|click, grid_cell|{
                match click.button {
                    PointerButton::Primary => { grid_cell.on_click(); }
                    PointerButton::Secondary => { grid_cell.on_right_click(); }
                    _ => {}
                }
            }));
        
        let entiy = spawn_commands.id();
        grid.set_entity(cell_coord, entiy);
    }
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

pub fn spawn_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 7.0, 22.0)
        .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
}