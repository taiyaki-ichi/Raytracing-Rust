use glam::Vec3;

use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    // World空間でのカメラの位置
    pub origin: Vec3,
    // World空間でのView空間の右方向
    pub u: Vec3,
    // World空間でのView空間の上方向
    pub v: Vec3,
    // World空間でのView空間の前方向
    pub w: Vec3,
}

impl Camera {
    pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Self {
        Self {
            origin: Vec3::ZERO,
            u,
            v,
            w,
        }
    }

    // vfovは画角
    pub fn from_lookat(origin: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let halfh = (vfov.to_radians() * 0.5).tan();
        let halfw = aspect * halfh;
        let w = (origin - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        let uw = halfw * u;
        let vh = halfh * v;
        Self {
            origin,
            u: 2.0 * uw,
            v: 2.0 * vh,
            w: origin - uw - vh - w,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.w + self.u * u + self.v * v - self.origin,
        }
    }
}
