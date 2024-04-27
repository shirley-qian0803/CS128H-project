mod maze;
mod ghost;
mod pacman;
mod collision;

use bevy::prelude::*;
use maze::MazePlugin;
use ghost::GhostPlugin;
use pacman::PacManPlugin;
use collision::CollisionPlugin;
use crate::pacman::PacMan;

#[derive(Resource)]
struct TextEntity(Entity);

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(MazePlugin)
        .add_plugins(PacManPlugin)
        .add_plugins(GhostPlugin)
        .add_plugins(CollisionPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_score_text, text_color_system))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>){
    // Text that displays the score of the player
    commands.spawn(TextBundle {
        text: Text::from_sections([
            TextSection::new(
                "Current score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    ..default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }
            }),
        ]),
        style: Style {
            position_type: PositionType::Absolute,      
            left: Val::Px(10.0), // Adjust based on your needs
            bottom: Val::Px(10.0), // Adjust based on your needs
            ..default()
        },
        // Adjust Transform to correctly place the text
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // Change these values based on your window size
        ..default()
    }
    );

    // #[cfg(not(feature = "default_font"))]
    // commands.spawn(
    //     TextBundle::from_section(
    //         "Default font disabled",
    //         TextStyle {
    //             font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //             ..default()
    //         },
    //     )
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         bottom: Val::Px(5.0),
    //         left: Val::Px(15.0),
    //         ..default()
    //     }),
    // );

    // put it into the resources
    let text_entity = commands.spawn(TextBundle {
        text: Text::from_sections([
            TextSection::new(
                "Current score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    ..default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }
            }),
        ]),
        style: Style {
            position_type: PositionType::Absolute,      
            left: Val::Px(350.0), 
            bottom: Val::Px(10.0), 
            ..default()
        },
        // Adjust Transform to correctly place the text
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // Change these values based on your window size
        ..default()
    }).id();
    commands.insert_resource(TextEntity(text_entity));

}

fn text_color_system(time: Res<Time>, mut query: Query<&mut Text, With<ColorText>>) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

fn update_score_text(
    text_entity: Res<TextEntity>,
    asset_server: Res<AssetServer>,
    pacman_query: Query<& PacMan>,
    mut text_query: Query<&mut Text>,
) {
    let mut current_score = 0.0;
    if let Ok(pacman) = pacman_query.get_single(){
        current_score = pacman.score;
    }

    if let Ok(mut text) = text_query.get_mut(text_entity.0) {
        let str = format!("{} points", current_score);
        text.sections = vec![
            TextSection::new(
                str,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(
                if cfg!(feature = "default_font") {
                    TextStyle {
                        font_size: 60.0,
                        color: Color::GOLD,
                        ..default()
                    }
                } else {
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 60.0,
                        color: Color::GOLD,
                    }
                },
            ),
        ];
    }
}