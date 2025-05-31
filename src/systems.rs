use crate::map::{CurrentLayer, MAP_HEIGHT, MAP_SIZE, MapData3D, Turn};
use crate::tile::{Tile, TileState};
use bevy::prelude::*;
use rand::Rng;

pub fn turn_timer_system(
    time: Res<Time>,
    mut timer: Local<f32>,
    mut turn: ResMut<Turn>,
    mut map: ResMut<MapData3D>,
    mut tiles: Query<(&mut Tile, Entity)>,
) {
    *timer += time.delta_secs();
    if *timer >= 1.0 {
        *timer = 0.0;
        turn.0 += 1;
        let mut rng = rand::rng();
        // bot的な自動処理: 各タイルの状態を進める（z=0のみ）
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let state = map.0[0][y][x];
                let new_state = match state {
                    TileState::Field => TileState::Growing,
                    TileState::Growing => TileState::Harvest,
                    TileState::Harvest => TileState::Field,
                    TileState::Empty => {
                        if rng.random_range(0.0..1.0) < 0.01 {
                            TileState::Field
                        } else {
                            TileState::Empty
                        }
                    }
                    TileState::Soil => TileState::Soil,
                    TileState::Stone => TileState::Stone,
                    TileState::Water => TileState::Water,
                };
                map.0[0][y][x] = new_state;
            }
        }
        // タイルのstateを更新
        for (mut tile, _entity) in tiles.iter_mut() {
            tile.state = map.0[tile.z][tile.y][tile.x];
        }
    }
}

pub fn update_tiles_visual(
    mut tiles: Query<(&Tile, Entity, &mut Sprite)>,
    map: Res<MapData3D>,
    current_layer: Res<CurrentLayer>,
) {
    use crate::tile_color::tile_color;
    let z = current_layer.0;
    for (tile, _entity, mut sprite) in tiles.iter_mut() {
        // 同層が存在する場合はそのまま表示
        if tile.z == z {
            sprite.color = tile_color(tile.state).with_alpha(1.0);
        } else if tile.z < z {
            // 下層の表示: 透過度を下げて表示
            let below = &map.0[z][tile.y][tile.x];
            if *below == TileState::Empty {
                sprite.color = tile_color(tile.state).with_alpha(0.3);
            } else {
                sprite.color = tile_color(tile.state).with_alpha(0.0);
            }
        } else {
            // 上層は非表示
            sprite.color = tile_color(tile.state).with_alpha(0.0);
        }
    }
}

pub fn layer_switch_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_layer: ResMut<CurrentLayer>,
) {
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        if current_layer.0 + 1 < MAP_HEIGHT {
            current_layer.0 += 1;
        }
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        if current_layer.0 > 0 {
            current_layer.0 -= 1;
        }
    }
}
