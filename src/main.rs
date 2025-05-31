use bevy::prelude::*;
use rand::prelude::*;

const MAP_SIZE: usize = 16; // 16x16の正方形タイル
const TILE_SIZE: f32 = 32.0;
const TURN_INTERVAL: f32 = 1.0; // 1秒ごとにターン進行

#[derive(Clone, Copy, PartialEq, Eq)]
enum TileState {
    Empty,
    Field,
    Growing,
    Harvest,
}

#[derive(Component)]
struct Tile {
    x: usize,
    y: usize,
    state: TileState,
}

#[derive(Resource)]
struct Turn(pub u64);

#[derive(Resource)]
struct MapData(pub Vec<Vec<TileState>>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Turn(0))
        .insert_resource(MapData(vec![vec![TileState::Empty; MAP_SIZE]; MAP_SIZE]))
        .add_startup_system(setup_map)
        .add_system(turn_timer_system)
        .add_system(update_tiles_visual)
        .run();
}

fn setup_map(mut commands: Commands, mut map: ResMut<MapData>) {
    commands.spawn(Camera2dBundle::default());
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let state = if rand::random::<f32>() < 0.2 {
                TileState::Field
            } else {
                TileState::Empty
            };
            map.0[y][x] = state;
            commands.spawn((
                SpriteBundle {
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
                    ..default()
                },
                Tile { x, y, state },
            ));
        }
    }
}

fn tile_color(state: TileState) -> Color {
    match state {
        TileState::Empty => Color::WHITE,
        TileState::Field => Color::GRAY,
        TileState::Growing => Color::DARK_GREEN,
        TileState::Harvest => Color::BLACK,
    }
}

fn turn_timer_system(
    time: Res<Time>,
    mut timer: Local<f32>,
    mut turn: ResMut<Turn>,
    mut map: ResMut<MapData>,
    mut tiles: Query<(&mut Tile, Entity)>,

    mut commands: Commands,
) {
    *timer += time.delta_seconds();
    if *timer >= TURN_INTERVAL {
        *timer = 0.0;
        turn.0 += 1;
        // bot的な自動処理: 各タイルの状態を進める
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let state = map.0[y][x];
                let new_state = match state {
                    TileState::Field => TileState::Growing,
                    TileState::Growing => TileState::Harvest,
                    TileState::Harvest => TileState::Field,
                    TileState::Empty => {
                        if rand::random::<f32>() < 0.01 {
                            TileState::Field
                        } else {
                            TileState::Empty
                        }
                    }
                };
                map.0[y][x] = new_state;
            }
        }
        // タイルのstateを更新
        for (mut tile, _entity) in &mut tiles {
            tile.state = map.0[tile.y][tile.x];
        }
    }
}

fn update_tiles_visual(mut tiles: Query<(&Tile, &mut Sprite)>) {
    for (tile, mut sprite) in &mut tiles {
        sprite.color = tile_color(tile.state);
    }
}
