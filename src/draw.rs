use super::*;

/// print the given image
pub fn print_as_ppm(values: Vec<Vec<Color>>) {
    print!("P3\n{} {}\n255\n", values[0].len(), values.len());
    for line in values.into_iter().rev() {
        for value in line {
            write_color(&value);
        }
    }
}

pub fn write_color(colors: &Color) {
    #[allow(unused_mut)]
    let [mut r, mut g, mut b] = colors.e;

    // gamma-correct for gamma=2.0
    // r = r.sqrt();
    // g = g.sqrt();
    // b = b.sqrt();

    fn clamp(x: f32, min: f32, max: f32) -> f32 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }

    println!(
        "{} {} {}",
        (256. * clamp(r, 0.0, 0.999)) as i32,
        (256. * clamp(g, 0.0, 0.999)) as i32,
        (256. * clamp(b, 0.0, 0.999)) as i32,
    );
}
