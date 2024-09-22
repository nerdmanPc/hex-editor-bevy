use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
mod grid; use grid::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins).add_plugins(EditorPlugin)
        .run();
}

#[derive(Component, Copy, Clone)]
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
            .add_systems(Startup, spawn_cells)
            .add_systems(Update, paint_grid);
    }
}

fn spawn_cells(mut grid: ResMut<Grid>, mut commands: Commands) {
    let mut build_cell_commands = commands.spawn((
        PbrBundle::default(),
    ));

    build_cell_commands.insert(On::<Pointer<Click>>::target_component_mut::<GridCell>(|_click, grid_cell|{
        grid_cell.on_click();
    }));
}

fn paint_grid(mut grid: ResMut<Grid>, mut query: Query<(Entity, &mut GridCell)>) {
    for (entity, mut cell) in &mut query{
        if cell.get_click() {
            grid.set_entity(*cell, entity);
        }
    }
}