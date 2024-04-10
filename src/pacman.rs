use bevy::prelude::*;

const PLAYER_SPEED: f32 = 1.0; // Speed at which the player moves
pub struct PacManPlugin;

impl Plugin for PacManPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pacman);
        app.add_systems(Update, player_movement);
    }
}


#[derive(Component)]
pub struct PacMan {
    pub speed: f32, // Speed of the player
}

fn spawn_pacman(mut commands: Commands, asset_server: Res<AssetServer>){
    let scale_factor = 28.0 / 14.0;
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("pacman.png"),
            transform: Transform::from_xyz(-120.0, 10.0, 1.0).with_scale(Vec3::splat(scale_factor)),
            ..default()
        })
        .insert(PacMan {
            speed: PLAYER_SPEED,
        });
}

fn player_movement(keyboard_input: Res<ButtonInput<KeyCode>>,mut query: Query<(&mut Transform, &mut PacMan)>) {

    let mut movement = Vec2::ZERO;
    // Get the set of pressed keys
    let pressed_inputs = keyboard_input.get_pressed();
    // Iterate through all input and change movement
    for input in pressed_inputs{
        match input {
            KeyCode::ArrowUp => movement += Vec2::new(0.0, 1.0),
            KeyCode::ArrowDown => movement += Vec2::new(0.0, -1.0),
            KeyCode::ArrowLeft => movement += Vec2::new(-1.0, 0.0),
            KeyCode::ArrowRight => movement += Vec2::new(1.0, 0.0),
            _ => {}
        }
    }


    if movement.length_squared() > 0.0 {
        if let Ok((mut transform, pacman)) = query.get_single_mut(){
            // Normalize movement vector to ensure consistent speed
            movement = movement.normalize() * pacman.speed;
            // Update Pac-Man's position
            transform.translation += movement.extend(0.0);
        }
    }
    
}