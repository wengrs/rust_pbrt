use base::transformation::Transform;
use base::bounding::AABB;
use crate::shape::Shape;

pub struct Sphere
{
    pub world_to_obj: Vec<Transform>,
    pub r: f64,
    pub z_min: f64,
    pub z_max: f64,
    pub t_min: f64,
    pub t_max: f64,
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
}