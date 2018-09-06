mod vector;
mod object;
mod camera;
mod color;

const image_res: (usize, usize) = (1960, 1080);

fn main() {
    let camera_focal = vector::Vector3D([0.0, 0.0, 10.0]);
    let camera = camera::Camera::new(camera_focal, image_res.0, image_res.1);
}

