use std::ops;
use std::cmp;
use crate::vector::Vec3d;

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
}