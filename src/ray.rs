use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    // 始点
    pub origin: Vec3,
    // 方向
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    // tは時間
    // Vec3にはMul<f64>トレイトが実装されていなかったのでf32
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
