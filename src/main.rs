extern crate image;
extern crate crossbeam;

mod vector;
mod object;
mod camera;
mod color;

use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use camera::Ray;
use object::Sphere;
use object::Object;
use std::thread;
use std::sync::Arc;

const IMAGE_RES: (usize, usize) = (1960, 1080);
const AA: usize = 4;

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
    let camera_focal = vector::Vector3D([0.0, 0.0, 100.0]);
    let rays = camera::generate_rays(camera_focal, IMAGE_RES.0 * AA, IMAGE_RES.1 * AA);

    let mut objects = Vec::new();

    let sphere_center = vector::Vector3D([0.0, 0.0, 0.0]);
    objects.push(Sphere::new(sphere_center, 0.1, color::Color([255, 0, 0])));

    let sphere_center = vector::Vector3D([0.2, 0.0, 0.0]);
    objects.push(Sphere::new(sphere_center, 0.1, color::Color([255, 255, 0])));

    let sphere_center = vector::Vector3D([0.0, 0.2, 0.0]);
    objects.push(Sphere::new(sphere_center, 0.1, color::Color([255, 0, 255])));

    let sphere_center = vector::Vector3D([0.2, 0.2, 0.0]);
    objects.push(Sphere::new(sphere_center, 0.1, color::Color([0, 255, 0])));

    let objects = Arc::new(objects);

    let nthreads = 8;
    let rows_per_band = IMAGE_RES.1 / nthreads + 1;

    let mut pixels: Vec<u8> = Vec::with_capacity(IMAGE_RES.0 * IMAGE_RES.1 * 3);
    {
        let bands: Vec<&[Ray]> =
            rays.chunks(rows_per_band * IMAGE_RES.0).collect();

        crossbeam::scope(|spawner| {
            let mut threads = Vec::new();
            for band in bands.into_iter() {
                let objects1 = objects.clone();
                let handle = spawner.spawn(move || {
                    render(band, &objects1)
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
fn render(rays: &[Ray], objects: &Vec<Sphere>) -> Vec<u8> {
    let mut pixels: Vec<u8> = Vec::with_capacity(rays.len() * 3);
    for i in rays {
        let mut color: [u8; 3] = [255, 255, 255];
        for j in objects {
            let collision = j.get_collision(&i);
            if collision != None {
                let val = collision.unwrap();
                color = j.get_color(i.origin + i.direction.mul_f64(val)).0;
            }
        }
        pixels.push(color[0]);
        pixels.push(color[1]);
        pixels.push(color[2]);
    }

    pixels
}
