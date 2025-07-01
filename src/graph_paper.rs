use crate::math::Vec2;

pub const A4:Vec2 = Vec2 {
    x: 2970.0,
    y: 2100.0
};

trait XScale {
    fn get_h_splitten(&self, graph_paper:&GraphPaper) -> Vec<String>;
}
trait YScale {
    fn get_v_splitten(&self, graph_paper:&GraphPaper) -> Vec<String>;
}
/// X軸のリニア軸
pub struct XLinearScale {
    // 横長目盛分割数 / 横長目盛の短目盛での分割数
    pub h_great_split:u32,
    pub h_short_split:u32,
}
impl XScale for XLinearScale {
    fn get_h_splitten(&self, graph_paper:&GraphPaper) -> Vec<String> {
        let x_from = graph_paper.margin;
        let y_from = graph_paper.size.y - graph_paper.margin;
        let unit = (graph_paper.size.x - 2_f32 * graph_paper.margin)
            / ((self.h_great_split * self.h_short_split) as f32);
        let get_a_splitten = |i:u32| -> String {
            let from = Vec2 {
                x: x_from + unit * i as f32,
                y: y_from
            };
            if i % self.h_great_split == 0 {
                let to = from + Vec2 { x: 0_f32, y: -graph_paper.great_split_length };
                format!(
                    "{}\n\t{}",
                    graph_paper.get_line(from, to),
                    GraphPaper::get_text(
                        from, ((graph_paper.max_value.x / self.h_great_split as f32)
                        * (i as f32 / self.h_great_split as f32)
                        ).to_string(),
                        Some(vec![
                            "text-anchor=\"end\"",
                            "dominant-baseline=\"hanging\"",
                            "font-size=\"20pt\""
                        ])
                    )
                )
            } else {
                let to = from + Vec2 { x: 0_f32, y: -graph_paper.short_split_length };
                graph_paper.get_line(from, to)
            }
        };
        (0..(self.h_great_split * self.h_short_split + 1))
            .map(|i| get_a_splitten(i))
            .collect::<Vec<String>>()
    }
}
/// Y軸のリニア軸
pub struct YLinearScale {
    // 縦長目盛分割数 / 縦長目盛の短目盛での分割数
    pub v_great_split:u32,
    pub v_short_split:u32,
}
impl YScale for YLinearScale {
    fn get_v_splitten(&self, graph_paper:&GraphPaper) -> Vec<String> {
        let y_from = graph_paper.margin;
        let unit = (graph_paper.size.y - 2_f32 * graph_paper.margin) / ((self.v_great_split * self.v_short_split) as f32);

        let get_a_splitten = |i: u32| -> String {
            let from = Vec2 {
                x: graph_paper.margin,
                y: graph_paper.size.y - y_from - unit * (i as f32)
            };
            if i % self.v_great_split == 0 {
                let to = from + Vec2 { x: graph_paper.great_split_length, y: 0_f32 };
                format!(
                    "{}\n\t{}",
                    graph_paper.get_line(from, to),
                    GraphPaper::get_text(
                        from, (
                            (graph_paper.max_value.y / self.v_great_split as f32)
                            * (i as f32 / self.v_great_split as f32)
                        ).to_string(),
                        Some(vec![
                            "text-anchor=\"end\"",
                            "font-size=\"20pt\""
                        ])
                    )
                )
            } else {
                let to = from + Vec2 { x: graph_paper.short_split_length, y: 0_f32 };
                graph_paper.get_line(from, to)
            }
        };
        (0..(self.v_great_split * self.v_short_split + 1))
            .map(|i| get_a_splitten(i))
            .collect::<Vec<String>>()
    }
}
/// X軸の対数軸
pub struct XLogScale {
    pub from: i32,
    pub to  : i32
}
impl XScale for XLogScale {
    fn get_h_splitten(&self, graph_paper:&GraphPaper) -> Vec<String> {
        let width = self.to - self.from;
        let unit = (graph_paper.size.x - 2_f32 * graph_paper.margin) / width as f32;
        let base = (graph_paper.max_value.x).powf(1 as f32 /self.to as f32);

        let get_a_splitten = |i:i32| -> String {
            let from = Vec2 {
                x: graph_paper.margin + unit * ((i - self.from) as f32),
                y: graph_paper.size.y - graph_paper.margin
            };
            let to = from + Vec2 {
                x: 0_f32,
                y: -graph_paper.great_split_length
            };
            format!(
                "{}\n\t{}",
                graph_paper.get_line(from, to),
                GraphPaper::get_text(
                    from,
                    base
                        .powi(i as i32)
                        .to_string(),
                    Some(vec![
                        "text-anchor=\"end\"",
                        "font-size=\"20pt\""
                    ])
                )
            )
        };
        (self.from..self.to+1)
            .map(|i| get_a_splitten(i))
            .collect::<Vec<String>>()
    }
}

