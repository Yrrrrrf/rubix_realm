//! This file defines a Matrix struct and its associated methods.
//! 
//! The Matrix struct is a simple 2D array of f64 values.
//! 

//! Matrix Struct Enhanced with Arithmetic Operations
#![allow(unused)]

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use std::fmt;


#[derive(Debug, Clone, PartialEq)] 
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Matrix { data }
    }

    pub fn identity(size: usize) -> Self {
        Matrix::new((0..size).map(|i| (0..size).map(|j| if i == j {1.0} else {0.0}).collect()).collect())
    }

    pub fn transpose(&self) -> Self {
        Matrix::new((0..self.data[0].len()).map(|i| self.data.iter().map(|row| row[i]).collect()).collect())
    }

    // todo: Test if the determinant fn works correctly!
    // todo: Check if the determinant is zero before computing the inverse
    // todo: Implement the LU decomposition, Gauss-Jordan elimination, Cholesky decomposition and QR decomposition
    // todo: Implement the Eigenvalue decomposition and Singular Value decomposition
    pub fn determinant(&self) -> f64 {
            match self.data.len() {
                1 => self.data[0][0],
                2 => self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0],
                n => {  // Recursive case for nxn matrix
                    let mut det = 0.0;
                    for j in 0..n {
                        let mut sub_matrix = self.clone();
                        sub_matrix.data.remove(0);
                        sub_matrix.data.iter_mut().for_each(|row| {row.remove(j);});
                        det += self.data[0][j] * (-1.0f64).powi(j as i32) * sub_matrix.determinant();
                    }
                    det
                }
            }
        }

    fn sub_matrix(&self, skip_row: usize, skip_col: usize) -> Matrix {
        let sub_data = self.data.iter().enumerate().filter_map(|(i, row)| 
            if i != skip_row {
                Some(row.iter().enumerate().filter_map(|(j, &val)| {
                    if j != skip_col { Some(val) } else { None }
                }).collect())
            } else {
                None
            }
        ).collect();
        Matrix { data: sub_data }
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
        sign * self.sub_matrix(row, col).determinant()
    }

    // todo: Test if the inverse fn works correctly!
    pub fn inverse(&self) -> Self {
        let det = self.determinant();
        assert!(det != 0.0, "Matrix is singular and does not have an inverse.");

        let n = self.data.len();
        // Use the cofactor helper function to compute each adjugate element
        // Transpose the cofactor matrix by swapping j and i
        let adjugate_data = (0..n).map(|i| (0..n).map(|j| self.cofactor(j, i)).collect::<Vec<_>>()).collect::<Vec<_>>();
        Matrix::new(adjugate_data.iter().map(|row| row.iter().map(|&item| item / det).collect::<Vec<_>>()).collect())
    }

    pub fn is_square(&self) -> bool {
        self.data.len() == self.data[0].len()
    }

    pub fn is_symmetric(&self) -> bool {
        self.is_square() && *self == self.transpose()
    }

    pub fn is_skew_symmetric(&self) -> bool {
        self.is_square() && *self == -self.transpose()
    }

    pub fn is_diagonal(&self) -> bool {
        self.data.iter().enumerate().all(|(i, row)| row.iter().enumerate().all(|(j, &val)| (i == j) || (val == 0.0)))
    }

    // todo: test this method
    pub fn is_upper_triangular(&self) -> bool {
        self.data.iter().enumerate().all(|(i, row)| row.iter().enumerate().all(|(j, &val)| (i <= j) || (val == 0.0)))
    }

    // todo: test this method
    pub fn is_lower_triangular(&self) -> bool {
        self.data.iter().enumerate().all(|(i, row)| row.iter().enumerate().all(|(j, &val)| (i >= j) || (val == 0.0)))
    }

    // todo: implement these methods
    // pub fn is_orthogonal(&self) -> bool {}  // A matrix is orthogonal if its transpose is equal to its inverse
    // pub fn is_normal(&self) -> bool {}  // A matrix is normal if it commutes with its conjugate transpose
    // pub fn is_singular(&self) -> bool {}  // A matrix is singular if its determinant is zero
    // pub fn is_invertible(&self) -> bool {}  // A matrix is invertible if its determinant is non-zero
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for (i, col) in row.iter().enumerate() {
                if i > 0 { write!(f, " ")?; } // Add a space between numbers, but not before the first number in a row
                write!(f, "{:4.4}", col)?;
                // write!(f, "{:4}", col)?;
            }
            writeln!(f)?; // Add a newline at the end of each row to separate the rows of the matrix
        }
        Ok(())
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, rhs: Matrix) -> Self {
        Matrix::new(self.data.iter().zip(rhs.data.iter())
            .map(|(a, b)| a.iter().zip(b.iter())  // Zip the rows of self and rhs
            .map(|(x, y)| x + y).collect()).collect()  // Add the elements and collect them into a new row
        )
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {*self = self.clone() + rhs;}
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(self, rhs: Matrix) -> Self {
        Matrix::new(self.data.iter().zip(rhs.data.iter())
            .map(|(a, b)| a.iter().zip(b.iter())  // Zip the rows of self and rhs
            .map(|(x, y)| x - y).collect()).collect()  // Subtract the elements and collect them into a new row
        )
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Matrix) {*self = self.clone() - rhs;}
}

