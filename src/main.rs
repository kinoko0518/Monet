pub mod math;
pub mod graph_paper;

use crate::{graph_paper::GraphPaper, math::vector2::Vec2};

use std::fs::File;
use std::io::Write;

fn main() {
    let graph_paper = GraphPaper {
        name: "テスターを用いた電圧測定における内部抵抗の影響(DC2.5Vレンジ)".to_string(),
        margin: 100.0,
        size: graph_paper::A4,
        max_value: Vec2::vec2(35.0, 2.5),
        points: vec![
            Vec2::vec2(0.47, 0.050),
            Vec2::vec2(1.0, 0.085),
            Vec2::vec2(1.3, 0.1),
            Vec2::vec2(1.77, 0.15),
            Vec2::vec2(2.3, 0.2),
            Vec2::vec2(3.0, 0.25),
            Vec2::vec2(3.9, 0.3),
            Vec2::vec2(5.3, 0.4),
            Vec2::vec2(10.0, 0.7),
            Vec2::vec2(13.0, 0.85),
            Vec2::vec2(16.9, 1.0),
            Vec2::vec2(24.0, 1.2),
            Vec2::vec2(34.0, 1.48),
        ],
        stroke_width: 3.0,
    };
    let linear_graph = graph_paper::LinearGraph {
        graph_paper: graph_paper,
        great_split_length: 50.0,
        short_split_length: 25.5,
        
        v_great_split: 5,
        v_short_split: 5,

        h_great_split: 7,
        h_short_split: 5,
    };
    export(linear_graph.serialise()).unwrap();
}

fn export(what_to_export:String) -> Result<(), Box<dyn std::error::Error>> {
    // ToDo: 今動けばいいクォリティなのでパスべた書き　後で直す
    let mut file = File::create("C:/Projects/Monet/output/output.svg")?;
    write!(file, "{}", what_to_export)?;
    file.flush()?;
    Ok(())
}