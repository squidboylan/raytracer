extern crate image;
extern crate crossbeam;

mod vector;
mod object;
mod camera;
mod color;

use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use camera::*;
use object::Sphere;
use object::Object;
use std::thread;
use std::sync::Arc;

const IMAGE_RES: (usize, usize) = (1920, 1080);
const AA: usize = 32;

// Write the image to a file
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
    let nthreads = 16;
    let view_port_chunks = generate_view_port_chunks(camera_focal, IMAGE_RES.0, IMAGE_RES.1, AA, nthreads);

    let mut objects = Vec::new();

    let sphere_center = vector::Vector3D([0.0, 0.0, 0.0]);
    objects.push(Sphere::new(sphere_center, 1.0, color::Color([255, 0, 0])));

    let sphere_center = vector::Vector3D([2.0, 0.0, 0.0]);
    objects.push(Sphere::new(sphere_center, 1.0, color::Color([255, 255, 0])));

    let sphere_center = vector::Vector3D([0.0, 2.0, 0.0]);
    objects.push(Sphere::new(sphere_center, 1.0, color::Color([255, 0, 255])));

    let sphere_center = vector::Vector3D([2.0, 2.0, 0.0]);
    objects.push(Sphere::new(sphere_center, 1.0, color::Color([0, 255, 0])));

    let objects = Arc::new(objects);

    let mut pixels: Vec<u8> = Vec::with_capacity(IMAGE_RES.0 * IMAGE_RES.1 * 3);
    {
        crossbeam::scope(|spawner| {
            let mut threads = Vec::new();
            for view_port in view_port_chunks {
                let objects1 = objects.clone();
                let handle = spawner.spawn(move || {
                    render(view_port, &objects1)
                });
                threads.push(handle);
            }
            for i in threads {
                pixels.append(&mut i.join().unwrap());
            }
        });
    }

    write_image("output.png", &pixels).unwrap();
}

// Convert the ray and object data to a vector that represents pixels
fn render(view_port_chunk: ViewPortChunk, objects: &Vec<Sphere>) -> Vec<u8> {
    let size_hint = view_port_chunk.size_hint();
    let pixels_n = size_hint.1.unwrap() - size_hint.0;
    let mut pixels: Vec<u8> = Vec::with_capacity(pixels_n * 3);

    for pixel in view_port_chunk {
        let mut final_color: [u16; 3] = [255, 255, 255];
        for (num, i) in pixel.rays.iter().enumerate() {
            let mut color: [u8; 3] = [255, 255, 255];
            for j in objects {
                let collision = j.get_collision(&i);
                if collision != None {
                    let val = collision.unwrap();
                    color = j.get_color(i.origin.clone() + i.direction.mul_f32(val)).0;
                }
            }
            // Calculate the AA stuff
            if num == 0 {
                final_color[0] = color[0] as u16;
                final_color[1] = color[1] as u16;
                final_color[2] = color[2] as u16;
            } else {
                final_color[0] = (final_color[0] + color[0] as u16)/2;
                final_color[1] = (final_color[1] + color[1] as u16)/2;
                final_color[2] = (final_color[2] + color[2] as u16)/2;
            }
        }
        pixels.push(final_color[0] as u8);
        pixels.push(final_color[1] as u8);
        pixels.push(final_color[2] as u8);
    }

    pixels
}
