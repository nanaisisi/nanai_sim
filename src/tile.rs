use bevy::prelude::*;

// タイルの状態や地形種別の定義
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileState {
    Empty,
    Field,
    Growing,
    Harvest,
    Soil,  // 土
    Stone, // 石
    Water, // 水（流体）
}

// タイル情報（3次元座標）
#[derive(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub state: TileState,
}