#[derive(Clone)]
struct SVGHandle {
    size: Vec2,
    elements: Vec<String>
}
impl SVGHandle {
    fn serialise(&self) -> String {
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n{}\n</svg>",
            self.size.x, self.size.y,
            self.elements.iter().map(|s| format!("\t{}", s)).collect::<Vec<String>>().join("\n")
        )
    }
    fn add_element(&mut self, element:String) -> &mut Self {
        self.elements.push(element);
        self
    }
    fn add_elements(&mut self, elements:Vec<String>) -> &mut Self {
        self.elements.extend(elements);
        self
    }
}

/// グラフ用紙の基底クラス
#[derive(Clone)]
pub struct GraphPaper {
    // グラフの名前
    pub name: String,
    // 余白
    pub margin: f32,
    // サイズ
    pub size: Vec2,
    // 最大値
    pub max_value: Vec2,
    // グラフにプロットする点
    pub points: Vec<Vec2>,
    // 線の太さ
    pub stroke_width: f32,
    // 長目盛の長さ / 短目盛の長さ
    pub great_split_length:f32,
    pub short_split_length:f32,
}
impl GraphPaper {
    fn get_margin(&self) -> String {
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"none\" opacity=\"1\" stroke=\"black\" x=\"{}\" y=\"{}\" stroke-width=\"{}\" />",
            self.size.x - self.margin * 2_f32, self.size.y - self.margin * 2_f32,
            self.margin, self.margin, self.stroke_width
        )
    }
    fn get_line(&self, from:Vec2, to:Vec2) -> String {
        format!(
            "<line stroke=\"black\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke-width=\"{}\" />",
            from.x, from.y, to.x, to.y, self.stroke_width
        )
    }
    fn get_text(anchor:Vec2, text:String, extra_property:Option<Vec<&str>>) -> String {
        format!(
            "<text x=\"{}\" y=\"{}\" {}>{}</text>",
            anchor.x, anchor.y,
            if let Some(s) = extra_property {
                s.join(" ")
            } else {
                "".to_string()
            },
            text
        )
    }
    fn get_paper<F>(&self, to_graph_coords:F) -> SVGHandle
        where F: Fn(Vec2) -> Vec2
    {
        let mut handle = SVGHandle {
            size: self.size,
            elements: Vec::new()
        };
        handle
            // 枠を追加
            .add_element(self.get_margin())
            // タイトルを追加
            .add_element(Self::get_text(
                self.size / Vec2::vec2(2.0, 1.0),
                self.name.clone(),
                Some(vec![
                    "text-anchor=\"middle\"",
                    "font-size=\"20pt\""
                ]))
            )
            // プロット点を追加
            .add_elements(
                self.points.iter()
                    .map(|p| to_graph_coords(*p).to_point())
                    .collect::<Vec<String>>()
            )
            .clone()
    }
}

/// 等間隔目盛りのグラフ
pub struct LinearGraph {
    pub graph_paper:GraphPaper,
    pub x_linear: XLinearScale,
    pub y_linear: YLinearScale
}
impl LinearGraph {
    pub fn serialise(&self) -> String {
        let min = Vec2::vec2(
            self.graph_paper.margin,
            self.graph_paper.margin
        );
        let max = Vec2::vec2(
            self.graph_paper.size.x - self.graph_paper.margin,
            self.graph_paper.size.y - self.graph_paper.margin
        );
        let to_graph_coords = |p:Vec2| {
            let pure_graph_coords = min + (max - min) * (p / self.graph_paper.max_value);
            Vec2::vec2(pure_graph_coords.x, min.y + max.y - pure_graph_coords.y)
        };
        self.graph_paper
            .get_paper(to_graph_coords)
            // 横基準線を追加
            .add_elements(self.x_linear.get_h_splitten(&self.graph_paper))
            // 縦基準線を追加
            .add_elements(self.y_linear.get_v_splitten(&self.graph_paper))
            .serialise()
    }
}

/// 片対数グラフ
pub struct SemiLogGraph {
    pub graph_paper:GraphPaper,

    pub x_log:XLogScale,
    pub y_linear:YLinearScale
}
impl SemiLogGraph {
    pub fn serialise(&self) -> String {
        let min = Vec2::vec2(
            self.graph_paper.margin,
            self.graph_paper.margin
        );
        let max = Vec2::vec2(
            self.graph_paper.size.x - self.graph_paper.margin,
            self.graph_paper.size.y - self.graph_paper.margin
        );
        let base = (self.graph_paper.max_value.x).powf(1 as f32 / self.x_log.to as f32);

        let to_graph_coords = |p:Vec2| {
            let pure_graph_coords = Vec2::vec2(
                (p.x.log(base) - self.x_log.from as f32) / (self.x_log.to - self.x_log.from) as f32  * (max.x - min.x),
                (max.y - min.y) * (p.y / self.graph_paper.max_value.y)
            );
            Vec2::vec2(min.x + pure_graph_coords.x, max.y - pure_graph_coords.y)
        };
        self.graph_paper
            .get_paper(to_graph_coords)
            // 縦基準線を追加
            .add_elements(self.y_linear.get_v_splitten(&self.graph_paper))
            // 横基準線を追加
            .add_elements(self.x_log.get_h_splitten(&self.graph_paper))
            .serialise()
    }
}