use crate::YLinearScale;
use super::{XScale, YScale, GraphPaper, Vec2};

/// X軸の対数軸
#[derive(Clone)]
pub struct XLogScale {
    pub base: f32,
    pub from: i32,
    pub to  : i32,
    pub tick: u32
}
impl XScale for XLogScale {
    fn get_h_splitten(&self, graph_paper:&GraphPaper) -> Vec<String> {
        let base = self.get_base();
        let get_a_splitten = |i:i32| -> String {
            let to_graph_x = self.to_scaled_x(&graph_paper);
            let from = Vec2 {
                x: to_graph_x(base.powi(i)),
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
    fn to_scaled_x<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a> {
        let min = Vec2::vec2(
            graph_paper.margin,
            graph_paper.margin
        );
        let max = Vec2::vec2(
            graph_paper.size.x - graph_paper.margin,
            graph_paper.size.y - graph_paper.margin
        );
        let size = max - min;
        let base = self.get_base();

        Box::new(move |p:f32| -> f32 {
            min.x + (p.log(base) - self.from as f32) / (self.to - self.from) as f32  * size.x
        })
    }
}
impl XLogScale {
    fn max_value(&self) -> f32 {
        self.base.powi(self.to)
    }
    fn get_base(&self) -> f32 {
        (self.max_value()).powf(1 as f32 /self.to as f32)
    }
    fn get_subscale(&self, graph_paper:&GraphPaper) -> Vec<String> {
        let to_graph_x = self.to_scaled_x(&graph_paper);
        let base: f32 = self.get_base();

        (self.from..self.to)
            .map(|i| -> Vec<String> {
                let power_of_base = base.powi(i);
                (2..self.base as u32)
                    .map(|j| {
                        let value_x = j as f32 * power_of_base;
                        let from = Vec2 {
                            x: to_graph_x(value_x),
                            y: graph_paper.size.y - graph_paper.margin,
                        };
                        let to = from + Vec2 {
                            x: 0.0,
                            y: -graph_paper.short_split_length,
                        };
                        graph_paper.get_line(from, to)
                    })
                    .collect::<Vec<String>>()
                })
                .flatten()
                .collect::<Vec<String>>()
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
        let to_graph_coords = |p:Vec2| -> Vec2 {
            let x = self.x_log.to_scaled_x(&self.graph_paper);
            let y = self.y_linear.to_scaled_y(&self.graph_paper);
            Vec2::vec2(x(p.x), y(p.y))
        };
        self.graph_paper
            .get_paper(to_graph_coords)
            // 縦基準線を追加
            .add_elements(self.y_linear.get_v_splitten(&self.graph_paper))
            // 横基準線を追加
            .add_elements(self.x_log.get_h_splitten(&self.graph_paper))
            .add_elements(self.x_log.get_subscale(&self.graph_paper))
            .serialise()
    }
}