use bevy::prelude::*;
use crate::maze::Maze;
use crate::pacman::PacMan;
use rand::prelude::*;

const GHOST_SPEED: f32 = 25.0; // Speed at which the ghost moves

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ghost);
        app.add_systems(Update, ghost_move_system);
    }
} 

// Define the Ghost struct here
#[derive(Component)]
pub struct Ghost {
    pub speed: f32, // Speed of the player
    pub direction: Vec2, // Current movement direction
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
            // Ensure to insert the Ghost component
            .insert(Ghost{
                speed: GHOST_SPEED,
                direction: Vec2::ZERO, // Initialize direction to zero
            });
    }
}

fn ghost_move_system(
    time: Res<Time>,
    maze: Res<Maze>,
    mut ghost_query: Query<(&mut Transform, &mut Ghost)>,
    pac_man_query: Query<&Transform, (With<PacMan>, Without<Ghost>)>,
) {
    // Get Pac-Man's position from its Transform
    let pac_man_pos = pac_man_query.single();

    for (mut ghost_transform, mut ghost) in ghost_query.iter_mut() {
        // Generate a random movement direction if the ghost hasn't chosen one yet
        if ghost.direction == Vec2::ZERO {
            ghost.direction = Vec2::new(
                random::<f32>() - 0.5, // Random number between -0.5 and 0.5 for X direction
                random::<f32>() - 0.5, // Random number between -0.5 and 0.5 for Y direction
            ).normalize(); // Normalize to ensure consistent speed
        }

        let next_position = ghost_transform.translation + ghost.direction.extend(0.0) * ghost.speed * time.delta_seconds();

        // Check if the next position is walkable in the maze
        if maze.is_walkable(next_position) {
            ghost_transform.translation = next_position;
        } else {
            // If the next position is not walkable, generate a new random direction
            ghost.direction = Vec2::new(
                random::<f32>() - 0.5,
                random::<f32>() - 0.5,
            ).normalize();
        }
    }
}
