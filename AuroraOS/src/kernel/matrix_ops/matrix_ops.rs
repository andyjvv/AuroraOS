use ndarray::{Array, Array2};
use std::ops::{Add, Mul};
// Implementación de operaciones matriciales
pub struct MatrixOps;
impl MatrixOps {
    // Multiplicación de matrices
    pub fn matmul(a: &Array2<f64>, b: &Array2<f64>) -> Array2<f64> {
        a.dot(b)
    }
    // Suma de matrices
    pub fn add(a: &Array2<f64>, b: &Array2<f64>) -> Array2<f64> {
        a + b
    }
    // Transpuesta de matriz
    pub fn transpose(a: &Array2<f64>) -> Array2<f64> {
        a.t()
    }
    // Inversa de matriz
    pub fn inverse(a: &Array2<f64>) -> Array2<f64> {
        a.inv().unwrap()
    }
}