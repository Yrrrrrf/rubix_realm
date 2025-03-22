#![allow(unused)]

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn lerp(p1: &Point, p2: &Point, t: f64) -> Point {
        Point {
            x: p1.x + (p2.x - p1.x) * t,
            y: p1.y + (p2.y - p1.y) * t,
            z: p1.z + (p2.z - p1.z) * t,
        }
    }
}

pub fn bezier_surface(control_points: &Vec<Vec<Point>>, u: f64, v: f64) -> Point {
    let mut temp_points = control_points.clone();
    let n = temp_points.len();
    let m = temp_points[0].len();

    for _ in 0..n {
        for i in 0..(n - 1) {
            for j in 0..m {
                temp_points[i][j] = Point::lerp(&temp_points[i][j], &temp_points[i + 1][j], u);
            }
        }
        temp_points.pop();
    }

    for _ in 0..m {
        for i in 0..(n - 1) {
            for j in 0..(m - 1) {
                temp_points[i][j] = Point::lerp(&temp_points[i][j], &temp_points[i][j + 1], v);
            }
        }
        for row in temp_points.iter_mut() {
            row.pop();
        }
    }

    temp_points[0][0]
}

pub fn nurbs_surface(control_points: &Vec<Vec<Point>>, weights: &Vec<Vec<f64>>, u: f64, v: f64) -> Point {
    let n = control_points.len() - 1;
    let m = control_points[0].len() - 1;

    let mut numerator = Point { x: 0.0, y: 0.0, z: 0.0 };
    let mut denominator = 0.0;

    for i in 0..=n {
        for j in 0..=m {
            let basis_u = bernstein(n, i, u);
            let basis_v = bernstein(m, j, v);
            let weight = weights[i][j];

            numerator.x += basis_u * basis_v * weight * control_points[i][j].x;
            numerator.y += basis_u * basis_v * weight * control_points[i][j].y;
            numerator.z += basis_u * basis_v * weight * control_points[i][j].z;

            denominator += basis_u * basis_v * weight;
        }
    }

    Point {
        x: numerator.x / denominator,
        y: numerator.y / denominator,
        z: numerator.z / denominator,
    }
}

fn bernstein(n: usize, i: usize, t: f64) -> f64 {
    let binom = binomial_coeff(n, i);
    binom as f64 * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
}

fn binomial_coeff(n: usize, k: usize) -> usize {
    let mut res = 1;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }
    res
}