impl Neg for Matrix {
    type Output = Self;

    fn neg(self) -> Self {
        Matrix::new(self.data.iter().map(|row| row.iter().map(|&x| -x).collect()).collect())
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self {
        assert_eq!(self.data[0].len(), rhs.data.len(), "Incompatible dimensions for matrix multiplication");
        
        Matrix::new(
            // todo: Use the Volker Strassen method to multiply the matrices. O(n^2.81)
            // todo: Implement the Coppersmith-Winograd algorithm. O(n^2.376)
            // * there's also the AlphaTensor algorithm. But I don't know the complexity of it
            (0..self.data.len()).map(|r| {  // For each row in self
                (0..rhs.data.get(0).expect("rhs matrix is empty").len()).map(|rc| {  // For each column in rhs
                    self.data[r].iter().zip(rhs.data.iter().map(|row| row[rc]))  // Zip the row of self with the column of rhs
                        .map(|(a, b)| a * b).sum()  // Multiply and sum the products
                }).collect()  // Collect the products into a new row
            }).collect()  // Collect the rows into a new matrix
        )
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {*self = self.clone() * rhs;}
}


// // Define the ScalarMult trait
// trait ScalarMult {
//     fn scalar_multiply(&self) -> f64;
// }

// // Define a macro to implement ScalarMult for specified types
// macro_rules! impl_scalar_mult {
//     ($($type:ty),*) => {
//         $(
//             impl ScalarMult for $type {
//                 fn scalar_multiply(&self) -> f64 {
//                     *self as f64
//                 }
//             }
//         )*
//     };
// }

// // Use the macro to implement ScalarMult for f64, usize, and isize
// impl_scalar_mult!(f64, usize, isize);

// impl<T: ScalarMult> Mul<T> for Matrix {
//     type Output = Self;

//     fn mul(self, rhs: T) -> Self {
//         Matrix::new(self.data.iter().map(|row| row.iter().map(|&x| x * x.scalar_multiply()).collect()).collect())
//     }
// }


// impl Maxtrix * f64
impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Matrix {
        Matrix::new(self.data.iter().map(|row| row.iter().map(|&x| x * rhs).collect()).collect())
    }
}

// impl f64 * Matrix
impl Mul<Matrix> for f64 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        rhs * self
    }
}


// impl Matrix / f64
impl Div<f64> for Matrix {
    type Output = Matrix;

    fn div(self, rhs: f64) -> Matrix {
        Matrix::new(self.data.iter().map(|row| row.iter().map(|&x| x / rhs).collect()).collect())
    }
}


// ? ------------------------------ Some ------------------------------ ? //


// create a type of matrix: The Bezier matrix
// * The Bezier matrix is a matrix that is used to calculate the Bezier curve
// the Caracteristic matrix for a Bezier curve of degree n is defined as:
// * B = [B0, B1, B2, ..., Bn] = [P0, P1, P2, ..., Pn] * M
//     where M is the Bezier matrix defined as:
//     * M = [C(n, 0), C(n, 1), C(n, 2), ..., C(n, n)] * [u^0, u^1, u^2, ..., u^n]
//     where C(n, k) is the binomial coefficient
//     * C(n, k) = n! / (k! * (n - k)!)
// the b3 matrix is defined as:
pub fn b3_matrix() -> Matrix {
    Matrix::new(vec![
        vec![-1.0,  3.0, -3.0, 1.0],
        vec![ 3.0, -6.0,  3.0, 0.0],
        vec![-3.0,  3.0,  0.0, 0.0],
        vec![ 1.0,  0.0,  0.0, 0.0]
    ])
}
