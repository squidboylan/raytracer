use super::vector::Vector3D;
use super::color::Color;
use std::f32::consts::PI;
use std::iter::Iterator;

pub fn generate_view_port_chunks(focal_point: Vector3D, resolution_w: usize, resolution_h: usize, aa: usize, chunks: usize) -> Vec<ViewPortChunk> {
    let total_pixel_n = resolution_w * resolution_h;
    let pixel_per_view_port_chunk = total_pixel_n/chunks + 1;

    let mut view_ports = Vec::new();
    for i in 0..chunks {
        let start = pixel_per_view_port_chunk * i;
        let end = if i < chunks - 1 {
            pixel_per_view_port_chunk * (i + 1)
        } else {
            total_pixel_n
        };
        let curr_view_port = ViewPortChunk::new(focal_point.clone(), resolution_w, resolution_h, aa, start, end);
        view_ports.push(curr_view_port);
    }

    view_ports
}

pub struct ViewPortChunk {
    focal_point: Vector3D,
    aspect_ratio: f32,
    resolution_w: usize,
    resolution_h: usize,
    aa: usize,
    inv_w: f32,
    inv_h: f32,
    fov: f32,
    angle: f32,

    // Each ViewPortChunk is responsible for [pixel_start, pixel_end) pixels
    pixel_start: usize,
    pixel_end: usize,
    curr_pixel: usize,
}

impl ViewPortChunk {
    pub fn new(focal_point: Vector3D, resolution_w: usize, resolution_h: usize, aa: usize, pixel_start: usize, pixel_end: usize) -> ViewPortChunk {
        let aspect_ratio = resolution_w as f32/resolution_h as f32;

        let aa_res_w = resolution_w * aa;
        let aa_res_h = resolution_h * aa;

        let inv_w = 1.0/(aa_res_w as f32);
        let inv_h = 1.0/(aa_res_h as f32);

        let fov = 60.0_f32;
        let angle = (PI as f32 * 0.5 * fov / 180.0).tan();

        ViewPortChunk {
            focal_point,
            aspect_ratio,
            resolution_w,
            resolution_h,
            aa,
            inv_w,
            inv_h,
            fov,
            angle,
            pixel_start,
            pixel_end,
            curr_pixel: pixel_start,
        }
    }
}

impl Iterator for ViewPortChunk {
    type Item = Pixel;

    fn next(&mut self) -> Option<Pixel> {
        if self.curr_pixel >= self.pixel_end {
            return None;
        }

        let mut pixel = Pixel::new(self.aa);
        let j = (self.curr_pixel / self.resolution_w) * self.aa;
        let i = (self.curr_pixel % self.resolution_w) * self.aa;
        for sub_j in 0..self.aa {
            let j_f32 = (j + sub_j) as f32;
            let y = (1.0 - 2.0 * ((j_f32 + 0.5) * self.inv_h)) * self.angle;
            for sub_i in 0..self.aa {
                let i_f32 = (i + sub_i) as f32;
                let x = (2.0 * ((i_f32 + 0.5) * self.inv_w) - 1.0) * self.angle * self.aspect_ratio;

                let mut direction = Vector3D([x, y, -1.0]);
                direction.normalize();

                pixel.rays.push(Ray::new(direction, self.focal_point.clone()));
            }
        }

        self.curr_pixel += 1;
        Some(pixel)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.pixel_start, Some(self.pixel_end))
    }
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
