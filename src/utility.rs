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

// スネルの法則を用いて屈折率の比から透過する物体に当たったときの反射ベクトルを導出する
// in_over_outは (入射側の媒質の屈折率)/(出射側の媒質の屈折率)
pub fn refract(vec3: Vec3, normal: Vec3, in_over_out: f32) -> Option<Vec3> {
    let uv = vec3.normalize();
    let dt = uv.dot(normal);
    let d = 1.0 - in_over_out.powi(2) * (1.0 - dt.powi(2));
    if d > 0.0 {
        Some(-in_over_out * (uv - normal * dt) - normal * d.sqrt())
    }
    // 全反射の場合
    else {
        None
    }
}

// フレネル方程式の解を導出するためのSchlickの近似式
// 大気中から媒質に入射する時用Ï
pub fn schlick(cosine: f32, ri: f32) -> f32 {
    let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
