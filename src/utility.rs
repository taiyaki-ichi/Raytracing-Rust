use glam::{vec3, Vec3};

// 外部で定義された構造体にメンバ関数を追加できないっぽいので普通の関数で
pub fn map<F>(v: Vec3, f: F) -> Vec3
where
    F: Fn(f32) -> f32,
{
    vec3(f(v.x), f(v.y), f(v.z))
}

// u8の配列に変換
pub fn to_rgb(v: Vec3) -> [u8; 3] {
    let rgb = map(v, |e| 255.99 * e.min(1.0).max(0.0));
    [rgb.x as u8, rgb.y as u8, rgb.z as u8]
}

// ガンマ補正をかける
pub fn gamma(color: Vec3, factor: f32) -> Vec3 {
    let recip = factor.recip();
    map(color, |x| x.powf(recip))
}

// ガンマ補正を解く
pub fn degamma(color: Vec3, factor: f32) -> Vec3 {
    map(color, |x| x.powf(factor))
}

// 法線で定義される面での反射
pub fn reflect(vec3: Vec3, normal: Vec3) -> Vec3 {
    vec3 - 2.0 * vec3.dot(normal) * normal
}
