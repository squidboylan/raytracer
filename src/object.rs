use super::vector::Vector3D;
use super::color::Color;

trait Object {
    fn get_normal(&self, coordinates: Vector3D) -> Vector3D;

    fn get_color(&self, coordinates: Vector3D) -> Color;
}

struct Circle {
    center: Vector3D,
    radius: f64,
    color: Color,
}

impl Object for Circle {
    fn get_normal(&self, coordinates: Vector3D) -> Vector3D {
        coordinates - self.center
    }

    fn get_color(&self, coordinates: Vector3D) -> Color {
        self.color
    }
}
