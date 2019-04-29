pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn add(&self, other: &Vector3f) -> Vector3f {
        Vector3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vector3f) -> Vector3f {
        Vector3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn cross(&self, other: &Vector3f) -> Vector3f {
        Vector3f::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }

    pub fn mult(&self, a: f32) -> Vector3f {
        Vector3f::new(self.x * a, self.y * a, self.z * a)
    }

    pub fn normalize(&self) -> Vector3f {
        let length = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self.mult(1.0 / length)
    }
}