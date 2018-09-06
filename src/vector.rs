use std::ops::*;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vector3D(
    pub [f64; 3]
);

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        let self_val = self.0;
        let other_val = other.0;
        Vector3D([self_val[0] + other_val[0], self_val[1] + other_val[1], self_val[2] + other_val[2]])
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, other: Vector3D) -> Vector3D {
        let self_val = self.0;
        let other_val = other.0;
        Vector3D([self_val[0] - other_val[0], self_val[1] - other_val[1], self_val[2] - other_val[2]])
    }
}

impl Deref for Vector3D {
    type Target = [f64; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vector3D {
    fn deref_mut (&mut self) -> &mut [f64; 3] {
        &mut self.0
    }
}

#[test]
fn test_Vector3D() {
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
