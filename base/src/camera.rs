use crate::ray::Ray;
use crate::transformation::Transform;
use crate::vector::Vec3d;
pub trait Camera
{
    fn generate_ray(self, pf: (f64, f64), pl: (f64, f64)) -> Ray;
}

pub struct PerspectiveCamera
{
    pub camera_to_world: Vec<Transform>,
    pub fov_x: f64,
    pub fov_y: f64,
}

impl Camera for PerspectiveCamera
{
    fn generate_ray(self, pf: (f64, f64), _: (f64, f64)) -> Ray
    {
        let x = (self.fov_x/2.).tan()*pf.0;
        let y = (self.fov_y/2.).tan()*pf.1;
        let d = Vec3d{x, y, z:1.};
        let mut r = Ray::new(Vec3d::zero(), d);
        for t in &self.camera_to_world
        {
            r = t.act_ray(&r);
        }
        r.d = r.d.norm();
        r
    }
}

pub struct OrthographicCamera
{
    pub camera_to_world: Vec<Transform>,
    pub wx: f64,
    pub wy: f64,
}

impl Camera for OrthographicCamera
{
    fn generate_ray(self, pf: (f64, f64), _: (f64, f64)) -> Ray
    {
        let x = self.wx*pf.0;
        let y = self.wy*pf.1;
        let d = Vec3d{x:0., y:0., z:1.};
        let o = Vec3d{x, y, z:0.};
        let mut r = Ray::new(o, d);
        for t in &self.camera_to_world
        {
            r = t.act_ray(&r);
        }
        r
    }
}