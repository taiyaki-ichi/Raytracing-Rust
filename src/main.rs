// これないとダメらしい
// rust-analyzerも動かなかったし
mod camera;
mod ray;
mod render;

use camera::Camera;
use glam::{vec3, Vec2, Vec3};
use image::{Rgb, RgbImage};
// prelude::*はfor_eachとか用
use rayon::{iter::IntoParallelRefMutIterator, prelude::*};
use render::{Scene, render};

use crate::ray::Ray;

// 空のクラスだ
// トレイツベースでプログラミングしてくとこんなクラスが増えてくるのかな
struct SimpleScene{}

impl SimpleScene{
    // この関数, メンバとして実装する意味あるのか?
    // あー, 関数をいくつも適用する際には便利なのか
    // ベクトル演算のライブラリのglamもメンバ関数ばっかだし
    // ベターな方法っぽいな
    fn hit_sphere(&self,center: Vec3, radius: f32, ray: &Ray) -> f32 {
        let oc = ray.origin - center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - radius.powi(2);
        let d = b * b - 4.0 * a * c;
    
        // 円と直線の交点が1つもない場合
        // つまり円と直線が交差していない場合
        if d < 0.0 {
            -1.0
        }
        // 交差している場合は交点の媒介変数を返す
        // このコードは2次方程式の解の公式まんまの形になっている
        else {
            (-b - d.sqrt()) / (2.0 * a)
        }
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
        let circle_center=vec3(0.0, 0.0, -1.0);
        let t=self.hit_sphere(circle_center,0.5,&ray);
        if t > 0.0 {
            let n = (ray.at(t) - circle_center).normalize();
            // わざわざreturn書きたくないのだが
            return 0.5 * (n + Vec3::ONE);
        }
        self.background(ray.direction)
    }
}

fn main() {

    // {}でも構造体はインスタンス化できんのか
    render(SimpleScene{});
}
