use base::transformation::Transform;
use base::bounding::AABB;
use base::vector::Vec3d;
use crate::shape::Shape;
use crate::shape::Interaction;

pub struct Triangle
{
    pub p0: Vec3d,
    pub p1: Vec3d,
    pub p2: Vec3d,
}

impl Shape for Triangle
{
    fn bound(&self) -> AABB
    {
        AABB::union_point(&AABB::new(self.p0, self.p1), self.p2)
    }
    fn intersect(&self, ray: &base::ray::Ray) -> Interaction
    {
        // Both ray and tri is in world coordinate
        // AABB check
        let aabb = self.bound();
        if !aabb.hit(&ray)
        {
            return Interaction::miss();
        }
        // Möller–Trumbore intersection algorithm
        let epsilon = 1e-8;
        let e1 = self.p1 - self.p0;
        let e2 = self.p2 - self.p0;
        let t = ray.o - self.p0;
        let p = Vec3d::cross(ray.d, e2);
        let q = Vec3d::cross(t, e1);
        let a = Vec3d::dot(p, e1);
        if f64::abs(a) < epsilon
        {
            return Interaction::miss();
        }
        let f = 1. / a;
        let u = f * Vec3d::dot(p, t);
        let v = f * Vec3d::dot(q, ray.d);
        if u < 0. || v < 0. || u+v > 1.
        {
            return Interaction::miss();
        }
        let t_hit = f * Vec3d::dot(q, e2);
        if t_hit < epsilon
        {
            return Interaction::miss();
        }
        let mut n_hit = Vec3d::cross(e1, e2).norm();
        if Vec3d::dot(n_hit, ray.d) > 0.
        {
            n_hit = -n_hit;
        }     
        Interaction{ hit: true, t_hit, n_hit } 
    }
}
