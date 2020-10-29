use crate::vector::Vec3d;
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct AABB
{
    pub p_min: Vec3d,
    pub p_max: Vec3d,
}

impl AABB
{
    pub fn single(p: Vec3d) -> AABB
    {
        AABB{ p_min: p, p_max:p }
    }
    pub fn new(p1: Vec3d, p2: Vec3d) -> AABB
    {
        let p_min = Vec3d::comp_min(p1, p2);
        let p_max = Vec3d::comp_max(p1, p2);
        AABB{ p_min, p_max }
    }
    pub fn corner(&self, mut index: usize) -> Vec3d
    {
        if index >= 8
        {
            index = index % 8;
        }
        let x = if index%2 == 0 {self.p_min.x } else { self.p_max.x };
        let y = if index%4 < 2 { self.p_min.y } else { self.p_max.y };
        let z = if index < 4 { self.p_min.z } else { self.p_max.z };
        Vec3d::new(x, y, z)
    }
    pub fn union_point(b: &AABB, p: Vec3d) -> AABB
    {
        let p_min = Vec3d::comp_min(b.p_min, p);
        let p_max = Vec3d::comp_max(b.p_max, p);
        AABB::new(p_min, p_max)
    }
    pub fn union_box(b1: &AABB, b2: &AABB) -> AABB
    {
        let p_min = Vec3d::comp_min(b1.p_min, b2.p_min);
        let p_max = Vec3d::comp_max(b1.p_max, b2.p_max);
        AABB::new(p_min, p_max)
    }
    pub fn intersection(b1: &AABB, b2: &AABB) -> AABB
    {
        let p_min = Vec3d::comp_max(b1.p_min, b2.p_min);
        let p_max = Vec3d::comp_min(b1.p_max, b2.p_max);
        AABB::new(p_min, p_max)
    }
    pub fn overlap(b1: &AABB, b2: &AABB) -> bool
    {
        let x = b1.p_max.x >= b2.p_min.x && b1.p_min.x >= b2.p_max.x;
        let y = b1.p_max.y >= b2.p_min.y && b1.p_min.y >= b2.p_max.y;
        let z = b1.p_max.z >= b2.p_min.z && b1.p_min.z >= b2.p_max.z;
        x && y && z
    }
    pub fn inside(b: &AABB, p: Vec3d) -> bool
    {
        p.x >= b.p_min.x && p.x <= b.p_max.x &&
        p.y >= b.p_min.y && p.y <= b.p_max.y &&
        p.z >= b.p_min.z && p.z <= b.p_max.z
    }
    pub fn diagonal(&self) -> Vec3d
    {
        self.p_max-self.p_min
    }
    pub fn surface_area(&self) -> f64
    {
        let d = self.diagonal();
        2. * (d.x*d.x + d.y*d.y + d.z*d.z)
    }
    pub fn volumn(&self) -> f64
    {
        let d = self.diagonal();
        d.x*d.y*d.z
    }
    pub fn hit(&self, r: &Ray) -> bool
    {
        let t1 = (self.p_min.x - r.o.x)/r.d.x;
        let t2 = (self.p_max.x - r.o.x)/r.d.x;
        let t3 = (self.p_min.y - r.o.y)/r.d.y;
        let t4 = (self.p_max.y - r.o.y)/r.d.y;
        let t5 = (self.p_min.z - r.o.z)/r.d.z;
        let t6 = (self.p_max.z - r.o.z)/r.d.z;

        let tmin = f64::max(f64::max(f64::min(t1, t2), f64::min(t3, t4)), f64::min(t5, t6));
        let tmax = f64::min(f64::min(f64::max(t1, t2), f64::max(t3, t4)), f64::max(t5, t6));

        if tmax < 0.
        {
            return false;
        }
        else if tmin > tmax
        {
            return false;
        }
        else if tmin > r.tmax
        {
            return false;
        }   
        else
        {
            return true;
        }
    }
}