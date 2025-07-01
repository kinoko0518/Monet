mod linear;
mod logarithm;

use crate::math::Vec2;

pub use self::linear::{
    LinearGraph,
    XLinearScale,
    YLinearScale
};
pub use self::logarithm::{
    SemiLogGraph,
    XLogScale,
};

pub const A4:Vec2 = Vec2 {
    x: 2970.0,
    y: 2100.0
};

trait XScale {
    fn get_h_splitten(&self, graph_paper:&GraphPaper) -> Vec<String>;
    fn to_scaled_x<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a>;
}
trait YScale {
    fn get_v_splitten(&self, graph_paper:&GraphPaper) -> Vec<String>;
    fn to_scaled_y<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a>;
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