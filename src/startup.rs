use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy_mod_picking::prelude::*;

use crate::grid::*;
use crate::cell_component::*;
use crate::CellTemplates;

pub fn spawn_cells(mut commands: Commands, mut grid: ResMut<Grid>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    let mesh_handle = meshes.add(Sphere::new(0.1));
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
    
    let cell_keys: Vec<Hex> = grid.cell_keys().collect();
    for cell_key in cell_keys {
        let world_coord = grid.hex_to_point(cell_key);
        let mut spawn_commands = commands
            .spawn((PbrBundle {
                mesh: mesh_handle.clone(),
                material: filled_material.clone(),
                transform: Transform::from_xyz(world_coord.x as f32, 0.0, world_coord.y as f32),
                ..default()
            },
            CellComponent::with_coords(cell_key),
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
        grid.set_entity(cell_key, entiy);
    }
}

pub fn spawn_tiles(mut commands: Commands, grid: ResMut<Grid>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    let filled_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });
    let tile_points = grid.tile_points();
    let tile_points: Vec<Vec3> = tile_points.into_iter().map(|point| {
        Vec3 { x:point.x as f32, y:0.0, z:point.y as f32 }
    }).collect();
    let triangle_a = vec![
        0, 1, 2,
    ];
    let triangle_b = vec![
        0, 2, 3,
    ];
    let mesh_a = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, tile_points.clone())
        .with_inserted_indices(Indices::U16(triangle_a))
        .with_computed_normals();
    let mesh_a = meshes.add(mesh_a);
    let mesh_b = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, tile_points)
        .with_inserted_indices(Indices::U16(triangle_b))
        .with_computed_normals();
    let mesh_b = meshes.add(mesh_b);

    for cell_key in grid.cell_keys() {
        let world_coord = grid.hex_to_point(cell_key);
        let has_tile_neighbors = grid.has_neighbor(cell_key, 0) && grid.has_neighbor(cell_key, 1);
        if !has_tile_neighbors { continue; }
        commands.spawn((PbrBundle {
                mesh: mesh_a.clone(),
                material: filled_material.clone(),
                transform: Transform::from_xyz(world_coord.x as f32, 0.0, world_coord.y as f32),
                ..default()
            },
            TileComponent::new(cell_key, 0)
        ));
    }
    for cell_key in grid.cell_keys() {
        let world_coord = grid.hex_to_point(cell_key);
        let has_tile_neighbors = grid.has_neighbor(cell_key, 1) && grid.has_neighbor(cell_key, 2);
        if !has_tile_neighbors { continue; }
        commands.spawn((PbrBundle {
                mesh: mesh_b.clone(),
                material: filled_material.clone(),
                transform: Transform::from_xyz(world_coord.x as f32, 0.0, world_coord.y as f32),
                ..default()
            },
            TileComponent::new(cell_key, 0)
        ));
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