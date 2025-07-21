mod linear;
mod logarithm;

use crate::math::Vec2;

const P_RADIUS:f32 = 10.0; 

pub use self::linear::{
    XLinearScale,
    YLinearScale
};

pub use self::logarithm::{
    XLogScale,
    YLogScale
};

pub const A4:Vec2 = Vec2 {
    x: 2970.0,
    y: 2100.0
};
const XSCALE_TEXT_SETTING:TextSetting = TextSetting {
    font_size: 20,
    v_anchor: Some(VerticalAnchor::Top),
    h_anchor: Some(HorizontalAnchor::Start)
};
const YSCALE_TEXT_SETTING:TextSetting = TextSetting {
    font_size: 20,
    v_anchor: Some(VerticalAnchor::Bottom),
    h_anchor: Some(HorizontalAnchor::End)
};

pub trait XScale {
    fn get_h_splitten(&self, graph_paper:&GraphPaper) -> Vec<String>;
    fn to_scaled_x<'a>(&'a self, graph_paper:&'a GraphPaper) -> Box<dyn Fn(f32) -> f32 + 'a>;
}
pub trait YScale {
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
    fn get_text(anchor:Vec2, text:String, extra_property:Option<Vec<String>>) -> String {
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
    fn to_plot(&self, point:&Vec2) -> String {
        format!("<circle r=\"{}\" cx=\"{}\" cy=\"{}\" />", P_RADIUS, point.x, point.y)
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
                Some(TextSetting {
                    font_size: 20,
                    v_anchor: Some(VerticalAnchor::Bottom),
                    h_anchor: Some(HorizontalAnchor::Centre)
                }.serialise())
            ))
            // プロット点を追加
            .add_elements(
                self.points.iter()
                    .map(|p| self.to_plot(&to_graph_coords(*p)))
                    .collect::<Vec<String>>()
            )
            .clone()
    }
}

pub struct Graph {
    pub graph_paper: GraphPaper,
    pub x_scale: Box<dyn XScale>,
    pub y_scale: Box<dyn YScale>
}

impl Graph {
    pub fn serialise(&self) -> String {
        let x = self.x_scale.to_scaled_x(&self.graph_paper);
        let y = self.y_scale.to_scaled_y(&self.graph_paper);
        let to_graph_coords = |p:Vec2| -> Vec2 {
            Vec2::vec2(x(p.x), y(p.y))
        };
        self.graph_paper
            .get_paper(to_graph_coords)
            // 縦基準線を追加
            .add_elements(self.y_scale.get_v_splitten(&self.graph_paper))
            // 横基準線を追加
            .add_elements(self.x_scale.get_h_splitten(&self.graph_paper))
            .serialise()
    }
}

#[allow(dead_code)]
enum VerticalAnchor {
    Top,
    Centre,
    Bottom
}
enum HorizontalAnchor {
    Start,
    Centre,
    End
}
struct TextSetting {
    font_size: u32,
    v_anchor : Option<VerticalAnchor>,
    h_anchor : Option<HorizontalAnchor>,
}
impl TextSetting {
    fn serialise(&self) -> Vec<String> {
        let mut setting = vec![
            format!("font-size=\"{}pt\"", self.font_size)
        ];
        if let Some(s) = &self.v_anchor {
            setting.push(
                format!("dominant-baseline=\"{}\"", match s {
                    VerticalAnchor::Top => "hanging",
                    VerticalAnchor::Centre => "middle",
                    VerticalAnchor::Bottom => "auto"
                })
            );
        };
        if let Some(s) = &self.h_anchor {
            setting.push(
                format!("text-anchor=\"{}\"", match s {
                    HorizontalAnchor::Start => "start",
                    HorizontalAnchor::Centre => "center",
                    HorizontalAnchor::End => "end"
                })
            );
        };
        setting
    }
}