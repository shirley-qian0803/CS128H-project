mod maze;
mod ghost;
mod pacman;
mod collision;


use bevy::prelude::*;
use maze::MazePlugin;
use ghost::GhostPlugin;
use pacman::PacManPlugin;
use collision::CollisionPlugin;




fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(MazePlugin)
        .add_plugins(GhostPlugin)
        .add_plugins(PacManPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}
