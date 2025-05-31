use crate::map::{MAP_SIZE, MapData3D, Turn};
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
        let mut rng = rand::thread_rng();
        // bot的な自動処理: 各タイルの状態を進める（z=0のみ）
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let state = map.0[0][y][x];
                let new_state = match state {
                    TileState::Field => TileState::Growing,
                    TileState::Growing => TileState::Harvest,
                    TileState::Harvest => TileState::Field,
                    TileState::Empty => {
                        if rng.gen_range(0.0..1.0) < 0.01 {
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

pub fn update_tiles_visual(mut tiles: Query<(&Tile, &mut Sprite)>) {
    use crate::tile_color::tile_color;
    for (tile, mut sprite) in tiles.iter_mut() {
        sprite.color = tile_color(tile.state);
    }
}
