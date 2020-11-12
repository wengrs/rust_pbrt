pub mod vector;
pub mod ray;
pub mod bounding;
pub mod transformation;
pub mod solver;

#[cfg(test)]
mod aabb_tests {
    use crate::vector::Vec3d;
    use crate::ray::Ray;
    use crate::bounding::AABB;
    #[test]
    fn hit_test_0() {
        let aabb = AABB::new(Vec3d::new(1., 1., 1.), Vec3d::new(-1., -1., -1.));
        let o = Vec3d::new(2., 0., 0.);
        let r1 = Ray::new(o, Vec3d::new(-1., 0., 0.));
        let r2 = Ray::new(o, Vec3d::new(1., 0., 0.));
        let r3 = Ray::new(o, Vec3d::new(-1., 0.5, 0.5));
        let r4 = Ray::new(o, Vec3d::new(-1., 1.5, 1.5));
        assert_eq!(aabb.hit(&r1), true);
        assert_eq!(aabb.hit(&r2), false);
        assert_eq!(aabb.hit(&r3), true);
        assert_eq!(aabb.hit(&r4), false);
        let r5 = Ray{ o:o, d:Vec3d::new(-1., 0., 0.), t:0., tmax:0.5 };
        assert_eq!(aabb.hit(&r5), false);
    }
}
#[cfg(test)]
mod matrix_tests{
    use crate::transformation::Matrix4;
    use rand::distributions::Uniform;
    use rand::distributions::Distribution;
    #[test]
    fn mul_test_0(){
        let m1 = Matrix4::new(&[[3., 5., 2., 1.], 
                                [6., 1., 8., 4.], 
                                [7., 3., 5., 1.], 
                                [7., 9., 9., 9.]]);
        let m2 = Matrix4::new(&[[1., 1., 4., 2.],
                                [7., 4., 9., 2.],
                                [8., 3., 2., 8.],
                                [8., 8., 3., 6.]]);
        let m3 = Matrix4::new(&[[62.,   37.,    64.,    38.], 
                                [109.,  66.,    61.,    102.], 
                                [76.,   42.,    68.,    66.], 
                                [214.,  142.,   154.,   158.]]);                  
        assert_eq!(Matrix4::mul(&m1, &m2), m3);
    }
    #[test]
    fn inv_test_0(){
        let mut rng = rand::thread_rng();
        let dice = Uniform::from(0. ..100.);
        let i = Matrix4::i();
        for _ in 0..1000
        {
            let mut v = [[0.; 4]; 4];
            for i in 0..4
            {
                for j in 0..4
                {
                    v[i][j] = dice.sample(&mut rng);
                }
            }
            let m = Matrix4::new(&v);
            let m_inv = m.inv();
            assert_eq!(Matrix4::mul(&m, &m_inv), i);
        }
    }
}