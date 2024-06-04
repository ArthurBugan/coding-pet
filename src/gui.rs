use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::player::{Health, Player};
use crate::state::GameState;
use crate::world::GameEntity;

pub struct GuiPlugin;

#[derive(Component)]
struct DebugText;
#[derive(Component)]
struct MainMenuItem;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                handle_main_menu_buttons.run_if(in_state(GameState::MainMenu)),
            );
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::BLACK,
                            ..default()
                        },
                    ));
                });
        })
        .insert(MainMenuItem);
}

fn handle_main_menu_buttons(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                next_state.set(GameState::GameInit);
            }
            _ => {}
        }
    }
}

fn despawn_main_menu(mut commands: Commands, menu_items_query: Query<Entity, With<MainMenuItem>>) {
    for e in menu_items_query.iter() {
        commands.entity(e).despawn_recursive();
    }
}
