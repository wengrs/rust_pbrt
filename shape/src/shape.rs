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
    pub p_hit: Vec3d,
    pub t_hit: f64,
    pub n_hit: Vec3d,
}

impl Interaction
{
    pub fn miss() -> Interaction
    {
        Interaction{ hit:false, p_hit:Vec3d::zero(), t_hit:0., n_hit:Vec3d::zero() }
    }
}