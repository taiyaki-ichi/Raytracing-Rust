// これないとダメらしい
// rust-analyzerも動かなかったし
mod ray;
mod camera;

use glam::{Vec2, Vec3};
use image::{Rgb, RgbImage};
// prelude::*はfor_eachとか用
use rayon::{prelude::*,iter::IntoParallelRefMutIterator};

use crate::ray::Ray;

fn main() {
    let a = Vec2::new(1.0, 1.0);
    let b = Vec2::new(0.0, 1.0);

    // 内積
    let c = a.dot(b);

    // 結果の確認
    // びっくりマークがつく関数はNever型と言うものを返すらしい
    println!("{}", c);

    let mut img = RgbImage::new(100, 100);
    img.enumerate_pixels_mut()
        // collectはFromIterator<Self::Item>トレイトに含まれる
        // [char]からStringへの変換とか行う時なんかに使う
        // あと、値と関数のペアの配列の要素を評価したりする時とか
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(_, _, pixel)| {
            pixel[0] = 255;
            pixel[1] = 0;
            pixel[2] = 0;
        });
    
    // 結果の確認
    // hoge.pngは後で消すこと
    img.save(String::from("hoge.png")).unwrap();

    let r=Ray::new(Vec3::new(0.0,0.0,0.0),Vec3::new(1.0,0.0,0.0));
    println!("{}",r.at(10.0));

}
