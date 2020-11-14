use base::transformation::Transform;
use base::bounding::AABB;
use crate::shape::Shape;
use crate::shape::Interaction;

pub struct Sphere
{
    pub world_to_obj: Vec<Transform>,
    pub r: f64,
    pub z_min: f64,
    pub z_max: f64,
    pub theta_min: f64,
    pub theta_max: f64,
    pub phi_max: f64,
}

impl Shape for Sphere
{
    fn bound(&self) -> AABB
    {
        let p1 = base::vector::Vec3d::new(-self.r, -self.r, self.z_min);
        let p2 = base::vector::Vec3d::new(self.r, self.r, self.z_max);
        AABB::new(p1, p2)
    } 
    fn intersect(&self, ray: &base::ray::Ray) -> Interaction
    {
        // Transform ray to object space
        let mut r = ray.clone();
        for t in &self.world_to_obj
        {
            r = t.act_ray(&r);
        }
        // AABB check
        let aabb = self.bound();
        if !aabb.hit(&ray)
        {
            return Interaction::miss();
        }
        // Compute quadratic sphere coefficients
        let a = r.d.x*r.d.x + r.d.y*r.d.y + r.d.z*r.d.z;
        let b = 2.*(r.d.x*r.o.x + r.d.y*r.o.y + r.d.z*r.o.z);
        let c = r.o.x*r.o.x + r.o.y*r.o.y + r.o.z*r.o.z + self.r*self.r;
        // Solve quadratic equation
        let (solved, t0, t1) = base::solver::quadratic(a, b, c);
        if !solved || t0 > r.tmax || t1 < 0.
        {
            return Interaction::miss();
        }
        let t_hit: f64;
        if t0 < 0.
        {
            t_hit = t1;
        }
        else
        {
            t_hit = t0;
        }
        // Compute sphere hit position
        let p_hit = r.pos(t_hit);
        let mut phi = p_hit.y.atan2(p_hit.x);
        if phi < 0.
        { 
            phi += 2.*std::f64::consts::PI;
        }
        // Test sphere intersection against clipping parameters
        if p_hit.z < self.z_min || p_hit.z > self.z_max || phi > self.phi_max
        {
            return Interaction::miss();
        }
        let n_hit = p_hit.norm();
        Interaction { hit:true, p_hit, t_hit, n_hit}
    }
}