use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
mod grid; use grid::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EditorPlugin,
            DefaultPickingPlugins,
        )).run();
}

#[derive(Component, Copy, Clone, Debug)]
struct GridCell{
    pub hex_coord: Hex,
    clicked: bool
}

impl Default for GridCell {
    fn default() -> Self {
        let hex_coord = [0, 0].into();
        Self {
            hex_coord,
            clicked: false,
        }
    }
}

impl Into<Hex> for GridCell {
    fn into(self) -> Hex {
        self.hex_coord
    }
}

impl GridCell {
    pub fn on_click(&mut self) {
        self.clicked = true
    }

    pub fn get_click(&mut self) -> bool {
        let clicked = self.clicked;
        self.clicked = false;
        clicked
    }
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
            .add_systems(Update, paint_grid);
    }
}

fn spawn_cells(mut commands: Commands, mut grid: ResMut<Grid>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    let cell_coords: Vec<Hex> = grid.cell_coords().collect();
    let mesh_handle = meshes.add(Sphere::new(1.0));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::linear_rgba(1.0, 1.0, 1.0, 0.0),
        alpha_mode: AlphaMode::Premultiplied,
        ..default()
    });

    for cell_coord in cell_coords {
        let world_coord = grid.world_coords_from_hex(cell_coord);
        let mut spawn_commands = commands
            .spawn((PbrBundle {
                mesh: mesh_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(world_coord.x as f32, 0.0, world_coord.y as f32),
                ..default()
            },
            GridCell::default(),
            //PickableBundle::default(),
        ));

        let spawn_commands = spawn_commands
            .insert(On::<Pointer<Click>>::target_component_mut::<GridCell>(|_click, grid_cell|{
                grid_cell.on_click();
            }));
        
        let entiy = spawn_commands.id();
        grid.set_entity(cell_coord, entiy);
    }
}

fn spawn_light(mut commands: Commands) {
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

fn spawn_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 7.0, 14.0)
        .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
}

fn paint_grid(mut grid: ResMut<Grid>, mut materials: ResMut<Assets<StandardMaterial>>, mut query: Query<(Entity, &mut GridCell,  &Handle<StandardMaterial>)>) {
    for (entity, mut cell, material) in &mut query {
        if cell.get_click() {
            //transform.translation.y += 0.1;
            let material = materials.get_mut(material.id()).expect("Cell material does not exist!");
            material.base_color = Color::WHITE;
            grid.set_entity(*cell, entity);
        }
    }
}