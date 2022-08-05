// これないとダメらしい
// rust-analyzerも動かなかったし
mod camera;
mod hit_info;
mod ray;
mod render;
mod shape;

use camera::Camera;
use glam::{vec3, Vec2, Vec3};
use image::{Rgb, RgbImage};
// prelude::*はfor_eachとか用
use rayon::{iter::IntoParallelRefMutIterator, prelude::*};
use render::{render, render_aa, Scene, map};
use shape::{Shape, ShapeList, Sphere};

use crate::ray::Ray;

// 大きさ1以下ののランダムなベクトルの取得
// 条件を満たすまで乱数を生成する方法を棄却法というらしい
fn random_vec3_in_unit_sphere() -> Vec3 {
    loop {
        // [-1.0,1.0)の乱数を取得するクロージャ
        let get_random = || rand::random::<f32>() * 2.0 - 1.0;
        let point = vec3(get_random(), get_random(), get_random());
        if point.length() < 1.0 {
            return point;
        }
    }
}

struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    // メンバ変数を持つとコンストラクタを書かなきゃいけないっぽい
    // コンストラクタについて他の記述の仕方もあると思う
    fn new() -> Self {
        let mut world = ShapeList::new();
        world.push(Box::new(Sphere::new(vec3(0.0, 0.0, -1.0), 0.5)));
        world.push(Box::new(Sphere::new(vec3(0.0, -100.5, -1.0), 100.0)));
        Self { world }
    }

    // 上側が青, 下側が白のグラデーションになるような背景
    fn background(&self, d: Vec3) -> Vec3 {
        let t = 0.5 * (d.y + 1.0);
        Vec3::ONE.lerp(vec3(0.5, 0.7, 1.0), t)
    }
}

impl Scene for SimpleScene {
    fn camera(&self) -> Camera {
        Camera::new(
            vec3(4.0, 0.0, 0.0),
            vec3(0.0, 2.0, 0.0),
            vec3(-2.0, -1.0, -1.0),
        )
    }

    fn trace(&self, ray: Ray) -> Vec3 {
        // 0.001は同じ場所で衝突したと判定されないようにするための補正値
        let hit_info = self.world.hit(&ray, 0.0001, f32::MAX);
        if let Some(hit) = hit_info {
            // 当たった場合はランダムな方向に反射させ再度追跡する
            // 当たらなくなるまで再帰させている
            let target = hit.p + hit.n + random_vec3_in_unit_sphere();
            0.5 * self.trace(Ray::new(hit.p, target - hit.p))
        } else {
            self.background(ray.direction)
        }
    }
}

fn main() {
    render_aa(SimpleScene::new());
}
