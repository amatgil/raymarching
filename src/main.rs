use raymarcher::*;

const CAM_WIDTH: usize  = 720; 
const CAM_HEIGHT: usize = 720;

use std::f32::consts::TAU;


fn main() {
    let path_points = 100;
    let radius = 40.0;
    let cam_path: Vec<Vec3> = (0..path_points)
        .map(|n| (n as f32 / path_points as f32)*TAU)
        .map(|t| Vec3::new(radius*t.cos(), radius*t.sin(), 10.0*t.sin()*0.0))
        .collect();

    let _ = cam_path.into_iter().enumerate().map(|(i, cp)| {
        let scene = Scene {
            cam: Camera::facing_towards(cp, Vec3::ZERO, CAM_WIDTH, CAM_HEIGHT),
        objs: vec![
            Shape {
                pos: Vec3::new(0.65, 0.3, 0.0),
                rot: Mat3::IDENTITY,
                kind: ShapeKind::Sphere { radius: 0.15  },
                color: Pixel::new(255,0,0),
            },
            Shape {
                pos: Vec3::new(0.0, 0.2, 0.1),
                rot: Mat3::IDENTITY,
                kind: ShapeKind::Sphere { radius: 0.3 },
                color: Pixel::new(0,255,0)
            },
            Shape {
                pos: Vec3::new(-0.25, -0.15, -0.2),
                rot: Mat3::from_rotation_z(TAU/8.0)*Mat3::from_rotation_x(TAU/8.0)*Mat3::from_rotation_y(TAU/5.0),
                kind: ShapeKind::Box { dims: Vec3::new(0.2, 0.3, 0.1) },
                color: Pixel::new(0,0,255)
            },
            Shape {
                pos: Vec3::new(0.0, -0.2, 0.4),
                rot: Mat3::from_rotation_x(TAU/16.0),
                kind: ShapeKind::Torus { r1: 0.1, r2: 0.1 },
                color: Pixel::new(255, 255, 0)
            },
        ]
        };

        println!("[INFO]: Starting render {i} ({}%)", (i as f32/path_points as f32)*100.0);

        let r = scene.render();

        println!("[INFO]: Render {i} finished {}%, saving to file", (i as f32/path_points as f32)*100.0);

        r.save(&format!("outputs/test-{:0>4}.ppm", i)).unwrap();

        println!("[INFO]: Render saved to file");
    }).for_each(drop);
}
