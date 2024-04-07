use bevy::prelude::*;
use std::fs;

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
        app.add_systems(Startup, spawn_maze);
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
                    });
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