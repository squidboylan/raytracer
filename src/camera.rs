use super::vector::Vector3D;
use super::color::Color;
use std::f32::consts::PI;

pub fn generate_rays(focal_point: Vector3D, resolution_w: usize, resolution_h: usize, aa: usize) -> Vec<Pixel> {
    let mut pixels = Vec::with_capacity(resolution_w * resolution_h);

    // This code is based on the camera code in
    // https://www.scratchapixel.com/code.php?id=3&origin=/lessons/3d-basic-rendering/introduction-to-ray-tracing
    let aspectratio = resolution_w as f64/resolution_h as f64;

    let aa_res_w = resolution_w * aa;
    let aa_res_h = resolution_h * aa;

    let inv_w = 1.0/(aa_res_w as f64);
    let inv_h = 1.0/(aa_res_h as f64);

    let fov = 60.0_f64;
    let angle = (PI as f64 * 0.5 * fov / 180.0).tan();

    for i in 0..resolution_h * resolution_w {
        pixels.push(Pixel::new(aa));
    }

    for j in 0..aa_res_h {
        let j_f64 = j as f64;
        let y = (1.0 - 2.0 * ((j_f64 + 0.5) * inv_h)) * angle;
        for i in 0..aa_res_w {
            let i_f64 = i as f64;
            let x = (2.0 * ((i_f64 + 0.5) * inv_w) - 1.0) * angle * aspectratio;
            let direction = Vector3D([x, y, -1.0]);

            pixels[(j/aa * resolution_w) + i/aa].rays.push(Ray::new(focal_point, direction.normal()));
        }
    }

    pixels
}

pub struct Pixel {
    pub rays: Vec<Ray>,
}

impl Pixel {
    pub fn new(capacity: usize) -> Self {
        Pixel {
            rays: Vec::with_capacity(capacity),
        }
    }
}

pub struct Ray {
    pub direction: Vector3D,
    pub origin: Vector3D,
}

impl Ray {
    pub fn new(direction: Vector3D, origin: Vector3D) -> Self {
        Ray {
            direction,
            origin,
        }
    }
}
