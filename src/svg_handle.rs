#[derive(Debug, Clone)]
struct Vec2 {
    x: f32,
    y: f32,
}
impl Vec2 {
    fn zero_coords() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
pub struct GraphHandle {
    output_size:Vec2,
    points:Vec<Vec2>,
}
impl GraphHandle {
    fn get_max(&self) -> Option<Vec2> {
        let mut max = self
            .points
            .get(0)?
            .clone();
        for c in &self.points {
            if c.x > max.x {
                max.x = c.x;
            }
            if c.y > max.y {
                max.y = c.y;
            }
        }
        Some(max)
    }
    fn get_min(&self) -> Option<Vec2> {
        let mut min = self
            .points
            .get(0)?
            .clone();
        for c in &self.points {
            if c.x < min.x {
                min.x = c.x;
            }
            if c.y < min.y {
                min.y = c.y;
            }
        }
        Some(min)
    }
    fn get_ascales_value_width(&self, x_line_amount:i32, y_line_amount:i32) -> Option<(i32, i32)> {
        let range = self.get_max()? - self.get_min()?;
        let mul_allowed = [1, 2, 5];
        let get_width = |line_amount:i32, size:f32| -> i32 {
            let mut i = 0;
            let width;
            'main: loop {
                for o in mul_allowed {
                    let width_now = line_amount * (10_i32.pow(i) * o);
                    if  width_now as f32 >= size {
                        width = width_now;
                        break 'main;
                    }
                }
                i += 1;
            }
            width
        };
        Some((
            get_width(x_line_amount, range.x),
            get_width(y_line_amount, range.y)
        ))
    }

}
struct Line {
    p1: Vec2,
    p2: Vec2
}
struct SVG {
    lines: Vec<Line>
}
impl SVG {
    fn add_line(&mut self, p1:Vec2, p2:Vec2) -> &mut Self {
        self.lines.push(Line {
            p1: p1,
            p2: p2
        });
        self
    }
}