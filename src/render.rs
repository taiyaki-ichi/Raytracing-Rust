use glam::{vec3, Vec3};
use image::{Rgb, RgbImage};
use rayon::prelude::*;

use crate::{
    camera::Camera,
    ray::Ray,
    utility::{gamma, to_rgb},
};

const IMAGE_WIDTH: u32 = 200;
const IMAGE_HEIGHT: u32 = 100;
const OUTPUT_FILENAME: &str = "render.png";
const SAMPLES_PER_PIXEL: usize = 64;
const GAMMA_FACTOR: f32 = 2.2;
const MAX_RAY_BOUNCE_DEPTH: usize = 50;

pub trait Scene {
    fn camera(&self) -> Camera;
    // 戻り値は色を表す
    // depthは反射回数を打ち切る用
    // depthの処理をユーザに定義させるのはおかしいので変更する
    fn trace(&self, ray: Ray, depth: usize) -> Vec3;
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

    // 1ピクセル当たりのサンプル数
    fn spp(&self) -> usize {
        SAMPLES_PER_PIXEL
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
            let rgb_vec3 = scene.trace(ray, MAX_RAY_BOUNCE_DEPTH);
            // 配列とか一部の方はバラして束縛できるっぽい
            // C++の構造化束縛みたいに通常の構造体はできないっぽい
            let [r, g, b] = to_rgb(rgb_vec3);
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        });

    img.save(OUTPUT_FILENAME).unwrap();
}

// アンチエイリアシングver
pub fn render_aa(scene: impl Scene + Sync) {
    let camera = scene.camera();
    let mut img = RgbImage::new(scene.width(), scene.height());
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            // into_iterはiterとは違うぞ
            // 畳み込んで合計を計算しているだけ
            let mut pixel_color = (0..scene.spp()).into_iter().fold(Vec3::ZERO, |acc, _| {
                // [0,1)->[-0.5,0.5)の範囲に写す
                let rx = rand::random::<f32>() - 0.5;
                let ry = rand::random::<f32>() - 0.5;

                let u = (*x as f32 + rx) / (scene.width() - 1) as f32;
                let v = ((scene.height() - *y - 1) as f32 + ry) / (scene.height() - 1) as f32;
                let ray = camera.ray(u, v);
                let rgb_vec3 = scene.trace(ray, MAX_RAY_BOUNCE_DEPTH);
                acc + rgb_vec3
            });
            pixel_color /= scene.spp() as f32;
            let [r, g, b] = to_rgb(gamma(pixel_color, GAMMA_FACTOR));
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
        });

    img.save(OUTPUT_FILENAME).unwrap();
}
