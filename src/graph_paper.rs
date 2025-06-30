use crate::math::Vec2;

pub const A4:Vec2 = Vec2 {
    x: 2970.0,
    y: 2100.0
};

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
}
impl GraphPaper {
    fn get_margin(&self, size: Vec2, margin:f32) -> String {
        format!(
            "<rect width=\"{}\" height=\"{}\" fill=\"none\" opacity=\"1\" stroke=\"black\" x=\"{}\" y=\"{}\" stroke-width=\"{}\" />",
            size.x - margin * 2_f32, size.y - margin * 2_f32,
            margin, margin, self.stroke_width
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
    fn get_paper(&self) -> SVGHandle {
        let min = Vec2::vec2(self.margin, self.margin);
        let max = Vec2::vec2(self.size.x - self.margin, self.size.y - self.margin);

        let mut handle = SVGHandle {
            size: self.size,
            elements: Vec::new()
        };
        let to_graph_coords = |p:Vec2| {
            let pure_graph_coords = min + (max - min) * (p / self.max_value);
            Vec2::vec2(pure_graph_coords.x, min.y + max.y - pure_graph_coords.y)
        };
        handle
            // 枠を追加
            .add_element(self.get_margin(self.size, self.margin))
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
pub struct LinearGraph {
    pub graph_paper:GraphPaper,
    // 長目盛の長さ
    pub great_split_length:f32,
    // 短目盛の長さ
    pub short_split_length:f32,

    // 横長目盛分割数 / 横長目盛に対応する数 / 横長目盛の短目盛での分割数
    pub h_great_split:u32,
    pub h_short_split:u32,

    // 縦長目盛分割数 / 縦長目盛に対応する数 / 縦長目盛の短目盛での分割数
    pub v_great_split:u32,
    pub v_short_split:u32,
}
impl LinearGraph {
    fn get_v_splitten(&self, i:u32) -> String {
        let y_from = self.graph_paper.margin;
        let unit = (self.graph_paper.size.y - 2_f32 * self.graph_paper.margin) / ((self.v_great_split * self.v_short_split) as f32);
        let from = Vec2 {
            x: self.graph_paper.margin,
            y: self.graph_paper.size.y - y_from - unit * (i as f32)
        };
        if i % self.v_great_split == 0 {
            let to = from + Vec2 { x: self.great_split_length, y: 0_f32 };
            format!(
                "{}\n\t{}",
                self.graph_paper.get_line(from, to),
                GraphPaper::get_text(
                    from, (
                        (self.graph_paper.max_value.y / self.v_great_split as f32)
                         * (i as f32 / self.v_great_split as f32)
                    ).to_string(),
                    Some(vec![
                        "text-anchor=\"end\"",
                        "font-size=\"20pt\""
                    ])
                )
            )
        } else {
            let to = from + Vec2 { x: self.short_split_length, y: 0_f32 };
            self.graph_paper.get_line(from, to)
        }
    }
    fn get_h_splitten(&self, i:u32) -> String {
        let x_from = self.graph_paper.margin;
        let y_from = self.graph_paper.size.y - self.graph_paper.margin;
        let unit = (self.graph_paper.size.x - 2_f32 * self.graph_paper.margin)
            / ((self.h_great_split * self.h_short_split) as f32);
        let from = Vec2 {
            x: x_from + unit * i as f32,
            y: y_from
        };
        if i % self.h_great_split == 0 {
            let to = from + Vec2 { x: 0_f32, y: -self.great_split_length };
            format!(
                "{}\n\t{}",
                self.graph_paper.get_line(from, to),
                GraphPaper::get_text(
                    from, ((self.graph_paper.max_value.x / self.h_great_split as f32)
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
            let to = from + Vec2 { x: 0_f32, y: -self.short_split_length };
            self.graph_paper.get_line(from, to)
        }
    }
    pub fn serialise(&self) -> String {
        // 縦基準線を追加
        self.graph_paper
            .get_paper()
            .add_elements(
                (1..(self.v_great_split * self.v_short_split + 1))
                    .map(|i| self.get_v_splitten(i))
                    .collect::<Vec<String>>()
            )
                // 横基準線を追加
            .add_elements(
                (1..(self.h_great_split * self.h_short_split + 1))
                    .map(|i| self.get_h_splitten(i))
                    .collect::<Vec<String>>()
            )
            .serialise()
    }
}