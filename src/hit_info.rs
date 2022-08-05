use glam::Vec3;

// ファイル名はスネークケースでいいのかな
pub struct HitInfo {
    // 光線の媒介変数
    // 進んだ距離とも表現できる感じか
    pub t: f32,
    // 当たった位置
    pub p: Vec3,
    // 当たった物体の法線
    pub n: Vec3,
}

impl HitInfo {
    // const fnは定数を返すメソッド
    // コンパイル時にも呼べるっぽい
    // でも制限も結構あるっぽい
    pub const fn new(t: f32, p: Vec3, n: Vec3) -> Self {
        Self { t, p, n }
    }
}
