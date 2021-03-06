use std::ops;
use std::cmp;
use crate::vector::Vec3d;
use crate::ray::Ray;

#[derive(Clone, Debug)]
pub struct Matrix4
{
    pub mat:[[f64; 4]; 4],
}

impl Matrix4
{
    pub fn i() -> Matrix4
    {
        let mut mat = [[0.; 4]; 4];
        for i in 0..4
        {
            mat[i][i] = 1.;
        }
        Matrix4::new_and_move(mat)
    }
    pub fn new(m: &[[f64; 4]; 4]) -> Matrix4
    {
        Matrix4 {mat: m.clone()}
    }
    pub fn new_and_move(m: [[f64; 4]; 4]) -> Matrix4
    {
        Matrix4 {mat: m}
    }
    pub fn mul(m1: &Matrix4, m2: &Matrix4) -> Matrix4
    {
        let mut mat = [[0.; 4]; 4];
        for i in 0..4
        {
            for j in 0..4
            {
                mat[i][j] = m1[(i,0)]*m2[(0,j)] + m1[(i,1)]*m2[(1,j)] + m1[(i,2)]*m2[(2,j)] + m1[(i,3)]*m2[(3,j)];
            }
        }
        Matrix4::new_and_move(mat)
    }
    pub fn inv(&self) -> Matrix4
    {
        let mut v = [[0.; 4]; 4];
        for i in 0..4
        {
            v[i][i] = 1.;
        }
        let mut m = self.mat.clone();
        let mut k = 0;
        while k < 4
        {
            let mut l = k;
            while m[k][l] == 0.
            {
                l+=1;
                if l >= 4
                {
                    panic!("Singular matrix!");
                }
            }
            // Interchange the row k and l
            if l != k 
            {
                let m_tmp = m[k].clone();
                let v_tmp = v[k].clone();
                m[k] = m[l];
                v[k] = v[l];
                m[l] = m_tmp;
                v[l] = v_tmp;
            }
            // The row k divide the pivot
            let pivot = m[k][k];
            for i in 0..4
            {
                m[k][i] /= pivot;
                v[k][i] /= pivot;
            }
            // Eliminate element at col k
            for i in 0..4
            {
                if i == k
                {
                    continue;
                }
                let time = m[i][k];
                for j in 0..4
                {
                    m[i][j] -= time*m[k][j];
                    v[i][j] -= time*v[k][j];
                }
            }
            k += 1;
        }
        Matrix4::new_and_move(v)
    }
}

impl ops::Index<(usize, usize)> for Matrix4
{
    type Output = f64;
    fn index(&self, (a, b): (usize, usize)) -> &f64
    {
        &self.mat[a][b]
    }
}

