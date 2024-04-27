use bevy::prelude::*;


pub struct PopUpWindowPlugin;


// #[derive(Resource)]
// struct PopupEntity(Entity);


#[derive(Resource)]
pub struct GameState {
    pub game_over: bool,
    pub player_won: bool
}

impl Plugin for PopUpWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, game_over_system_set);
        // app.add_systems(Update, handle_popup_input);
    }
} 


fn game_over_system_set(
    mut commands: Commands,
    game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
) {
    if game_state.game_over {
        let text_style = TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 50.0,
            color: Color::WHITE,
        };
        let win_text = if game_state.player_won {
            "You Win!"
        } else {
            "You Lose!"
        };
        let popup_node = create_popup_scene();
        commands
            .spawn(popup_node)
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(win_text, text_style));
            });
        // commands.insert_resource(PopupEntity(popup_entity));
    }
}

// fn handle_popup_input(
//     mut commands: Commands,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     popup_entity: Res<PopupEntity>,
// ) {
//     if keyboard_input.just_pressed(KeyCode::Escape) {
//         if let entity = popup_entity.0 {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }

fn create_popup_scene() -> NodeBundle {
    NodeBundle {
        style: Style {
            left: Val::Percent(30.),
            top: Val::Percent(30.0),
            width: Val::Percent(40.0),
            height: Val::Percent(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(41.0 / 256.0,189.0 / 256.0,193.0 / 256.0)),
        ..default()
    } 
}