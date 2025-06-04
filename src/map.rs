use crate::tile::{Tile, TileState};
use bevy::math::primitives::Cuboid;
use bevy::pbr::{Mesh3d, MeshMaterial3d};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub const MAP_SIZE: usize = 16;
pub const TILE_SIZE: f32 = 1.0; // 3D用に1.0に変更
pub const MAP_HEIGHT: usize = 8;

#[derive(Resource)]
pub struct MapData3D(pub Vec<Vec<Vec<TileState>>>);

#[derive(Resource)]
pub struct Turn(pub u64);

#[derive(Resource)]
pub struct CurrentLayer(pub usize);

pub fn setup_map(
    mut commands: Commands,
    mut map: ResMut<MapData3D>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use crate::tile_color::tile_color;
    // 3Dカメラ（正投影）
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: MAP_SIZE as f32,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(
            MAP_SIZE as f32 * 0.5,
            MAP_SIZE as f32 * 1.2,
            MAP_HEIGHT as f32 * 2.5,
        )
        .looking_at(
            Vec3::new(MAP_SIZE as f32 / 2.0, MAP_SIZE as f32 / 2.0, 0.0),
            Vec3::Y,
        ),
    ));
    // ライト
    commands.spawn((
        PointLight::default(),
        Transform::from_xyz(
            MAP_SIZE as f32 * 0.5,
            MAP_SIZE as f32 * 1.5,
            MAP_HEIGHT as f32 * 3.0,
        ),
    ));
    let mut new_map = vec![vec![vec![TileState::Empty; MAP_SIZE]; MAP_SIZE]; MAP_HEIGHT];
    for z in 0..MAP_HEIGHT {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let state = if z < MAP_HEIGHT / 3 {
                    TileState::Stone
                } else if z < MAP_HEIGHT / 2 {
                    TileState::Soil
                } else if z == MAP_HEIGHT / 2 && (x > 4 && x < 11) && (y > 4 && y < 11) {
                    TileState::Water
                } else {
                    TileState::Empty
                };
                new_map[z][y][x] = state;
                // すべてのz層をspawn
                let color = tile_color(state);
                let mesh = meshes.add(Cuboid::default().mesh().size(
                    TILE_SIZE - 0.05,
                    TILE_SIZE - 0.05,
                    TILE_SIZE - 0.05,
                ));
                let material = materials.add(StandardMaterial {
                    base_color: color,
                    perceptual_roughness: 0.9,
                    ..default()
                });
                commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(material),
                    Transform::from_xyz(x as f32, y as f32, z as f32),
                    Tile { x, y, z, state },
                ));
            }
        }
    }
    *map = MapData3D(new_map);
}
