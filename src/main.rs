// これないとダメらしい
// rust-analyzerも動かなかったし
mod camera;
mod hit_info;
mod material;
mod ray;
mod render;
mod shape;
mod shape_builder;
mod utility;

use std::sync::Arc;

use camera::Camera;
use glam::{vec3, Vec2, Vec3};
use image::{Rgb, RgbImage};
use material::{Dielectric, Lambertian, Metal};
// prelude::*はfor_eachとか用
use rayon::{iter::IntoParallelRefMutIterator, prelude::*};
use render::{render, render_aa, Scene};
use shape::{Shape, ShapeList, Sphere};
use shape_builder::ShapeBuilder;
use utility::map;

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

struct RandomScene {
    world: ShapeList,
}

impl RandomScene {
    fn new() -> Self {
        let mut world = ShapeList::new();

        world.push(
            ShapeBuilder::new()
                .lambertian(vec3(0.5, 0.5, 0.5))
                .sphere(vec3(0.0, -1000.0, 0.0), 1000.0)
                .build(),
        );

        for au in -11..11 {
            let a = au as f32;
            for bu in -11..11 {
                let b = bu as f32;
                let rx = rand::random::<f32>();
                let rz = rand::random::<f32>();
                let material_choice = rand::random::<f32>();
                let center = vec3(a + 0.9 * rx, 0.2, b + 0.9 * rz);
                if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                    world.push(if material_choice < 0.8 {
                        let albedo =
                            map(Vec3::ONE, |_| rand::random::<f32>() * rand::random::<f32>());
                        ShapeBuilder::new()
                            .lambertian(albedo)
                            .sphere(center, 0.2)
                            .build()
                    } else if material_choice < 0.95 {
                        let albedo = map(Vec3::ONE, |_| rand::random::<f32>() * 0.5 + 0.5);
                        let fuzz = rand::random::<f32>();
                        ShapeBuilder::new()
                            .metal(albedo, fuzz)
                            .sphere(center, 0.2)
                            .build()
                    } else {
                        ShapeBuilder::new()
                            .dielectric(1.5)
                            .sphere(center, 0.2)
                            .build()
                    });
                }
            }
        }

        world.push(
            ShapeBuilder::new()
                .dielectric(1.5)
                .sphere(vec3(0.0, 1.0, 0.0), 1.0)
                .build(),
        );
        world.push(
            ShapeBuilder::new()
                .lambertian(vec3(0.4, 0.2, 0.1))
                .sphere(vec3(-4.0, 1.0, 0.0), 1.0)
                .build(),
        );
        world.push(
            ShapeBuilder::new()
                .metal(vec3(0.7,0.6,0.5), 0.0)
                .sphere(vec3(4.0, 1.0, 0.0), 1.0)
                .build(),
        );

        Self { world }
    }

    fn background(&self, d: Vec3) -> Vec3 {
        let t = 0.5 * (d.y + 1.0);
        Vec3::ONE.lerp(vec3(0.5, 0.7, 1.0), t)
    }
}


impl Scene for RandomScene {
    fn camera(&self) -> Camera {
        Camera::from_lookat(
            vec3(13.0, 2.0, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            20.0,
            self.aspect(),
        )
    }

    fn trace(&self, ray: Ray, depth: usize) -> Vec3 {
        // 0.001は同じ場所で衝突したと判定されないようにするための補正値
        let hit_info = self.world.hit(&ray, 0.0001, f32::MAX);
        if let Some(hit) = hit_info {
            // 3項演算子ってないん?
            let scatter_info = if depth > 0 {
                hit.m.scatter(&ray, &hit)
            } else {
                None
            };
            if let Some(scatter) = scatter_info {
                // depth-1をここで記述するのはおかしい
                return scatter.albedo * self.trace(scatter.ray, depth - 1);
            } else {
                return Vec3::ZERO;
            }
        } else {
            self.background(ray.direction)
        }
    }
}

fn main() {
    render_aa(RandomScene::new());
}
