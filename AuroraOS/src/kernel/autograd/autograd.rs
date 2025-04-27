use ndarray::{Array, Array2};
use std::ops::{Add, Mul};
// Implementación de autogradiente
pub struct Autograd {
    inputs: Vec<Array2<f64>>,
    outputs: Vec<Array2<f64>>,
    gradients: Vec<Array2<f64>>,
}
impl Autograd {
    pub fn new(inputs: Vec<Array2<f64>>) -> Self {
        Autograd {
            inputs,
            outputs: Vec::new(),
            gradients: Vec::new(),
        }
    }
    // Operación de suma
    pub fn add(&mut self, a: &Array2<f64>, b: &Array2<f64>) -> Array2<f64> {
        let output = a + b;
        self.outputs.push(output.clone());
        output
    }
    // Operación de multiplicación
    pub fn mul(&mut self, a: &Array2<f64>, b: &Array2<f64>) -> Array2<f64> {
        let output = a * b;
        self.outputs.push(output.clone());
        output
    }
    // Cálculo de gradientes
    pub fn compute_gradients(&mut self) {
        // Implementación de backpropagation...
    }
}