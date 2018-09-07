use super::vector::Vector3D;
use super::color::Color;
use super::camera::*;

pub trait Object {
    // Returns the normal of the object at that coordinate
    fn get_normal(&self, coordinates: Vector3D) -> Vector3D;

    // Returns the color of the object at that coordinate
    fn get_color(&self, coordinates: Vector3D) -> Color;

    // Returns the distance along the ray in which there is a collision with the
    // object
    fn get_collision(&self, ray: &Ray) -> Option<f32>;
}

pub struct Sphere {
    center: Vector3D,
    radius: f32,
    color: Color,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f32, color: Color) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

impl Object for Sphere {
    fn get_normal(&self, coordinates: Vector3D) -> Vector3D {
        (coordinates - self.center).normal()
    }

    fn get_color(&self, coordinates: Vector3D) -> Color {
        self.color
    }

    fn get_collision(&self, ray: &Ray) -> Option<f32> {
        // Equation for this was obtained from
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let tmp = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&(ray.origin - self.center));
        let c = tmp.dot(&tmp) - self.radius.powi(2);
        let val = b * b - 4.0 * a * c;
        if val < 0.0 {
            return None;
        }

        let p1 = -b - val.sqrt() / (2.0 * a);
        let p2 = -b + val.sqrt() / (2.0 * a);
        if p1 < p2 {
            return Some(p1);
        } else {
            return Some(p2);
        }
    }
}
