use bevy::prelude::*;

const GHOST_SPEED: f32 = 1.0; // Speed at which the player moves


#[derive(Component, Debug)]
pub struct Ghost {
    pub speed: f32, // Speed of the player
}

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pacman);
    }
} 

fn spawn_pacman(mut commands: Commands, asset_server: Res<AssetServer>){
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