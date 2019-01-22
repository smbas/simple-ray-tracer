use rand::prelude::*;

use crate::geometry::{Vector3};

pub fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = Vector3::new(random::<f64>(), random::<f64>(), random::<f64>()) * 2.0 - Vector3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vector3 {
    loop {
        let p = Vector3::new(random::<f64>(), random::<f64>(), 0.0) * 2.0 - Vector3::new(1.0, 1.0, 0.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}
