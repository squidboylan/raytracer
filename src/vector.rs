use std::ops::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vector3D(
    pub [f32; 3]
);

impl Vector3D {
    pub fn normal(&self) -> Self {
        let vals = &self.0;
        let magnitude = (vals[0].powi(2) + vals[1].powi(2) + vals[2].powi(2)).sqrt();
        Vector3D([vals[0]/magnitude, vals[1]/magnitude, vals[2]/magnitude])
    }

    pub fn normalize(&mut self) {
        let vals = &mut self.0;
        let magnitude = (vals[0].powi(2) + vals[1].powi(2) + vals[2].powi(2)).sqrt();
        vals[0] /= magnitude;
        vals[1] /= magnitude;
        vals[2] /= magnitude;
    }

    pub fn dot(&self, vec2: &Vector3D) -> f32 {
        let vals = &self.0;
        let vec2_vals = vec2.0;
        vals[0] * vec2_vals[0] + vals[1] * vec2_vals[1] + vals[2] * vec2_vals[2]
    }

    pub fn mul_f32(&self, scalar: f32) -> Vector3D {
        let vals = &self.0;
        Vector3D([vals[0] * scalar, vals[1] * scalar, vals[2] * scalar])
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        let self_val = &self.0;
        let other_val = &other.0;
        Vector3D([self_val[0] + other_val[0], self_val[1] + other_val[1], self_val[2] + other_val[2]])
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        let self_val = &self.0;
        let other_val = &other.0;
        Vector3D([self_val[0] - other_val[0], self_val[1] - other_val[1], self_val[2] - other_val[2]])
    }
}

impl Deref for Vector3D {
    type Target = [f32; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vector3D {
    fn deref_mut (&mut self) -> &mut [f32; 3] {
        &mut self.0
    }
}

#[test]
fn test_vector3d() {
    let mut v1: Vector3D = Vector3D( [1.0, 2.0, 3.0] );
    let mut v2: Vector3D = Vector3D( [3.0, 2.0, 1.0] );

    // Test impl Sub
    assert_eq!(v1 - v2, Vector3D( [-2.0, 0.0, 2.0] ));
    assert_eq!(v2 - v1, Vector3D( [2.0, 0.0, -2.0] ));

    // Test impl Add
    assert_eq!(v1 + v2, Vector3D( [4.0, 4.0, 4.0] ));

    // Test impl Deref
    assert_eq!(v1[0], 1.0);

    // Test impl DerefMut
    v1[0] = 4.0;
    assert_eq!(v1[0], 4.0);
}
