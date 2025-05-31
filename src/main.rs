#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod map;
mod systems;
mod tile;
mod tile_color;

use bevy::prelude::*;
use map::{CurrentLayer, MAP_HEIGHT, MAP_SIZE, MapData3D, Turn, setup_map};
use systems::{layer_switch_system, turn_timer_system, update_tiles_visual};

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
        .insert_resource(CurrentLayer(MAP_HEIGHT - 1)) // 一番上層から表示
        .add_systems(Startup, setup_map)
        .add_systems(
            Update,
            (turn_timer_system, update_tiles_visual, layer_switch_system),
        )
        .run();
}
