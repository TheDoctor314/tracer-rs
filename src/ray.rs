use crate::vec3::{Vec3, Point3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new( orig: &Point3, dir: &Vec3) -> Self {
        Self {
            orig: *orig,
            dir: *dir,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn orig(&self) -> Point3 {
        self.orig
    }
    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}
