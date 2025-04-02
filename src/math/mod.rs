mod matrix;
mod point;

pub mod surface;

// reimport the matrix module to avoid the need to use the full path
pub use matrix::Matrix;
pub use point::Point;
pub mod bezier;
pub mod polynomial_eval;

// todo: add a "Numerical Analysis" module
// * add the methods for:
//   - one variable functions (closed methods, open methods)
//   - iterative methods
//   - direct methods
//   - non-linear equations (systems of equations)
//   - interpolation
//   - approximation
//   - numerical integration & differentiation
