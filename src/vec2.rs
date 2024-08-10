#[derive(Copy, Clone)]
pub struct Vec2(pub f32, pub f32);

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2(self.0 / rhs, self.1 / rhs)
    }
}

impl Vec2 {
    pub fn dot(self, other: Vec2) -> f32 {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn mag(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn dist(self, other: Vec2) -> f32 {
        (self - other).mag()
    }
}
