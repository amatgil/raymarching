use raymarcher::*;

const CAM_WIDTH: usize  = 600; // Must be even, I think
const CAM_HEIGHT: usize = 600;

use std::f32::consts::TAU;


fn main() {
    let path_points = 100;
    let radius = 30.0;
    let cam_path: Vec<Vec3> = (0..path_points)
        .map(|n| (n as f32 / (path_points-1) as f32)*TAU)
        .map(|t| Vec3::new(t.cos(), t.sin(), 10.0))
        .collect();

    for (i, cp) in cam_path.into_iter().enumerate() {
        let scene = Scene {
            cam: Camera::facing_towards(cp, Vec3::ZERO, CAM_WIDTH, CAM_HEIGHT),
            objs: vec![
                Shape {
                    pos: Vec3::new(0.75, 0.0, 0.0),
                    kind: ShapeKind::Sphere { radius: 0.35  },
                    color: Pixel::new(255,0,0)
                },
                Shape {
                    pos: Vec3::new(0.0, 0.2, 0.1),
                    kind: ShapeKind::Sphere { radius: 0.3 },
                    color: Pixel::new(0,255,0)
                },
                Shape {
                    pos: Vec3::new(-0.75, -0.2, 0.2),
                    kind: ShapeKind::Box { dims: Vec3::new(0.2, 0.3, 0.1) },
                    color: Pixel::new(0,0,255)
                }
            ]
        };

        println!("[INFO]: Starting render {i}");

        let r = scene.render();

        println!("[INFO]: Render {i} finished, saving to file");

        r.save(&format!("test-{:0>2}.ppm", i)).unwrap();

        println!("[INFO]: Render saved to file");
    }
}
