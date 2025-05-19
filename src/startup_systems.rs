use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::grid::*;
use crate::components::*;
use crate::picking_systems::*;
use crate::commands::*;
use crate::common_resources::CellTemplates;

pub fn init_cells(mut commands: Commands, grid: Res<Grid>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    let mesh_handle = create_cell_mesh(&grid, &mut meshes);
    let (default_material, hovered_material) = create_cell_materials(&mut materials);
    commands.insert_resource(CellTemplates{
        default_material: default_material.clone(),
        hovered_material: hovered_material.clone(),
        mesh: mesh_handle.clone(),
    });
    create_cells(&mut commands, &grid, &mesh_handle, &default_material);
}

fn create_cell_materials(materials: &mut ResMut<Assets<StandardMaterial>>) -> (Handle<StandardMaterial>, Handle<StandardMaterial>) {
    let default_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(1.0, 1.0, 1.0, 0.1),
        alpha_mode: AlphaMode::Add,
        ..default()
    });
    let hovered_material = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(1.0, 1.0, 1.0, 0.5),
        alpha_mode: AlphaMode::Add,
        ..default()
    });
    (default_material, hovered_material)
}

fn create_cell_mesh(grid: &Res<Grid>, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    let mesh_info = grid.cell_mesh();
    let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
        .with_inserted_indices(Indices::U16(mesh_info.indices));
    let mesh_handle = meshes.add(mesh);
    mesh_handle
}

/*pub fn spawn_tiles(mut commands: Commands, grid: ResMut<Grid>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });
    
    let tile_points = grid.tile_points();
    let tile_points: Vec<Vec3> = tile_points.into_iter().map(|point| {
        Vec3 { x: point.x as f32, y:0.0, z: point.y as f32 }
    }).collect();

    let triangle_a = vec![0, 1, 2];
    let neighbors_a = [0, 1];
    let mesh_a = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, tile_points.clone())
        .with_inserted_indices(Indices::U16(triangle_a))
        .with_computed_normals();
    let mesh_a = meshes.add(mesh_a);
    spawn_tile_group(&mut commands, &grid, MeshMaterial3d(material.clone()), Mesh3d(mesh_a), neighbors_a, 0);

    let triangle_b = vec![0, 2, 3];
    let neighbors_b = [1, 2];
    let mesh_b = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, tile_points)
        .with_inserted_indices(Indices::U16(triangle_b))
        .with_computed_normals();
    let mesh_b = meshes.add(mesh_b);
    spawn_tile_group(&mut commands, &grid, MeshMaterial3d(material), Mesh3d(mesh_b), neighbors_b, 1);

}*/

/*fn spawn_tile_group(commands: &mut Commands, grid: &ResMut<Grid>, material: MeshMaterial3d<StandardMaterial>, mesh: Mesh3d, neighbors: [u8; 2], tile_id: i32) {
    for cell_key in grid.cell_keys() {
        let world_coord = grid.hex_to_point(cell_key);
        let has_tile_neighbors = grid.has_neighbor(cell_key, neighbors[0]) && grid.has_neighbor(cell_key, neighbors[1]);
        if !has_tile_neighbors { continue; }
        let transform = Transform::from_xyz(world_coord.x as f32, 0.0, world_coord.y as f32);
        let tile_component = TileComponent::new(cell_key, tile_id);
        commands.spawn((mesh.clone(), material.clone(), transform, tile_component));
    }
}*/

pub fn init_light(mut commands: Commands) {
    let point_light = PointLight {
        shadows_enabled: true,
        intensity: 10_000_000.,
        range: 100.0,
        shadow_depth_bias: 0.2,
        ..default()
    };
    let transform = Transform::from_xyz(8.0, 16.0, 8.0);
    commands.spawn((point_light, transform));
}

pub fn init_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 7.0, 22.0)
        .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
    commands.spawn((PanOrbitCamera::default(), transform));
}