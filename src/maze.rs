use bevy::{prelude::*, transform};
use std::fs;
use crate::pacman::PacMan;

// Assuming TileType and main function are defined as before
enum TileType {
    Wall,
    Path,
    Dot,
    Cherry,
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Maze::new(load_maze("assets/maze.txt")));
        app.add_systems(Startup, spawn_maze);
        app.add_systems(Update, pellet_collision_system);
    }
}

#[derive(Resource)]
struct Maze {
    cells: Vec<Vec<TileType>>
}

#[derive(Component)]
pub struct Dot;

impl Maze {
    fn new(cells: Vec<Vec<TileType>>) -> Self {
        Self { cells }
    }
    
    fn get_pac_man_cell(&self, pac_man_position: (f32, f32)) -> (usize, usize) {
        // Calculate the cell indices based on Pac-Man's position
        let x_index = (pac_man_position.0 - 120.0 / 32.0) as usize;
        let y_index = (pac_man_position.1 / 32.0) as usize;

        // Check if the indices are within the bounds of the map
       (x_index,y_index)
    }
    
    fn handle_pellet_collision(&mut self, pac_man_position: (usize, usize)) {
        if let TileType::Dot = self.cells[pac_man_position.1][pac_man_position.0] {
            // Handle pellet collision
            // For example, increase score and remove the pellet from the map
            // Increase score
            // Remove pellet from the map
            self.cells[pac_man_position.1][pac_man_position.0] = TileType::Path;
        }
    }
}

fn spawn_maze(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let maze = load_maze("assets/maze.txt"); // Make sure to implement load_maze appropriately

    // Assuming you have separate images for each type
    let wall_texture_handle = asset_server.load("wall.png");
    let dot_texture_handle = asset_server.load("dot.png");
    let cherry_texture_handle = asset_server.load("cherry.png");

    for (y, row) in maze.iter().enumerate() {
        for (x, ref tile) in row.iter().enumerate() {
            let position = Vec3::new(x as f32 * 32.0 - 600.0, y as f32 * -32.0 + 300.0, 0.0); // Example tile size of 32x32
            match tile {
                TileType::Wall => {
                    commands.spawn(SpriteBundle {
                        texture: wall_texture_handle.clone(),
                        transform: Transform::from_translation(position),
                        ..default()
                    });
                }
                TileType::Dot => {
                    commands.spawn(SpriteBundle {
                        texture: dot_texture_handle.clone(),
                        transform: Transform::from_translation(position),
                        ..default()
                    }).insert(Dot);
                }
                // Repeat for Cherry, Player, etc., adjusting texture_handle as needed
                TileType::Cherry => {
                    commands.spawn(SpriteBundle {
                        texture: cherry_texture_handle.clone(),
                        transform: Transform::from_translation(position),
                        ..default()
                    });
                }
                _ => ()
            }
        }
    }
}

fn load_maze(file_path: &str) -> Vec<Vec<TileType>> {
    let content = fs::read_to_string(file_path).expect("Failed to load maze file");
    content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'W' => TileType::Wall,
                    'D' => TileType::Dot,
                    'C' => TileType::Cherry,
                    _ => TileType::Path,
                })
                .collect()
        })
        .collect()
}

fn pellet_collision_system(
    mut commands: Commands,
    mut maze: ResMut<Maze>,
    pac_man_query: Query<(&Transform, &PacMan)>,
    dots_query: Query<(Entity, &Transform), With<Dot>>,
) {
    for (pac_man_transform, _) in pac_man_query.iter() {
        let index = maze.get_pac_man_cell((pac_man_transform.translation.x, pac_man_transform.translation.y));

        maze.handle_pellet_collision(index);

        // Integrate the despawning logic directly here
        for (dot_entity, dot_transform) in dots_query.iter() {
            if dot_transform.translation.distance(pac_man_transform.translation) < 1.0 {
                commands.entity(dot_entity).despawn();
            }
        }
    }
}
