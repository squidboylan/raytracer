use super::vector::Vector3D;
use super::color::Color;
use std::f32::consts::PI;

pub struct Camera {
    sub_pixels: Vec<Ray>,
    curr: usize,
}

impl Camera {
    pub fn new(focal_point: Vector3D, resolution_w: usize, resolution_h: usize) -> Self {
        let mut sub_pixels = Vec::with_capacity(resolution_w * resolution_h);

        // This code is based on the camera code in
        // https://www.scratchapixel.com/code.php?id=3&origin=/lessons/3d-basic-rendering/introduction-to-ray-tracing
        let aspectratio = resolution_w as f64/resolution_h as f64;

        let inv_w = 1.0/(resolution_w as f64);
        let inv_h = 1.0/(resolution_h as f64);

        let fov = 60.0_f64;
        let angle = (PI as f64 * 0.5 * fov / 180.0).tan();

        for j in 0..resolution_h {
            let j = j as f64;
            let y = (1.0 - 2.0 * ((j + 0.5) * inv_h)) * angle;
            for i in 0..resolution_w {
                let i = i as f64;
                let x = (2.0 * ((i + 0.5) * inv_w) - 1.0) * angle * aspectratio;
                let direction = Vector3D([x, y, -1.0]);

                sub_pixels.push(Ray::new(focal_point, direction.normal()));
            }
        }

        Camera {
            sub_pixels,
            curr: 0,
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
