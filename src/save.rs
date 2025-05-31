use crate::map::{MapData3D, Turn};
use std::fs::File;
use std::io::{Result, Write};

pub fn save_map_and_turn(
    map: &MapData3D,
    turn: &Turn,
    next_turn_time: f64,
    path: &str,
) -> Result<()> {
    let mut file = File::create(path)?;
    // ヘッダー: ターン数と次ターンまでの残り秒数
    writeln!(file, "turn:{}", turn.0)?;
    writeln!(file, "next_turn_time:{}", next_turn_time)?;
    // マップデータ
    for (z, layer) in map.0.iter().enumerate() {
        writeln!(file, "[layer {}]", z)?;
        for row in layer {
            for &cell in row {
                write!(file, "{}", cell as u8)?;
            }
            writeln!(file)?;
        }
    }
    Ok(())
}
