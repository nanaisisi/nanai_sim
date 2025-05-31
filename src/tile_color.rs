use crate::tile::TileState;
use bevy::prelude::*;

pub fn tile_color(state: TileState) -> Color {
    match state {
        TileState::Empty => Color::BLACK,
        TileState::Field => Color::srgb(0.3, 0.3, 0.3), // 濃いグレー
        TileState::Growing => Color::srgb(0.6, 0.6, 0.6), // 明るいグレー
        TileState::Harvest => Color::WHITE,
        TileState::Soil => Color::srgb(0.2, 0.2, 0.2), // 黒に近い茶色
        TileState::Stone => Color::srgb(0.5, 0.5, 0.5), // 中間グレー
        TileState::Water => Color::srgb(0.8, 0.8, 0.8), // 白に近いグレー（水は明るく）
    }
}
