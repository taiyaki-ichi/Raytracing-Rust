// これないとダメらしい
// rust-analyzerも動かなかったし
mod camera;
mod ray;
mod render;
mod hit_info;
mod shape;

use camera::Camera;
use glam::{vec3, Vec2, Vec3};
use image::{Rgb, RgbImage};
// prelude::*はfor_eachとか用
use rayon::{iter::IntoParallelRefMutIterator, prelude::*};
use render::{Scene, render};
use shape::{ShapeList, Sphere, Shape};

use crate::ray::Ray;

struct SimpleScene{
    world: ShapeList,
}

impl SimpleScene{

    // メンバ変数を持つとコンストラクタを書かなきゃいけないっぽい
    // コンストラクタについて他の記述の仕方もあると思う
    fn new()->Self{
        let mut world=ShapeList::new();
        world.push(Box::new(Sphere::new(vec3(0.0, 0.0, -1.0),0.5)));
        world.push(Box::new(Sphere::new(vec3(0.0, -100.5, -1.0),100.0)));
        Self{world}
    }

    // 上側が青, 下側が白のグラデーションになるような背景
    fn background(&self, d:Vec3)->Vec3{
        let t = 0.5 * (d.y + 1.0);
        Vec3::ONE.lerp(vec3(0.5,0.7,1.0), t)
    }
}

impl Scene for SimpleScene{
    fn camera(&self) -> Camera {
        Camera::new(
            vec3(4.0,0.0,0.0),
             vec3(0.0, 2.0, 0.0), 
            vec3(-2.0, -1.0, -1.0)
        )
    }

    fn trace(&self, ray: Ray) -> Vec3 {
        let hit_info=self.world.hit(&ray, 0.0, f32::MAX);
        if let Some(hit)=hit_info{
            // 当たった場合は物体の法線をもとにした色をつける
            0.5*(hit.n+Vec3::ONE)
        }else {
            self.background(ray.direction)
        }
    }
}

fn main() {
    render(SimpleScene::new());
}
