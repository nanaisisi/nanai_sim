#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod map;
mod systems;
mod tile;
mod tile_color;

use bevy::prelude::*;
use map::{MAP_HEIGHT, MAP_SIZE, MapData3D, TILE_SIZE, Turn, setup_map};
use systems::{turn_timer_system, update_tiles_visual};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Turn(0))
        .insert_resource(MapData3D(vec![
            vec![
                vec![tile::TileState::Empty; MAP_SIZE];
                MAP_SIZE
            ];
            MAP_HEIGHT
        ]))
        .add_systems(Startup, setup_map)
        .add_systems(Update, (turn_timer_system, update_tiles_visual))
        .run();
}
