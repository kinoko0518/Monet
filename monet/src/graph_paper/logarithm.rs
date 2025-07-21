use crate::graph_paper::TextSetting;

use super::{
    XScale, YScale,
    GraphPaper, Vec2,
    XSCALE_TEXT_SETTING,
    YSCALE_TEXT_SETTING,
};

fn get_subscale<F, G>(
    base: f32,
    power_of_base:f32,
    graph_paper:&GraphPaper,
    calc_tick_from: F,
    calc_tick_to: G
) -> Vec<String>
    where F: Fn(f32) -> Vec2, G: Fn(Vec2, f32) -> Vec2
{
    (2..base as u32)
        .map(|j| {
            let value = j as f32 * power_of_base;
            let from = calc_tick_from(value);
            let to = calc_tick_to(from, graph_paper.short_split_length);
            graph_paper.get_line(from, to)
        }).collect::<Vec<String>>()
}

fn generate_ticks<F, G>(
    from: i32,
    to  : i32,
    graph_paper: &GraphPaper,
    base: f32,
    calc_tick_from: F,
    calc_tick_to: G,
    text_setting: &TextSetting,
) -> Vec<String>
    where F: Fn(f32) -> Vec2, G: Fn(Vec2, f32) -> Vec2
{
    (from..to+1)
        .flat_map(|i:i32| -> Vec<String> {
            let value = base.powi(i);
            let from = calc_tick_from(value);
            let to = calc_tick_to(from, graph_paper.great_split_length);
            let mut res = vec![
                graph_paper.get_line(from, to),
                GraphPaper::get_text(
                    from,
                    value.to_string(),
                    Some(text_setting.serialise())
                )
            ];
            res.append(&mut get_subscale(
                base,
                value,
                graph_paper,
                &calc_tick_from,
                &calc_tick_to
            ));
            res
        })
        .collect::<Vec<String>>()
}

fn to_value(graph_paper: &GraphPaper, base: f32, from: i32, to: i32, size: f32) -> Box<dyn Fn(f32) -> f32 + '_> {
    Box::new(move |p:f32| -> f32 {
        graph_paper.margin + (p.log(base) - from as f32) / (to - from) as f32  * size
    })
}

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
        let x = self.to_scaled_x(graph_paper);
        generate_ticks(
            self.from,
            self.to,
            graph_paper,
            self.base,
            |i:f32| Vec2 {
                x: x(i),
                y: graph_paper.size.y - graph_paper.margin
            },
            |from, scale_length:f32| {
                from - Vec2::vec2(0_f32, scale_length)
            },
            &XSCALE_TEXT_SETTING
        )
    }
    fn to_scaled_x<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a> {
        to_value(
            graph_paper,
            self.base,
            self.from,
            self.to,
            graph_paper.size.x - 2_f32 * graph_paper.margin
        )
    }
}

/// Y軸の対数軸
#[derive(Clone)]
pub struct YLogScale {
    pub base: f32,
    pub from: i32,
    pub to  : i32,
}

impl YScale for YLogScale {
    fn get_v_splitten(&self, graph_paper:&GraphPaper) -> Vec<String> {
        let y = self.to_scaled_y(graph_paper);
        generate_ticks(
            self.from,
            self.to,
            graph_paper,
            self.base,
            |i:f32| Vec2 {
                x: graph_paper.margin,
                y: graph_paper.size.y - y(i)
            },
            |from, scale_length:f32| {
                from - Vec2::vec2(-scale_length, 0_f32)
            },
            &YSCALE_TEXT_SETTING
        )
    }
    fn to_scaled_y<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a> {
        Box::new(|f:f32| {
            graph_paper.size.y - to_value(
                graph_paper,
                self.base,
                self.from,
                self.to,
                graph_paper.size.y - 2_f32 * graph_paper.margin
            )(f)
        })
    }
}