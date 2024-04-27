use bevy::prelude::*;
use crate::maze::Maze;
use rand::prelude::*;

const GHOST_SPEED: f32 = 60.0; // Speed at which the ghost moves

pub struct GhostPlugin;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GhostMode {
    Chase,
    Scatter,
    Frightened,
}

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
    pub ghost_mode: GhostMode,
    pub last_direction: Vec2,
}


fn spawn_ghost(mut commands: Commands, asset_server: Res<AssetServer>){
    let v = vec!["cyan_ghost.png".to_string(), "pink_ghost.png".to_string(), "red_ghost.png".to_string(), "orange_ghost.png".to_string()];
    // scaling the ghosts according to pixel number
    let scale_factor = 28.0 / 14.0;
    for i in 0..4 {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load(v[i].clone()),
                // transform: Transform::from_xyz(-80.0 - 25.0 * i as f32, 140.0, 1.0).with_scale(Vec3::splat(scale_factor)),
                transform: Transform::from_xyz(-200.0, 140.0, 1.0).with_scale(Vec3::splat(scale_factor)),
                ..default()
            })
            // Ensure to insert the Ghost component
            .insert(Ghost{
                speed: GHOST_SPEED,
                direction: Vec2::ZERO, // Initialize direction to zero
                last_direction: Vec2::ZERO,
                ghost_mode: GhostMode::Chase
            });
    }
}

fn ghost_move_system(
    time: Res<Time>,
    maze: Res<Maze>,
    mut ghost_query: Query<(&mut Transform, &mut Ghost)>,
    pac_man_query: Query<&Transform, Without<Ghost>>,
) {
    for (mut ghost_transform, mut ghost) in ghost_query.iter_mut() {


        if ghost.ghost_mode == GhostMode::Scatter{
            let (x_index, y_index) = world_to_grid(ghost_transform.translation, 32.0);
            if x_index <= 16 && x_index >= 14 && y_index == 5{
                ghost_transform.translation.x = -117.89949;
                ghost_transform.translation.y = 174.38478;
                break;
            } 
            if x_index == 15 && y_index == 4{
                ghost_transform.translation.y = 174.38478 + 20.0;
            }

            if ghost.direction == Vec2::ZERO {
                ghost.direction = Vec2::new(
                    random::<f32>() - 0.5, // Random number between -0.5 and 0.5 for X direction
                    random::<f32>() - 0.5, // Random number between -0.5 and 0.5 for Y direction
                ).normalize(); // Normalize to ensure consistent speed
            }

        } else if ghost.ghost_mode == GhostMode::Chase{     
            // Implement chase mode logic here
            if let Ok(pac_man_transform) = pac_man_query.get_single() {
                // Calculate the direction towards the Pac-Man
                let direction = (pac_man_transform.translation - ghost_transform.translation).truncate().normalize();
                ghost.direction = direction;
            }
            
        } else if ghost.ghost_mode == GhostMode::Frightened{
            // Implement frightened mode logic here
        }
        let next_position = ghost_transform.translation + ghost.direction.extend(0.0) * ghost.speed * time.delta_seconds();

        // Check if the next position is walkable in the maze
        if maze.is_walkable(next_position) {
            ghost_transform.translation = next_position;
        } else {
            // If the next position is not walkable, generate a new random direction
            ghost.direction = random_direction_exclude(ghost.last_direction);
            ghost_transform.translation += ghost.direction.extend(0.0) * ghost.speed * time.delta_seconds();

        }
    }
}

// Helper function to generate a random direction excluding the last direction
fn random_direction_exclude(exclude: Vec2) -> Vec2 {
    let mut new_direction = Vec2::new(
        random::<f32>() - 0.5,
        random::<f32>() - 0.5,
    ).normalize();
    while new_direction == exclude || new_direction == -exclude {
        new_direction = Vec2::new(
            random::<f32>() - 0.5,
            random::<f32>() - 0.5,
        ).normalize();
    }
    new_direction
}

fn world_to_grid(world_position: Vec3, cell_size: f32) -> (usize, usize) {
    let grid_x = ((world_position.x + 615.0) / cell_size).floor() as usize;
    let grid_y = (13.0 - (world_position.y + 100.0) / cell_size).floor() as usize;
    (grid_x, grid_y)
}