use crate::vector::Vec3d;

#[derive(Clone,Debug)]
pub struct Ray
{
    pub o: Vec3d,
    pub d: Vec3d,
    pub t: f64,
    pub tmax: f64,
}

impl Ray
{
    pub fn new(o: Vec3d, d: Vec3d) -> Ray
    {
        Ray{ o, d, t: 0., tmax: f64::INFINITY }
    }
}