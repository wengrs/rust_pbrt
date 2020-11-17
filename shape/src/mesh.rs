use base::transformation::Transform;
use base::bounding::AABB;
use base::vector::Vec3d;
use base::ray::Ray;
use crate::shape::Shape;
use crate::shape::Interaction;

pub struct Mesh
{
    pub obj_to_world: Vec<Transform>,
    pub world_to_obj: Vec<Transform>,
    pub bouding_box: AABB,
    pub vertex_indices: Vec<usize>,
    pub vertices: Vec<Vec3d>,
}

impl Shape for Mesh
{
    fn bound(&self) -> AABB
    {
        self.bouding_box.clone()
    }
    fn intersect(&self, ray: &Ray) -> Interaction
    {
        // AABB check
        let mut ray_obj = ray.clone();
        for t in &self.world_to_obj
        {
            ray_obj = t.act_ray(&ray_obj);
        }
        let aabb = self.bound();
        if !aabb.hit(&ray_obj)
        {
            return Interaction::miss();
        }        
        // Transform vertices to world coordinate
        let mut v_world: Vec<Vec3d> = Vec::with_capacity(self.vertices.len());
        let mut i = 0;
        for v in &self.vertices
        {
            let mut vt = v.clone();
            for t in &self.obj_to_world
            {
                vt = t.act_point(vt);
            }
            v_world[i] = vt;
            i += 1;
        }
        let num_tri = self.vertex_indices.len() / 3;
        let mut t_hit = f64::INFINITY;
        let mut n_hit = Vec3d::zero();
        for i in 0..num_tri
        {
            let p0 = v_world[self.vertex_indices[3*i]];
            let p1 = v_world[self.vertex_indices[3*i+1]];
            let p2 = v_world[self.vertex_indices[3*i+2]];
            let tri = Triangle{ p0, p1, p2 };
            let inter = tri.intersect(ray);
            if inter.hit && inter.t_hit < t_hit
            {
                t_hit = inter.t_hit;
                n_hit = inter.n_hit;
            }
        }
        if t_hit == f64::INFINITY
        {
            return Interaction::miss();
        }
        else
        {
            return Interaction{ hit: true, t_hit, n_hit };
        }
    }
}

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
    fn intersect(&self, ray: &Ray) -> Interaction
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
