use super::vector::Vector3D;
use super::color::Color;
use super::camera::*;

trait Object {
    // Returns the normal of the object at that coordinate
    fn get_normal(&self, coordinates: Vector3D) -> Vector3D;

    // Returns the color of the object at that coordinate
    fn get_color(&self, coordinates: Vector3D) -> Color;

    // Returns the distance along the ray in which there is a collision with the
    // object
    fn get_collision(&self, ray: &Ray) -> f64;
}

pub struct Sphere {
    center: Vector3D,
    radius: f64,
    color: Color,
}

impl Sphere {
    pub fn new(center: Vector3D, radius: f64, color: Color) -> Self {
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

    fn get_collision(&self, ray: &Ray) -> f64 {
        // Equation for this was obtained from
        // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
        let b = ray.direction.dot(&(ray.origin - self.center));
        let tmp = ray.origin - self.center;
        let c = tmp.dot(&tmp) - self.radius.powi(2);
        let val = b * b - c;

        let p1 = -b - val.sqrt();
        let p2 = -b + val.sqrt();
        if p1 < p2 {
            return p1;
        } else {
            return p2;
        }
    }
}
