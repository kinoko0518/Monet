use crate::graph_paper::{
    TextSetting,
    XSCALE_TEXT_SETTING,
    YSCALE_TEXT_SETTING
};

use super::{GraphPaper, Vec2};

fn generate_ticks<F, G>(
    graph_paper    : &GraphPaper,
    calc_tick_from : F,
    great_split    : u32,
    short_split    : u32,
    max_value      : f32,
    text_setting   : TextSetting,
    calc_tick_endpoint :G
) -> Vec<String>
    where F: Fn(f32) -> Vec2, G: Fn(Vec2, f32) -> Vec2
{
    let total_split = great_split * short_split;
    (0..(total_split + 1))
        .map(|i| {
            let value = (i as f32 / total_split as f32) * max_value;
            let from = calc_tick_from(value);
            let scale_length:f32;
            if i % great_split == 0 {
                scale_length = graph_paper.great_split_length;
                let to = calc_tick_endpoint(from, scale_length);
                let line = graph_paper.get_line(from, to);
                let text = GraphPaper::get_text(
                    from, value.to_string(),
                    Some(text_setting.serialise())
                );
                format!("{}\n\t{}", line, text)
            } else {
                scale_length = graph_paper.short_split_length;
                let to = calc_tick_endpoint(from, scale_length);
                graph_paper.get_line(from, to)
            }
        })
        .collect::<Vec<String>>()
}

/// X軸のリニア軸
pub struct XLinearScale {
    // 横長目盛分割数 / 横長目盛の短目盛での分割数
    pub h_great_split:u32,
    pub h_short_split:u32,
    pub max_value    :f32,
}
impl super::XScale for XLinearScale {
    fn get_h_splitten(&self, graph_paper:&GraphPaper) -> Vec<String> {
        let to_scaled = self.to_scaled_x(graph_paper);
        generate_ticks(
            graph_paper,
            Box::new(|i:f32| {
                Vec2::vec2(
                    to_scaled(i),
                    graph_paper.size.y - graph_paper.margin
                )
            }),
            self.h_great_split,
            self.h_short_split,
            self.max_value,
            XSCALE_TEXT_SETTING,
            Box::new(|from:Vec2, scale_length:f32| {
                from - Vec2::vec2(0_f32, scale_length)
            })
        )
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
        let to_scaled = self.to_scaled_y(graph_paper);
        generate_ticks(
            graph_paper,
            Box::new(|i:f32| {
                Vec2::vec2(
                    graph_paper.margin,
                    to_scaled(i)
                )
            }),
            self.v_great_split,
            self.v_short_split,
            self.max_value,
            YSCALE_TEXT_SETTING,
            Box::new(|from:Vec2, scale_length:f32| {
                from + Vec2::vec2(scale_length, 0_f32)
            })
        )
    }
    fn to_scaled_y<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a> {
        Box::new(|y:f32| -> f32 {
            let size = graph_paper.size.y - 2_f32 * graph_paper.margin;
            graph_paper.size.y - graph_paper.margin - (y / self.max_value) * size
        })
    }
}