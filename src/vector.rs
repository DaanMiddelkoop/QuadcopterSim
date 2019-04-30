pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3f {
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

    pub fn mult(&self, a: f64) -> Vector3f {
        Vector3f::new(self.x * a, self.y * a, self.z * a)
    }

    pub fn normalize(&self) -> Vector3f {
        let length = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        self.mult(1.0 / length)
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn angle(&self, other: &Vector3f) -> f64 {
        let ab = (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
        (ab / (self.length() * other.length())).acos()
    }

    pub fn dot(&self, other: &Vector3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn set_length(&self, l: f64) -> Vector3f {
        self.mult(l / self.length())
    }
}