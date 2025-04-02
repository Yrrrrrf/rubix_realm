//
#![allow(unused)]

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    data: Vec<f64>,
}

impl Point {
    pub fn new(data: Vec<f64>) -> Self {
        Point { data }
    }

    pub fn zero(size: usize) -> Self {
        Point::new(vec![0.0; size])
    }

    pub fn distance(&self, other: &Point) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn magnitude(&self) -> f64 {
        self.data.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    // pub fn magnitude_squared(&self) -> f64 {
    //     self.data.iter().map(|x| x.powi(2)).sum()
    // }

    pub fn normalize(&self) -> Point {
        let mag = self.magnitude();
        Point::new(self.data.iter().map(|x| x / mag).collect())
    }

    pub fn scale(&self, scalar: f64) -> Point {
        Point::new(self.data.iter().map(|x| x * scalar).collect())
    }

    pub fn angle(&self, other: &Point) -> f64 {
        self.dot(other) / (self.magnitude() * other.magnitude()).acos()
    }

    // pub fn project(&self, other: &Point) -> Point {
    //     other.scale(self.dot(other) / other.magnitude_squared())
    // }

    // pub fn cross(&self, other: &Point) -> Point {}

    // pub fn reflect(&self, normal: &Point) -> Point {}

    // pub fn lerp(&self, other: &Point, t: f64) -> Point {
    //     self.scale(1.0 - t).add(&other.scale(t))
    // }

    // pub fn slerp(&self, other: &Point, t: f64) -> Point {
    //     let dot = self.dot(other).min(1.0).max(-1.0);
    //     let theta = dot.acos() * t;
    //     let relative = other.subtract(self.scale(dot)).normalize();
    //     self.scale(theta.cos()).add(&relative.scale(theta.sin()))
    // }

    // pub fn transform(&self, matrix: &crate::math::matrix::Matrix) -> Point {
    //     let mut result = vec![0.0; matrix.data.len()];
    //     for (i, row) in matrix.data.iter().enumerate() {
    //         for (j, col) in row.iter().enumerate() {
    //             result[i] += col * self.data[j];
    //         }
    //     }
    //     Point::new(result)
    // }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = self.clone() + other;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a - b)
                .collect(),
        )
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Point) {
        *self = self.clone() - other;
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point::new(self.data.iter().map(|x| -x).collect())
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, scalar: f64) -> Point {
        Point::new(self.data.iter().map(|x| x * scalar).collect())
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        point * self
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, scalar: f64) {
        *self = self.clone() * scalar;
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, scalar: f64) -> Point {
        Point::new(self.data.iter().map(|x| x / scalar).collect())
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, scalar: f64) {
        *self = self.clone() / scalar;
    }
}

impl Mul<Point> for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point::new(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a * b)
                .collect(),
        )
    }
}

impl MulAssign<Point> for Point {
    fn mul_assign(&mut self, other: Point) {
        *self = self.clone() * other;
    }
}

impl Div<Point> for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point::new(
            self.data
                .iter()
                .zip(other.data.iter())
                .map(|(a, b)| a / b)
                .collect(),
        )
    }
}

impl DivAssign<Point> for Point {
    fn div_assign(&mut self, other: Point) {
        *self = self.clone() / other;
    }
}
