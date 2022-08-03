// これないとダメらしい
// rust-analyzerも動かなかったし
mod camera;
mod ray;

use camera::Camera;
use glam::{vec3, Vec2, Vec3};
use image::{Rgb, RgbImage};
// prelude::*はfor_eachとか用
use rayon::{iter::IntoParallelRefMutIterator, prelude::*};

use crate::ray::Ray;

const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = 100;

// 外部で定義された構造体にメンバ関数を追加できないっぽいので普通の関数で
fn map<F>(v: Vec3, f: F) -> Vec3
where
    F: Fn(f32) -> f32,
{
    vec3(f(v.x), f(v.y), f(v.z))
}

fn to_rgb(v: Vec3) -> [u8; 3] {
    let rgb = map(v, |e| 255.99 * e.min(1.0).max(0.0));
    [rgb.x as u8, rgb.y as u8, rgb.z as u8]
}

// とりあえずここに定義しておく
pub type Color = Vec3;

fn color(ray: &Ray) -> Color {
    let d = ray.direction.normalize();
    let t = 0.5 * (d.y + 1.0);
    Color::new(0.5, 0.7, 1.0).lerp(Color::ONE, t)
}

fn main() {
    let camera = Camera::new(
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(-2.0, 1.0, -1.0),
    );

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let u = *x as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = *y as f32 / (IMAGE_HEIGHT - 1) as f32;
            let ray = camera.ray(u, v);
            let rgbVec3 = color(&ray);
            let [r,g,b]=to_rgb(rgbVec3);
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        });

    img.save(String::from("render.png")).unwrap();
}
