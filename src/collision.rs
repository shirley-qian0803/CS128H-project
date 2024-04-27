use bevy::prelude::*;
use crate::ghost::Ghost;
use crate::pacman::PacMan;
use crate::pop_up_window::GameState;
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_pacman_ghost);
    }
}

fn collision_pacman_ghost(
    pac_man_query: Query<(&Transform, &PacMan)>,
    ghost_query: Query<(&Transform, &Ghost)>,
    mut game_state: ResMut<GameState>
) {
    for (pac_man_transform, _) in pac_man_query.iter() {
        for (ghost_transform, _) in ghost_query.iter() {
            // Calculate distance between Pac-Man and ghost centers
            let distance = pac_man_transform.translation.distance(ghost_transform.translation);
            // Calculate sum of radii
            let sum_radii = 28.0;
            // Check for collision
            if distance < sum_radii {
                // Handle collision between Pac-Man and ghost
                // Example: Game over, decrease lives, etc.
                info!("Game is over!");
                game_state.game_over = true;
            }
        }
    }
}

