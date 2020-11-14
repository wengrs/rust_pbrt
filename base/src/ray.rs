use crate::vector::Vec3d;
use crate::transformation::Transform;

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
    pub fn apply_trans(r: &Ray, trans: &Vec<Transform>) -> Ray
    {
        let mut o = r.o;
        let mut d = r.d;
        for tran in trans
        {
            o = tran.act_point(o);
            d = tran.act_vector(d);
        }
        Ray { o:o, d:d, t:r.t, tmax:r.tmax}
    }
    pub fn pos(&self, t:f64) -> Vec3d
    {
        self.o + t*self.d
    }
}