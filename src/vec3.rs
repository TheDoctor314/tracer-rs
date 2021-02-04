#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }
    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z}
    }

    //test these functions!!!
    pub fn x(&mut self) -> &mut f32 {
        &mut self.x
    }
    pub fn y(&mut self) -> &mut f32 {
        &mut self.y
    }
    pub fn z(&mut self) -> &mut f32 {
        &mut self.z
    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) +
        (self.y * self.y) +
        (self.z * self.z)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    
    pub fn dot(a: &Self, b: &Self) -> f32 {
        (a.x * b.x) +
        (a.y * b.y) +
        (a.z * b.z)
    }

    pub fn cross(a: &Self, b: &Self) -> Self {
        Self {
            x: (a.y * b.z) - (a.z * b.y),
            y: (a.z * b.x) - (a.x * b.z),
            z: (a.x * b.y) - (a.y * b.x),
        }
    }

    pub fn normalize(&mut self) {
        *self /= self.length()
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from( (x, y, z) : (f32, f32, f32) ) -> Self {
        Self {x, y, z}
    }
}

use std::ops;
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self *= 1.0 / other;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        self * 1.0 / other
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

//type aliases
type Point3 = Vec3;
type Color = Vec3;
type Colour = Vec3;

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_tuple() {
        let v: Vec3 = (0f32, 4f32, 2.4f32).into();
        
        assert_eq!(v.x, 0f32);
        assert_eq!(v.y, 4f32);
        assert_eq!(v.z, 2.4f32);
    }

    #[test]
    fn test_mutable() {
        let mut v: Vec3 = (0f32, 4f32, 2.4f32).into();

        let x = v.x();
        *x = 5f32;
        assert_eq!(v.x, 5f32);
    }
    
    #[test]
    fn add_assign() {
        let mut v = Vec3::from(1f32, 2f32, 3f32);
        let u = Vec3::from(3f32, 2f32, 1f32);

        v += u;
        assert_eq!(Vec3::from(4f32, 4f32, 4f32), v);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::from(2., 4., 3.);
        let v = -v;

        assert_eq!(Vec3::from(-2., -4., -3.), v);
    }

    #[test]
    fn test_mul_and_div_assign() {
        let mut v: Vec3 = (1., 2., 3.).into();
        let t = 2f32;

        v *= t;
        assert_eq!(Vec3::from(2., 4., 6.), v);
        v /= t;
        assert_eq!(Vec3::from(1., 2., 3.), v);
    }

    #[test]
    fn test_ops() {
        let v: Vec3 = (1.0, 2.0, 3.0).into();
        let u: Vec3 = (3.0, 2.0, 1.0).into();

        assert_eq!(Vec3::from(4., 4., 4.), v + u);
        assert_eq!(Vec3::from(-2., 0., 2.), v - u);
        assert_eq!(Vec3::from(2., 4., 6.), v * 2.);

        assert_eq!( 10., Vec3::dot( &v, &u));
        assert_eq!( Vec3::from(-4., 8., -4.), Vec3::cross(&v, &u) );
    }
}
