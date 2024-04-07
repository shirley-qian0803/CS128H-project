mod maze;
mod ghost;
mod pacman;


use bevy::prelude::*;
use maze::MazePlugin;
use ghost::GhostPlugin;
use pacman::PacManPlugin;




fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(MazePlugin)
        .add_plugins(GhostPlugin)
        .add_plugins(PacManPlugin)
        .run();
}
