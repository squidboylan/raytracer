extern crate image;

mod vector;
mod object;
mod camera;
mod color;

use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

const IMAGE_RES: (usize, usize) = (1960, 1080);

fn write_image(filename: &str, pixels: &[u8])
    -> Result<(), std::io::Error> {

    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
                   IMAGE_RES.0 as u32, IMAGE_RES.1 as u32,
                   ColorType::RGB(8))?;
    Ok(())
}

fn main() {
    let camera_focal = vector::Vector3D([0.0, 0.0, 10.0]);
    let camera = camera::Camera::new(camera_focal, IMAGE_RES.0, IMAGE_RES.1);
    let sphere_center = vector::Vector3D([0.0, 0.0, 0.0]);
    let objects = vec![object::Sphere::new(sphere_center, 1.0, color::Color([255, 0, 0]))];
}
