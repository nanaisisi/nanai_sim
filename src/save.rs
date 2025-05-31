use crate::map::{MapData3D, Turn};
use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use std::fs::File;
use std::io::{BufWriter, Result, Write};

#[allow(dead_code)]
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

pub fn save_map_and_turn_xml(
    map: &MapData3D,
    turn: &Turn,
    next_turn_time: f64,
    path: &str,
) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = Writer::new(BufWriter::new(file));

    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
    writer.write_event(Event::Start(BytesStart::new("save")))?;

    // ターン
    writer.write_event(Event::Start(BytesStart::new("turn")))?;
    writer.write_event(Event::Text(BytesText::new(&turn.0.to_string())))?;
    writer.write_event(Event::End(BytesEnd::new("turn")))?;

    // 次ターンまでの時間
    writer.write_event(Event::Start(BytesStart::new("next_turn_time")))?;
    writer.write_event(Event::Text(BytesText::new(&next_turn_time.to_string())))?;
    writer.write_event(Event::End(BytesEnd::new("next_turn_time")))?;

    // マップ
    writer.write_event(Event::Start(BytesStart::new("map")))?;
    for (z, layer) in map.0.iter().enumerate() {
        let mut layer_tag = BytesStart::new("layer");
        layer_tag.push_attribute(("index", z.to_string().as_str()));
        writer.write_event(Event::Start(layer_tag))?;
        for (y, row) in layer.iter().enumerate() {
            let mut row_tag = BytesStart::new("row");
            row_tag.push_attribute(("y", y.to_string().as_str()));
            writer.write_event(Event::Start(row_tag))?;
            for (x, &cell) in row.iter().enumerate() {
                let mut cell_tag = BytesStart::new("cell");
                cell_tag.push_attribute(("x", x.to_string().as_str()));
                writer.write_event(Event::Start(cell_tag))?;
                writer.write_event(Event::Text(BytesText::new(&(cell as u8).to_string())))?;
                writer.write_event(Event::End(BytesEnd::new("cell")))?;
            }
            writer.write_event(Event::End(BytesEnd::new("row")))?;
        }
        writer.write_event(Event::End(BytesEnd::new("layer")))?;
    }
    writer.write_event(Event::End(BytesEnd::new("map")))?;
    writer.write_event(Event::End(BytesEnd::new("save")))?;
    Ok(())
}
