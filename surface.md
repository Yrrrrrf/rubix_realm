# Superficies en Gráficos por Computadora

![surface](./assets/surface_example.png)

# [Algoritmos](./src/math/surface_eval.rs)

Las superficies en gráficos por computadora se utilizan para modelar y representar formas tridimensionales suaves y continuas. Se emplean en aplicaciones como diseño asistido por computadora (CAD), animación y visualización científica.

## Superficies Bézier

Las superficies Bézier se definen mediante un conjunto de puntos de control y utilizan una combinación de funciones de interpolación para crear superficies suaves.

```rust
// Estructura para representar un punto en el espacio 3D
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    // Método para la interpolación lineal entre dos puntos
    fn lerp(p1: &Point, p2: &Point, t: f64) -> Point {
        Point {
            x: p1.x + (p2.x - p1.x) * t,
            y: p1.y + (p2.y - p1.y) * t,
            z: p1.z + (p2.z - p1.z) * t,
        }
    }
}

// Función para evaluar una superficie Bézier
fn bezier_surface(control_points: &Vec<Vec<Point>>, u: f64, v: f64) -> Point {
    let mut temp_points = control_points.clone();
    let n = temp_points.len();
    let m = temp_points[0].len();

    // Interpolación en la dirección u
    for k in 0..n {
        for i in 0..(n - 1) {
            for j in 0..m {
                temp_points[i][j] = Point::lerp(&temp_points[i][j], &temp_points[i + 1][j], u);
            }
        }
        temp_points.pop();
    }

    // Interpolación en la dirección v
    for k in 0..m {
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
```

### NURBS (Non-Uniform Rational B-Splines)

Las superficies NURBS son una **generalización de las superficies de Bézier**. Son ampliamente utilizadas en aplicaciones de modelado y animación 3D debido a su flexibilidad y precisión.

```rust
// Función para evaluar una superficie NURBS
fn nurbs_surface(control_points: &Vec<Vec<Point>>, weights: &Vec<Vec<f64>>, u: f64, v: f64) -> Point {
    let n = control_points.len() - 1;
    let m = control_points[0].len() - 1;

    let mut numerator = Point { x: 0.0, y: 0.0, z: 0.0 };
    let mut denominator = 0.0;

    // Evaluación de la superficie utilizando las funciones de base de Bernstein
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

    // División para obtener la coordenada final de la superficie NURBS
    Point {
        x: numerator.x / denominator,
        y: numerator.y / denominator,
        z: numerator.z / denominator,
    }
}

// Función de Bernstein para la interpolación
fn bernstein(n: usize, i: usize, t: f64) -> f64 {
    let binom = binomial_coeff(n, i);
    binom as f64 * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
}

// Función para calcular el coeficiente binomial
fn binomial_coeff(n: usize, k: usize) -> usize {
    let mut res = 1;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }
    res
}
```
