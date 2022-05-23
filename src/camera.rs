#[allow(dead_code)]
use crate::linalg::coord::Coord3;
use crate::linalg::matrix::Matrix4;

/// Viewport transformation matrix
pub fn viewport(nx: f32, ny: f32) -> Matrix4<f32> {
    Matrix4([
        [nx / 2., 0., 0., (nx - 1.) / 2.],
        [0., ny / 2., 0., (ny - 1.) / 2.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ])
}

/// Orthographic projection matrix
pub fn orth(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Matrix4<f32> {
    Matrix4::<f32>([
        [2. / (r - l), 0., 0., -(r + l) / (r - l)],
        [0., 2. / (t - b), 0., -(t + b) / (t - b)],
        [0., 0., 2. / (n - f), -(n + f) / (n - f)],
        [0., 0., 0., 1.],
    ])
}

/// Perspective matrix
pub fn persp(n: f32, f: f32) -> Matrix4<f32> {
    Matrix4::<f32>([
        [n, 0., 0., 0.],
        [0., n, 0., 0.],
        [0., 0., n + f, -f * n],
        [0., 0., 1., 0.],
    ])
}

pub fn persp_proj(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Matrix4<f32> {
    Matrix4::<f32>([
        [2. * n / (r - l), 0., (l + r) / (l - r), 0.],
        [0., 2. * n / (t - b), (b + t) / (b - t), 0.],
        [0., 0., (n + f) / (n - f), 2. * f * n / (f - n)],
        [0., 0., 1., 0.],
    ])
}

/// Camera transformation matrix
/// eye: The eye coordinate
/// g: gaze direction
/// up: view-up direction
pub fn lookat(eye: Coord3<f32>, g: Coord3<f32>, up: Coord3<f32>) -> Matrix4<f32> {
    let w = g.normalized() * -1.;
    let u = up.cross(w);
    let u = u.normalized();
    let v = w.cross(u);
    Matrix4([
        [u.x, u.y, u.z, 0.],
        [v.x, v.y, v.z, 0.],
        [w.x, w.y, w.z, 0.],
        [0., 0., 0., 1.],
    ])
    .dotm(&Matrix4([
        [1., 0., 0., -eye.x],
        [0., 1., 0., -eye.y],
        [0., 0., 1., -eye.z],
        [0., 0., 0., 1.],
    ]))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persp_test() {
        let c = 20.;
        assert_eq!(
            orth(-3., 1., -1., 10., 1., c).dotm(&persp(1., c)),
            persp_proj(-3., 1., -1., 10., 1., c)
        );
    }
}
