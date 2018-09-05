use std::ops;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector {
    pub fn new (x:f32, y: f32, z: f32) -> Vector {
        Vector {
            x,
            y,
            z
        }
    }

    pub fn zero() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn mult(&self, rhs: &Vector) -> Vector {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }

    pub fn normify(&mut self)  {
        let result =  *self * (1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt());
        self.x = result.x;
        self.y = result.y;
        self.x = result.z;
    }

    pub fn dot(&self, rhs: &Vector)  -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.x * self.z * self.z
    }
}


impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl  ops::Rem<Vector> for Vector {
    type Output = Vector;
    fn rem(self, rhs: Vector) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}

