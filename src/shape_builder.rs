use std::sync::Arc;

use glam::Vec3;

use crate::{
    material::{Dielectric, Lambertian, Material, Metal},
    shape::{Shape, Sphere},
};

// インスタンス化しやすくするためのBuilderパターン
pub struct ShapeBuilder {
    // Arcは共有スマートポインタのスレッドセーフバージョン
    pub material: Option<Arc<dyn Material>>,
    // Boxは共有されないスマートポインタ
    pub shape: Option<Box<dyn Shape>>,
}

impl ShapeBuilder {
    pub fn new() -> Self {
        Self {
            material: None,
            shape: None,
        }
    }

    // これ所有権が移動してるから戻り値を束縛しないとエラーでそう
    // こんな書き方もあるんだな
    pub fn lambertian(mut self, albedo: Vec3) -> Self {
        self.material = Some(Arc::new(Lambertian::new(albedo)));
        self
    }

    pub fn metal(mut self, albedo: Vec3, fuzz: f32) -> Self {
        self.material = Some(Arc::new(Metal::new(albedo, fuzz)));
        self
    }

    pub fn dielectric(mut self, ri: f32) -> Self {
        self.material = Some(Arc::new(Dielectric::new(ri)));
        self
    }

    // 材料指定->形決定
    pub fn sphere(mut self, center: Vec3, radius: f32) -> Self {
        self.shape = Some(Box::new(Sphere::new(
            center,
            radius,
            self.material.unwrap(),
        )));
        self.material = None;
        self
    }

    pub fn build(self) -> Box<dyn Shape> {
        self.shape.unwrap()
    }
}
