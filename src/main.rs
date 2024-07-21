use raymarcher::*;

const CAM_WIDTH: usize  = 800; // Must be even, I think
const CAM_HEIGHT: usize = 800;


fn main() {
    let scene = Scene {
        cam: Camera::facing_towards(Vec3::new(-30.0, 0.0, 3.0), Vec3::ZERO, CAM_WIDTH, CAM_HEIGHT),
        objs: vec![
            Shape {
                pos: Vec3::new(1.0, 0.0, 0.0),
                kind: ShapeKind::Sphere { radius: 0.35  },
                color: Pixel::new(255,0,0)
            },
            Shape {
                pos: Vec3::new(0.0, 0.2, 0.1),
                kind: ShapeKind::Sphere { radius: 0.3 },
                color: Pixel::new(0,255,0)
            },
            Shape {
                pos: Vec3::new(-1.0, -0.2, 0.2),
                kind: ShapeKind::Box { dims: Vec3::new(0.2, 0.3, 0.1) },
                color: Pixel::new(0,0,255)
            }
        ]
    };

    println!("[INFO]: Starting render");

    let r = scene.render();

    println!("[INFO]: Render finished, saving to file");

    r.save("test.ppm").unwrap();

    println!("[INFO]: Render saved to file");
}
