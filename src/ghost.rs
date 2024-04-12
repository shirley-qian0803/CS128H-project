use bevy::prelude::*;
use bevy::utils::info;
use crate::pacman::PacMan;
use crate::maze::Maze;
use rand::prelude::*;

const GHOST_SPEED: f32 = 1.0; // Speed at which the player moves


#[derive(Component, Debug)]
pub struct Ghost {
    pub speed: f32, // Speed of the player
}

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ghost);
        app.add_systems(Update, ghost_move_system);
    }
} 

fn spawn_ghost(mut commands: Commands, asset_server: Res<AssetServer>){
    let v = vec!["cyan_ghost.png".to_string(), "pink_ghost.png".to_string(), "red_ghost.png".to_string(), "orange_ghost.png".to_string()];
    // scaling the ghosts according to pixel number
    let scale_factor = 28.0 / 14.0;
    for i in 0..4 {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load(v[i].clone()),
                transform: Transform::from_xyz(-80.0 - 25.0 * i as f32, 140.0, 1.0).with_scale(Vec3::splat(scale_factor)),
                ..default()
            })
            .insert(Ghost{
                speed: GHOST_SPEED,
            });
    }
}

fn ghost_move_system(
    time: Res<Time>,
    maze: Res<Maze>,
    mut ghost_query: Query<(&mut Transform, &Ghost)>,
    pac_man_query: Query<&Transform, (With<PacMan>, Without<Ghost>)>,
) {
    // Get Pac-Man's position from its Transform
    let pac_man_pos = pac_man_query.single();

    for (mut ghost_transform, ghost) in ghost_query.iter_mut() {


        let ghost_grid_pos = world_to_grid(ghost_transform.translation, 32.0);

        // the case when the maze is walkable
        if maze.is_walkable(ghost_grid_pos.0, ghost_grid_pos.1) {
            // Calculate the direction and distance to move towards the player
            let delta_position = (pac_man_pos.translation
                - ghost_transform.translation)
                .normalize_or_zero()
                * ghost.speed;

            // Update the ghost's position
            ghost_transform.translation += delta_position;
        } // handle the case when it's wall for the ghosts: e.g. choose a different path.
    }
}



fn world_to_grid(world_position: Vec3, cell_size: f32) -> (usize, usize) {
    let grid_x = ((world_position.x + 615.0) / cell_size).floor() as usize;
    let grid_y = (13.0 - (world_position.y + 100.0) / cell_size).floor() as usize;
    (grid_x, grid_y)
}

fn grid_to_world(grid_position: (usize, usize), cell_size: f32) -> Vec3 {
    // Convert grid coordinates (assuming to be cell center) back to world coordinates
    let world_x = grid_position.0 as f32 * cell_size - 615.0 + cell_size / 2.0;
    let world_y = (13.0 - grid_position.1 as f32) * cell_size - 100.0 + cell_size / 2.0;
    
    Vec3::new(world_x, world_y, 0.0) // Assuming Z coordinate is 0 for a 2D game
}