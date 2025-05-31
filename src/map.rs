use crate::tile::{Tile, TileBundle, TileState};
use bevy::prelude::*;

pub const MAP_SIZE: usize = 16;
pub const TILE_SIZE: f32 = 32.0;
pub const MAP_HEIGHT: usize = 8;

#[derive(Resource)]
pub struct MapData3D(pub Vec<Vec<Vec<TileState>>>);

#[derive(Resource)]
pub struct Turn(pub u64);

pub fn setup_map(mut commands: Commands, mut map: ResMut<MapData3D>) {
    use crate::tile_color::tile_color;
    commands.spawn(Camera2d);
    let mut new_map = vec![vec![vec![TileState::Empty; MAP_SIZE]; MAP_SIZE]; MAP_HEIGHT];
    // 地形生成: 石(下層), 土(中層), 水(一部), 空気(上層)
    for z in 0..MAP_HEIGHT {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let state = if z < MAP_HEIGHT / 3 {
                    TileState::Stone // 下層は石
                } else if z < MAP_HEIGHT / 2 {
                    TileState::Soil // 中層は土
                } else if z == MAP_HEIGHT / 2 && (x > 4 && x < 11) && (y > 4 && y < 11) {
                    TileState::Water // 中央に水の池
                } else {
                    TileState::Empty // それ以外は空気
                };
                new_map[z][y][x] = state;
                // z=0のみ描画（2D表示のまま）
                if z == 0 {
                    commands.spawn(TileBundle {
                        sprite: Sprite {
                            color: tile_color(state),
                            custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            x as f32 * TILE_SIZE - (MAP_SIZE as f32 * TILE_SIZE) / 2.0
                                + TILE_SIZE / 2.0,
                            y as f32 * TILE_SIZE - (MAP_SIZE as f32 * TILE_SIZE) / 2.0
                                + TILE_SIZE / 2.0,
                            0.0,
                        ),
                        tile: Tile { x, y, z, state },
                    });
                }
            }
        }
    }
    *map = MapData3D(new_map);
}
