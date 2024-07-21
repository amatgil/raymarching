
use std::{fmt::Display, fs::File, io::Write, path::PathBuf};
use rayon::prelude::*;
pub use glam::Vec3A as Vec3;
use itertools::Itertools; // I ain't typing all that
use indicatif::ProgressIterator;
use indicatif::ParallelProgressIterator;
use std::ops::Mul;
use std::f32::consts::TAU;

// ==============   Math stuff   ===============
#[derive(Clone, Debug)]
pub struct Scene {
    pub cam: Camera,
    pub objs: Vec<Shape>
}

#[derive(Clone, Debug, Copy)]
pub struct Ray {
    pub source: Vec3,
    pub dir: Vec3
}

impl Scene {
    /// How far do we have to be to call it a collision
    const RAY_DELTA: f32 = 0.001; 

    /// How far do we have to be to call it quits
    const RAY_WONT_HIT_ANYTHING: f32 = 10000.0; 

    /// Bg color
    pub const BACKGROUND: Pixel = Pixel::new(138, 173, 244); // How far do we have to be to call it a collision

    /// (0,0) would be the center pixel of the camera. Negative is up, positive is down
    pub fn shoot_ray_from_cam(&self, x: isize, y: isize) -> Pixel {
        let source = self.cam.as_world_space(x, y);
        let dir = self.cam.dir;

        self.shoot_ray(Ray { source, dir })
    }
    fn shoot_ray(&self, mut ray: Ray) -> Pixel {
        loop {
            let Some((hit_obj, min_dist)) = self.objs.iter().map(|o| (o, o.distance_from(ray.source))).min_by(|(_, a), (_, b)| a.total_cmp(b))
            else { return Self::BACKGROUND }; // No shapes, make it bg

            if min_dist <= Self::RAY_DELTA {
                let shading: f32 = {
                    let a = hit_obj.gradient_at(ray.source);
                    let b = ray.dir;
                    let theta = a.angle_between(b);
                    theta / (TAU/2.0) 
                };
                return hit_obj.color * shading;
            }
            else if min_dist >= Self::RAY_WONT_HIT_ANYTHING { return Self::BACKGROUND; }

            ray.source += (ray.dir)*min_dist;
        }
    }
    pub fn render(&self) -> Image {

        let pixels =         (self.cam.height as isize/-2..self.cam.height as isize/2)
            .cartesian_product(self.cam.width as isize/-2..self.cam.width as isize/2)
            .map(|(y, x)| self.shoot_ray_from_cam(x, y))
            .progress_count((self.cam.width*self.cam.height) as u64)
            .collect();
        println!("[INFO]: Render done, returning");

        Image { width: self.cam.width, height: self.cam.height, pixels }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Shape {
    pub pos: Vec3,
    pub kind: ShapeKind,
    pub color: Pixel
    // material, ... eventually
}

#[derive(Clone, Debug, Copy)]
#[non_exhaustive]
pub enum ShapeKind {
    Sphere { radius: f32 },
    Box { dims: Vec3 }
}

impl Shape {
    fn distance_from(&self, p: Vec3) -> f32 {
        match self.kind {
            ShapeKind::Sphere { radius } => (self.pos - p).length() - radius,
            ShapeKind::Box { dims: b }   => {
                let q = p.abs() - b;
                (q.max(Vec3::ZERO)
                 + 0.0f32.min([q.x, q.y, q.z].into_iter().max_by(|a,b| a.total_cmp(b)).unwrap()))
                    .length()
            }
        }
    }
    fn gradient_at(&self, p: Vec3) -> Vec3 {
        const h: f32 = 0.0001;
        let f_p = self.distance_from(p); // f(p) (should be 0, is approx 0)
        Vec3::new(
            self.distance_from(p + h*Vec3::X) - f_p,
            self.distance_from(p + h*Vec3::Y) - f_p,
            self.distance_from(p + h*Vec3::Z) - f_p,
        ).normalize()
    }
}


#[derive(Clone, Debug, Copy)]
pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
    pub width: usize,
    pub height: usize
}

impl Camera {
    pub fn as_world_space(&self, x: isize, y: isize) -> Vec3 {
        let x_delta = x as f32 / self.width  as f32;
        let y_delta = y as f32 / self.height as f32;

        let left_normal = self.dir.cross(Vec3::Z).normalize();  // Z IS UP

        //         x shift (in the x-y plane) y shift (which upwards for the camera, Z)
        self.pos + (x_delta * left_normal) + (y_delta * Vec3::Z)
    }
    pub fn facing_towards(start: Vec3, end: Vec3, width: usize, height: usize) -> Self {
        let dir = end - start;
        Self { pos: start, dir, width, height }
    }
}


// ==============   Visual stuff   ===============
#[derive(Clone, Debug, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Clone, Debug)]
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>
}

impl Pixel {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Image {
    pub fn new(width: usize, height: usize, bg: Pixel) -> Self {
        Self { width, height, pixels: vec![bg; width*height] }
    }
    pub fn save(&self, path: impl Into<PathBuf>) -> Result<(), ()> {
        let mut file = File::create(path.into()).map_err(|_| ())?;
        file.write(self.to_string().as_bytes()).map_err(|_| ())?;
        Ok(())
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P3\n{} {}\n255\n{}\n",
               self.width,
               self.height,
               self.pixels.iter().map(|p| p.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

impl Mul<f32> for Pixel {
    type Output = Self;
    fn mul(self, v: f32) -> <Self as Mul<f32>>::Output {
        let f = |w: u8| (w as f32 * v) as u8;
        Pixel::new(f(self.r), f(self.g), f(self.b))
    }
}
