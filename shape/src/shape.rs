use base::bounding::AABB;
use base::vector::Vec3d;
pub trait Shape
{
    fn bound(&self) -> AABB;
    fn intersect(&self, ray:&base::ray::Ray) -> Interaction;
}

#[derive(Clone,Debug)]
pub struct Interaction
{
    pub hit: bool,
    pub hit_point: Vec3d,
    pub t: f64,
}

impl Interaction
{
    pub fn miss() -> Interaction
    {
        Interaction{ hit:false, hit_point:Vec3d::zero(), t:0. }
    }
}