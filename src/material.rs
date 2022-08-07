use glam::Vec3;

use crate::{
    hit_info::HitInfo,
    random_vec3_in_unit_sphere,
    ray::Ray,
    utility::{reflect, refract, schlick},
};

// 散乱の情報
pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Vec3,
}

impl ScatterInfo {
    pub fn new(ray: Ray, albedo: Vec3) -> Self {
        Self { ray, albedo }
    }
}

// Sendはスレッドを跨いで所有権を移動させることができる
pub trait Material: Sync + Send {
    // 散乱の情報の取得
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}

// ランバート反射
// 大きさ1以下のベクトルを利用した反射のモデルのやつ
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let target = hit.p + hit.n + random_vec3_in_unit_sphere();
        Some(ScatterInfo::new(
            Ray::new(hit.p, target - hit.p),
            self.albedo,
        ))
    }
}

// 金属
// 鏡面反射する
pub struct Metal {
    pub albedo: Vec3,
    // 反射した光線のずらし具合
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = reflect(ray.direction.normalize(), hit.n);
        // 次の光線の向きを乱数でずらす
        reflected += self.fuzz * random_vec3_in_unit_sphere();
        if reflected.dot(hit.n) > 0.0 {
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), self.albedo))
        } else {
            None
        }
    }
}

// 誘導体
pub struct Dielectric {
    pub ri: f32,
}

impl Dielectric {
    pub const fn new(ri: f32) -> Self {
        Self { ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let reflected = reflect(ray.direction, hit.n);
        // タプルで束縛!
        let (outward_normal, in_over_out, cosine) = {
            let dot = ray.direction.dot(hit.n);
            // 入射する場合
            if ray.direction.dot(hit.n) > 0.0 {
                (-hit.n, self.ri, self.ri * dot / ray.direction.length())
            }
            // 出射する場合
            else {
                (hit.n, self.ri.recip(), -dot / ray.direction.length())
            }
        };

        if let Some(refracted) = refract(-ray.direction, outward_normal, in_over_out) {
            if rand::random::<f32>() > schlick(cosine, self.ri) {
                return Some(ScatterInfo::new(Ray::new(hit.p, refracted), Vec3::ONE));
            }
        }

        Some(ScatterInfo::new(Ray::new(hit.p, reflected), Vec3::ONE))
    }
}
