//! Everything needed to run the main game logic

use bevy::{prelude::*, window::close_on_esc};

/// Set the game state to align systems with their respective runtimes
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Tactical RPG"),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((lib::player::PlayerPlugin, lib::graphics::GraphicsPlugin))
        .add_systems(Update, close_on_esc)
        .run();
}
