use std::sync::Arc;

use glam::Vec3;

use crate::{
    hit_info::{self, HitInfo},
    material::{self, Material},
    ray::Ray,
};

// トレイトの継承ってやつ
// Syncは複数スレッドからのアクセスを許可するらしい, 後々に調べていくことにする
// トレイトの継承より合成の方が推奨されているっぽい
// この場合はrenderで並行処理することが確定なので継承してしまおう!
pub trait Shape: Sync {
    // t0とt1は光線の衝突範囲の媒介変数らしい
    fn hit(&self, ray: &Ray, t0: f32, t1: f32) -> Option<HitInfo>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    // cloneする方法以外に実装方法がありそう
    material: Arc<dyn Material>,
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t0: f32, t1: f32) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);
        let d = b * b - 4.0 * a * c;

        // 直線と円の交点が1つ以上あるとき
        // つまり, 光線と円が当たったとき
        if d > 0.0 {
            let root = d.sqrt();
            // 2時方程式の解の公式まんま
            let tmp = (-b - root) / (2.0 * a);

            if t0 < tmp && tmp < t1 {
                let p = ray.at(tmp);
                return Some(HitInfo::new(
                    tmp,
                    p,
                    (p - self.center) / self.radius,
                    Arc::clone(&self.material),
                ));
            }

            let tmp = (-b + root) / (2.0 * a);
            if t0 < tmp && tmp < t1 {
                let p = ray.at(tmp);
                return Some(HitInfo::new(
                    tmp,
                    p,
                    (p - self.center) / self.radius,
                    Arc::clone(&self.material),
                ));
            }
        }

        None
    }
}

// impl Shape for Vec<Box<dyn Shape>>も一応できる
pub struct ShapeList {
    // Vecは可変長配列
    // Boxはヒープにメモリ確保したスマートポインタ
    // dynはトレイトであることを示すキーワードっぽい
    pub objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
}

impl Shape for ShapeList {
    // 光線を飛ばしたときに当たった一番近い物体をO(n)で走査して返すメソッド
    // ここにはpubはいらないんだな
    fn hit(&self, ray: &Ray, t0: f32, t1: f32) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = t1;
        for object in &self.objects {
            if let Some(info) = object.hit(ray, t0, closest_so_far) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }

        hit_info
    }
}
