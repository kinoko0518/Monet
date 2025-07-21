use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}
impl Vec2 {
    pub fn vec2(x:f32, y:f32) -> Self {
        Vec2 { x: x, y: y }
    }
    pub fn abs(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn mul(&self, multiplation:f32) -> Self {
        Vec2 {
            x: self.x * multiplation,
            y: self.y * multiplation
        }
    }
    pub fn log(self, base:f32) -> Vec2 {
        Vec2::vec2(self.x.log(base), self.y.log(base))
    }
}
impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}
impl std::ops::Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}
impl std::ops::Div for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}