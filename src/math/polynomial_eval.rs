#![allow(dead_code)]


pub fn horner(coefficients: &[f64], x: f64) -> f64 {
    let mut result = coefficients[0];  // The first coefficient is the highest power
    coefficients.iter().skip(1).for_each(|&coefficient| result = result * x + coefficient);
    result
}

fn bernstein(n: usize, i: usize, t: f64) -> f64 {
    let mut coeff = 1.0;
    let mut j = i;
    let mut k = n - i;

    while j > 0 {
        coeff *= t;
        j -= 1;
    }

    while k > 0 {
        coeff *= 1.0 - t;
        k -= 1;
    }

    coeff * binomial_coeff(n, i) as f64
}

fn binomial_coeff(n: usize, k: usize) -> usize {
    let mut res = 1;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }
    res
}

// Casteljau algorithm for evaluating Bézier curves
fn casteljau(points: &[Point], t: f64) -> Point {
    let mut temp_points = points.to_vec();
    for _ in 1..points.len() {
        for i in 0..(temp_points.len() - 1) {
            temp_points[i] = Point::lerp(&temp_points[i], &temp_points[i + 1], t);
        }
        temp_points.pop();
    }
    temp_points[0]
}

// Point struct
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn lerp(p1: &Point, p2: &Point, t: f64) -> Point {
        Point {
            x: p1.x + (p2.x - p1.x) * t,
            y: p1.y + (p2.y - p1.y) * t,
        }
    }
}




pub fn eval_polynomial(method: fn(&[f64], f64) -> f64, coefficients: &[f64], min: f64, max: f64, step: f64) {
    (0..((max - min) / step) as u32 + 1).map(|i| min + i as f64 * step)
        .for_each(|x| println!("f({x:.2}) = {:.4}", method(&coefficients, x))
    );
}
