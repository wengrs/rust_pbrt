use std::ops;
use std::cmp;

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
        Matrix4::new(mat)
    }
    pub fn new(m: [[f64; 4]; 4]) -> Matrix4
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
        Matrix4::new(mat)
    }
    pub fn inv(matrix: &Matrix4) -> Matrix4
    {
        let mut v = [[0.; 4]; 4];
        for i in 0..4
        {
            v[i][i] = 1.;
        }
        let mut m = matrix.mat.clone();
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
        Matrix4::new(v)
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