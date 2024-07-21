use raymarcher::*;

const CAM_WIDTH: usize  = 400; // Must be even, I think
const CAM_HEIGHT: usize = 400;


fn main() {
    let scene = Scene {
        cam: Camera::facing_towards(Vec3::new(-1.2, 0.0, 0.0), Vec3::X, CAM_WIDTH, CAM_HEIGHT),
        objs: vec![
            Shape {
                pos: Vec3::new(20.0, 0.0, 0.0),
                kind: ShapeKind::Sphere { radius: 1.4 },
                color: Pixel::new(255,0,0)
            },
            Shape {
                pos: Vec3::new(15.0, 0.2, 0.1),
                kind: ShapeKind::Sphere { radius: 1.2 },
                color: Pixel::new(0,255,0)
            },
            Shape {
                pos: Vec3::new(11.0, -0.2, -0.1),
                kind: ShapeKind::Box { dims: Vec3::new(0.5, 0.5, 0.3) },
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
