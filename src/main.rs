use glam::*;

fn main() {

    let a=Vec2::new(1.0,1.0);
    let b=vec2(0.0,1.0);

    // 内積
    let c=a.dot(b);

    println!("{}",c);
}
