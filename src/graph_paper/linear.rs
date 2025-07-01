use super::{XScale, YScale, GraphPaper, Vec2};

/// X軸のリニア軸
pub struct XLinearScale {
    // 横長目盛分割数 / 横長目盛の短目盛での分割数
    pub h_great_split:u32,
    pub h_short_split:u32,
    pub max_value    :f32,
}
impl super::XScale for XLinearScale {
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
                        from, ((self.max_value / self.h_great_split as f32)
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
    fn to_scaled_x<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a> {
        Box::new(|x:f32| -> f32 {
            let size = graph_paper.size.x - 2_f32 * graph_paper.margin;
            graph_paper.margin + (x / self.max_value) * size
        })
    }
}
/// Y軸のリニア軸
pub struct YLinearScale {
    // 縦長目盛分割数 / 縦長目盛の短目盛での分割数
    pub v_great_split :u32,
    pub v_short_split :u32,
    pub max_value     :f32,
}
impl super::YScale for YLinearScale {
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
                            (self.max_value / self.v_great_split as f32)
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
    fn to_scaled_y<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a> {
        Box::new(|y:f32| -> f32 {
            let size = graph_paper.size.y - 2_f32 * graph_paper.margin;
            graph_paper.size.y - graph_paper.margin - (y / self.max_value) * size
        })
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
        let to_graph_coords = |p:Vec2| {
            let x = self.x_linear.to_scaled_x(&self.graph_paper);
            let y = self.y_linear.to_scaled_y(&self.graph_paper);
            Vec2::vec2(x(p.x), y(p.y))
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