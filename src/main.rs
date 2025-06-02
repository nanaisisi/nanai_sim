#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod map;
mod save;
mod systems;
mod tile;
mod tile_color;

use bevy::prelude::*;
use map::{CurrentLayer, MAP_HEIGHT, MAP_SIZE, MapData3D, Turn, setup_map};
use save::save_map_and_turn_xml;
use systems::{layer_switch_system, turn_timer_system, update_tiles_visual};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Terrain Simulation".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.95, 0.95, 1.0)))
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
        .add_systems(Update, turn_timer_system)
        .add_systems(Update, update_tiles_visual)
        .add_systems(Update, layer_switch_system)
        .add_systems(Update, save_system)
        .run();
}

fn save_system(map: Res<MapData3D>, turn: Res<Turn>, timer: Local<f32>) {
    // 例: "map_save.xml" に保存
    let _ = save_map_and_turn_xml(&map, &turn, *timer as f64, "map_save.xml");
}
