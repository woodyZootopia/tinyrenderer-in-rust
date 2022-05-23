pub mod coord;
pub mod matrix;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::coord::*;
    use super::matrix::*;

    #[test]
    fn simple_arith_test() {
        let P3 = Matrix3::<f32>([[3., 7., -3.], [9., 8., 30.], [-3., -4., -10.]]);
        let v3 = Coord3::<f32> {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        assert_eq!(
            P3.dotv(&v3),
            Coord3::<f32> {
                x: 8.,
                y: 115.,
                z: -41.
            }
        );
        let P4 = Matrix4::<f32>([
            [1., 5., 9., 13.],
            [2., 6., 0., -14.],
            [0., 0., 0., 0.],
            [0., 0., 1., 1.],
        ]);
        let v4 = v3.homogenize();
        assert_eq!(
            P4.dotv(&v4),
            Coord4 {
                x: 51.,
                y: 0.,
                z: 0.,
                w: 4.
            }
        );
        assert_eq!(
            P4.dotm(&P4),
            Matrix4::<f32>([
                [11., 35., 22., -44.],
                [14., 46., 4., -72.],
                [0., 0., 0., 0.],
                [0., 0., 1., 1.],
            ])
        )
    }
}
