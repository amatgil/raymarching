use raymarcher::*;

const CAM_WIDTH: usize  = 300;
const CAM_HEIGHT: usize = 300;

fn main() {
    let scene = Scene {
        cam: Camera::facing_towards(Vec3::ZERO, Vec3::X, CAM_WIDTH, CAM_HEIGHT),
        objs: vec![
            Shape {
                pos: Vec3::new(20.0, 0.0, 0.0),
                kind: ShapeKind::Sphere { radius: 1.0 },
                color: Pixel::new(255,0,0) }
        ]
    };

    dbg!(scene.shoot_ray_from_cam(0,0));
    dbg!(scene.shoot_ray_from_cam(CAM_WIDTH as isize/2, CAM_HEIGHT as isize/2));
    dbg!(scene.shoot_ray_from_cam(CAM_WIDTH as isize/-2, CAM_HEIGHT as isize/-2));

    let r = scene.render();
    println!("{}", r.to_string());
}
