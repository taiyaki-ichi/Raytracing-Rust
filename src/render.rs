use glam::{vec3, Vec3};
use image::{Rgb, RgbImage};
use rayon::prelude::*;

use crate::{camera::Camera, ray::Ray};

const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = 100;
const OUTPUT_FILENAME: &str = "render.png";

// 外部で定義された構造体にメンバ関数を追加できないっぽいので普通の関数で
fn map<F>(v: Vec3, f: F) -> Vec3
where
    F: Fn(f32) -> f32,
{
    vec3(f(v.x), f(v.y), f(v.z))
}

//
fn to_rgb(v: Vec3) -> [u8; 3] {
    let rgb = map(v, |e| 255.99 * e.min(1.0).max(0.0));
    [rgb.x as u8, rgb.y as u8, rgb.z as u8]
}

pub trait Scene {
    fn camera(&self) -> Camera;
    // 戻り値は色を表す
    fn trace(&self, ray: Ray) -> Vec3;
    // デフォルト実装もできるっぽい
    fn width(&self) -> u32 {
        IMAGE_WIDTH
    }
    // widthとheightのフォーマット整えたら1行からバラされた
    // rust-analyzer的にはこれが決まった関数の書き方っぽい
    fn height(&self) -> u32 {
        IMAGE_HEIGHT
    }
    fn aspect(&self) -> f32 {
        // as演算子の優先度たっか
        self.width() as f32 / self.height() as f32
    }
}

// 具体的な型の代わりにトレイトを指定できる
// トレイトの制約をトレイト境界というっぽい
// where句でもうちょい明確に記述することもできるっぽい
pub fn render(scene: impl Scene + Sync) {
    let camera = scene.camera();
    let mut img = RgbImage::new(scene.width(), scene.height());
    img.enumerate_pixels_mut()
        // ここのcollectの引数の型、推論してほしいんだけど
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let u = *x as f32 / (scene.width() - 1) as f32;
            let v = (scene.height() - *y - 1) as f32 / (scene.height() - 1) as f32;
            let ray = camera.ray(u, v);
            // 変数をスネークケースで記述しろって警告が出のでそうした
            // 何が主流なんだ?
            let rgb_vec3 = scene.trace(ray);
            // 配列とか一部の方はバラして束縛できるっぽい
            // C++の構造化束縛みたいに通常の構造体はできないっぽい
            let [r, g, b] = to_rgb(rgb_vec3);
            pixel[0]=r;
            pixel[1]=g;
            pixel[2]=b;
        });
    
    img.save(OUTPUT_FILENAME).unwrap();
}
