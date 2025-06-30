use crate::math::Vec2;

pub const A4:Vec2 = Vec2 {
    x: 2970.0,
    y: 2100.0
};

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
    // グラフにプロットする点
    pub points: Vec<Vec2>,
    // 線の太さ
    pub stroke_width: f32,

    // 長目盛の長さ
    pub great_split_length:f32,
    // 短目盛の長さ
    pub short_split_length:f32,

    // 横長目盛分割数 / 横長目盛に対応する数 / 横長目盛の短目盛での分割数
    pub h_great_split:u32,
    pub h_unit:f32,
    pub h_short_split:u32,

    // 縦長目盛分割数 / 縦長目盛に対応する数 / 縦長目盛の短目盛での分割数
    pub v_great_split:u32,
    pub v_unit:f32,
    pub v_short_split:u32,
}
impl GraphPaper {
    pub fn serialise(&self) -> String {
        let min = Vec2::vec2(self.margin, self.margin);
        let max = Vec2::vec2(self.size.x - self.margin, self.size.y - self.margin);

        let mut handle = SVGHandle {
            size: self.size,
            elements: Vec::new()
        };
        let get_margin = |size: Vec2, margin:f32| {
            format!(
                "<rect width=\"{}\" height=\"{}\" fill=\"none\" opacity=\"1\" stroke=\"black\" x=\"{}\" y=\"{}\" stroke-width=\"{}\" />",
                size.x - margin * 2_f32, size.y - margin * 2_f32,
                margin, margin, self.stroke_width
            )
        };
        let get_line = |from:Vec2, to:Vec2| -> String {
            format!(
                "<line stroke=\"black\" x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke-width=\"{}\" />",
                from.x, from.y, to.x, to.y, self.stroke_width
            )
        };
        let get_text = |anchor:Vec2, text:String, extra_property:Option<Vec<&str>>| -> String {
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
        };
        let get_v_great_splitten = |i:u32| -> String {
            let y_from = self.margin;
            let unit = (self.size.y - 2_f32 * self.margin) / (self.v_great_split as f32);
            let from = Vec2 {
                x: self.margin,
                y: self.size.y - y_from - unit * (i as f32)
            };
            let to = from + Vec2 { x: self.great_split_length, y: 0_f32 };
            format!(
                "{}\n\t{}",
                get_line(from, to),
                get_text(
                    from, (self.v_unit * i as f32).to_string(),
                    Some(vec![
                        "text-anchor=\"end\"",
                        "font-size=\"20pt\""
                    ])
                )
            )
        };
        let get_v_short_splitten = |i:u32| -> String {
            let y_from = self.margin;
            let unit = (self.size.y - 2_f32 * self.margin) / ((self.v_great_split * self.v_short_split) as f32);
            let from = Vec2 {
                x: self.margin,
                y: self.size.y - y_from - unit * (i as f32)
            };
            let to = from + Vec2 { x: self.short_split_length, y: 0_f32 };
            get_line(from, to)
        };
        let get_h_great_splitten = |i:u32| -> String {
            let x_from = self.margin;
            let y_from = self.size.y - self.margin;
            let unit = (self.size.x - 2_f32 * self.margin) / (self.h_great_split as f32);
            let from = Vec2 {
                x: x_from + unit * (i as f32),
                y: y_from
            };
            let to = from + Vec2 { x: 0_f32, y: -self.great_split_length };
            format!(
                "{}\n\t{}",
                get_line(from, to),
                get_text(
                    from, (self.h_unit * i as f32).to_string(),
                    Some(vec![
                        "text-anchor=\"end\"",
                        "dominant-baseline=\"hanging\"",
                        "font-size=\"20pt\""
                    ])
                )
            )
        };
        let get_h_short_splitten = |i:u32| -> String {
            let x_from = self.margin;
            let y_from = self.size.y - self.margin;
            let unit = (self.size.x - 2_f32 * self.margin) / ((self.h_great_split * self.h_short_split) as f32);
            let from = Vec2 {
                x: x_from + unit * (i as f32),
                y: y_from
            };
            let to = from + Vec2 { x: 0_f32, y: -self.short_split_length };
            get_line(from, to)
        };
        let to_graph_coords = |p:Vec2| {
            let value_max = Vec2::vec2(
                self.h_unit * (self.h_great_split as f32),
                self.v_unit * (self.v_great_split as f32),
            );
            let pure_graph_coords = min + (max - min) * (p / value_max);
            Vec2::vec2(pure_graph_coords.x, min.y + max.y - pure_graph_coords.y)
        };
        handle
            // 枠を追加
            .add_element(get_margin(self.size, self.margin))
            // タイトルを追加
            .add_element(get_text(
                self.size / Vec2::vec2(2.0, 1.0),
                self.name.clone(),
                Some(vec![
                    "text-anchor=\"middle\"",
                    "font-size=\"20pt\""
                ]))
            )
            // 縦長基準線を追加
            .add_elements(
                (1..(self.v_great_split + 1))
                    .map(|i| get_v_great_splitten(i))
                    .collect::<Vec<String>>()
            )
            // 縦短基準線を追加
            .add_elements(
                (1..self.v_great_split * self.v_short_split)
                    .map(|i| get_v_short_splitten(i))
                    .collect::<Vec<String>>()
            )
             // 横長基準線を追加
            .add_elements(
                (1..(self.h_great_split + 1))
                    .map(|i| get_h_great_splitten(i))
                    .collect::<Vec<String>>()
            )
            // 横短基準線を追加
            .add_elements(
                (1..self.h_great_split * self.h_short_split)
                    .map(|i| get_h_short_splitten(i))
                    .collect::<Vec<String>>()
            )
            // プロット点を追加
            .add_elements(
                self.points.iter()
                    .map(|p| to_graph_coords(*p).to_point())
                    .collect::<Vec<String>>()
            )
            .serialise()
    }
}