impl cmp::PartialEq for Matrix4
{
    fn eq(&self, other: &Matrix4) -> bool
    {
        for i in 0..4
        {
            for j in 0..4
            {
                if f64::abs(self[(i,j)]-other[(i,j)]) > 1e-4
                {
                    return false;
                }
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
pub struct Transform
{
    pub m: Matrix4,
    pub m_inv: Matrix4,
}

impl Transform
{
    pub fn new(m: &Matrix4) -> Transform
    {
        Transform{ m:m.clone(), m_inv:m.inv() }
    }
    pub fn inv(&self) -> Transform
    {
        Transform{ m: self.m_inv.clone(), m_inv:self.m.clone() }
    }
    pub fn translate(delta: Vec3d) -> Transform
    {
        let mat = [ [1., 0., 0., delta.x], 
                    [0., 1., 0., delta.y], 
                    [0., 0., 1., delta.z], 
                    [0., 0., 0., 1.]];
        Transform::new(&(Matrix4::new_and_move(mat)))
    }
    pub fn scale(x: f64, y: f64, z: f64) -> Transform
    {
        let mat = [ [x,     0.,     0.,     0.], 
                    [0.,    y,      0.,     0.], 
                    [0.,    0.,     z,      0.], 
                    [0.,    0.,     0.,     1.]];
        Transform::new(&(Matrix4::new_and_move(mat)))       
    }
    pub fn rotate_x(t: f64) -> Transform
    {
        let mat = [ [1.,    0.,     0.,         0.], 
                    [0.,    t.cos(),-t.sin(),   0.], 
                    [0.,    t.sin(), t.cos(),   0.], 
                    [0.,    0.,     0.,         1.]];
        Transform::new(&(Matrix4::new_and_move(mat)))       
    }
    pub fn rotate_y(t: f64) -> Transform
    {
        let mat = [ [ t.cos(),  0.,     t.sin(),    0.], 
                    [0.,        1.,     0.,         0.], 
                    [-t.sin(),  0.,     t.cos(),    0.], 
                    [0.,        0.,     0.,         1.]];
        Transform::new(&(Matrix4::new_and_move(mat)))       
    }
    pub fn rotate_z(t: f64) -> Transform
    {
        let mat = [ [t.cos(),   -t.sin(),   0.,     0.], 
                    [t.sin(),    t.cos(),   0.,     0.], 
                    [0.,        0.,         1.,     0.], 
                    [0.,        0.,         0.,     1.]];
        Transform::new(&(Matrix4::new_and_move(mat)))       
    }
    pub fn rotate(t: f64, axis: Vec3d) -> Transform
    {
        let (c, s) = (t.cos(), t.sin());
        let (ux, uy, uz) = (axis.x, axis.y, axis.z);
        let mut mat = [[0.; 4]; 4];
        mat[0][0] = c+ux*ux*(1.-c);
        mat[0][1] = ux*uy*(1.-c)-uz*s;
        mat[0][2] = ux*uz*(1.-c)+uy*s;
        mat[1][0] = uy*ux*(1.-c)+uz*s;
        mat[1][1] = c+uy*uy*(1.-c);
        mat[1][2] = uy*uz*(1.-c)-ux*s;
        mat[2][0] = uz*ux*(1.-c)-uy*s;
        mat[2][1] = uz*uy*(1.-c)+ux*s;
        mat[2][2] = c+uz*uz*(1.-c);
        mat[3][3] = 1.;
        Transform::new(&(Matrix4::new_and_move(mat)))   
    }
    pub fn look_at(pos: Vec3d, look: Vec3d, up: Vec3d) -> Transform
    {
        let mut c_to_w = [[0.; 4]; 4];
        c_to_w[0][3] = pos.x;
        c_to_w[1][3] = pos.y;
        c_to_w[2][3] = pos.z;
        c_to_w[3][3] = 1.;
        let dir = (look - pos).norm();
        let left = Vec3d::cross(up.norm(), dir).norm();
        let new_up = Vec3d::cross(dir, left);
        c_to_w[0][0] = left.x;
        c_to_w[1][0] = left.y;
        c_to_w[2][0] = left.z;
        c_to_w[0][1] = new_up.x;
        c_to_w[1][1] = new_up.y;
        c_to_w[2][1] = new_up.z;
        c_to_w[0][2] = dir.x;
        c_to_w[1][2] = dir.y;
        c_to_w[2][2] = dir.z; 
        let c_to_w = Matrix4::new_and_move(c_to_w);     
        let w_to_c = c_to_w.inv();
        Transform{ m: w_to_c, m_inv: c_to_w}  
    }
    pub fn mul(t1: &Transform, t2: &Transform) -> Transform
    {
        Transform{ m:Matrix4::mul(&t1.m, &t2.m), m_inv:Matrix4::mul(&t2.m_inv, &t1.m_inv) }
    }
    pub fn swap_handedness(&self) -> bool
    {
        let det =   self.m[(0,0)] * (self.m[(1,1)]*self.m[(2,2)] - self.m[(1,2)]*self.m[(2,1)]) - 
                    self.m[(0,1)] * (self.m[(1,0)]*self.m[(2,2)] - self.m[(1,2)]*self.m[(2,0)]) + 
                    self.m[(0,2)] * (self.m[(1,0)]*self.m[(2,1)] - self.m[(1,1)]*self.m[(2,0)]);
        det < 0.
    }
    pub fn act_point(&self, p: Vec3d) -> Vec3d
    {
        let w = p.x*self.m[(3,0)] + p.y*self.m[(3,1)] + p.z*self.m[(3,2)] + self.m[(3,3)];
        let x = (p.x*self.m[(0,0)] + p.y*self.m[(0,1)] + p.z*self.m[(0,2)] + self.m[(0,3)]) / w;
        let y = (p.x*self.m[(1,0)] + p.y*self.m[(1,1)] + p.z*self.m[(1,2)] + self.m[(1,3)]) / w;
        let z = (p.x*self.m[(2,0)] + p.y*self.m[(2,1)] + p.z*self.m[(2,2)] + self.m[(2,3)]) / w;
        Vec3d::new(x, y, z)
    }
    pub fn act_vector(&self, p: Vec3d) -> Vec3d
    {
        let x = p.x*self.m[(0,0)] + p.y*self.m[(0,1)] + p.z*self.m[(0,2)];
        let y = p.x*self.m[(1,0)] + p.y*self.m[(1,1)] + p.z*self.m[(1,2)];
        let z = p.x*self.m[(2,0)] + p.y*self.m[(2,1)] + p.z*self.m[(2,2)];
        Vec3d::new(x, y, z)        
    }
    pub fn act_normal(&self, p: Vec3d) -> Vec3d
    {
        let x = p.x*self.m_inv[(0,0)] + p.y*self.m_inv[(0,1)] + p.z*self.m_inv[(0,2)];
        let y = p.x*self.m_inv[(1,0)] + p.y*self.m_inv[(1,1)] + p.z*self.m_inv[(1,2)];
        let z = p.x*self.m_inv[(2,0)] + p.y*self.m_inv[(2,1)] + p.z*self.m_inv[(2,2)];
        Vec3d::new(x, y, z)         
    }
    pub fn act_ray(&self, r: &Ray) -> Ray
    {
        Ray{ o: self.act_point(r.o), d:self.act_vector(r.d), ..*r }
    }
}