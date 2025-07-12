use crate::nn::matrix::Matrix;
use std::collections::HashMap;

pub struct Gradients {
    container: Vec<(usize, Matrix)>,
}

impl Gradients {
    pub fn new(id: usize, grad: Matrix) -> Self {
        Self {
            container: vec![(id, grad)],
        }
    }

    pub fn empty() -> Self {
        Self { container: vec![] }
    }

    pub fn concat(&mut self, other: Self) {
        self.container.extend(other.container);
    }

    pub fn make(self) -> Result<HashMap<usize, Matrix>, String> {
        let mut result = HashMap::<usize, Matrix>::new();
        for (id, grad) in self.container {
            if let Some(existing) = result.get_mut(&id) {
                existing.broadcast(&grad, |x, y| x + y)?;
            } else {
                result.insert(id, grad);
            }
        }
        Ok(result)
    }
}

pub type Parameters = HashMap<usize, (Matrix, Matrix)>;

pub struct Optimizer {
    parameters: Parameters,
    momentum: f64,
    random: Random,
}

impl Optimizer {
    pub fn new(parameters: Parameters, momentum: f64, seed: usize) -> Self {
        Self {
            parameters,
            momentum,
            random: Random::new(seed),
        }
    }

    pub fn export(self) -> Parameters {
        self.parameters
    }

    pub fn get(&mut self, id: &usize, shape: (usize, usize)) -> Result<Matrix, String> {
        let (matrix, _) = self
            .parameters
            .entry(*id)
            .or_insert((self.random.matrix(shape)?, Matrix::full(0.0, shape)));
        Ok(matrix.clone())
    }

    pub fn step(&mut self, gradients: Gradients, lr: f64) -> Result<(), String> {
        for (id, grad) in gradients.make()? {
            if let Some((x, m)) = self.parameters.get_mut(&id) {
                m.broadcast(&grad, |mu, gt| self.momentum * mu - lr * gt)?;
                x.broadcast(m, |theta, vt| theta + vt)?;
            } else {
                return Err(format!("parameter {id} does not exist"));
            }
        }
        Ok(())
    }
}

pub struct Random {
    state: usize,
}

impl Random {
    fn new(seed: usize) -> Self {
        Random { state: seed }
    }

    fn signed(&mut self) -> f64 {
        const A: usize = 1664525;
        const C: usize = 1013904223;
        const M: usize = 1 << 32;

        self.state = (A.wrapping_mul(self.state).wrapping_add(C)) % M;
        (self.state as f64) / (M as f64) * 2.0 - 1.0
    }

    fn matrix(&mut self, shape: (usize, usize)) -> Result<Matrix, String> {
        let length = shape.0 * shape.1;
        let mut data = Vec::with_capacity(length);
        for _ in 0..length {
            data.push(self.signed() * 0.1)
        }
        Matrix::new(data, shape)
    }
}
