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

pub struct Optimizer {
    parameters: HashMap<usize, (Matrix, Matrix)>,
    momentum: f64,
}

impl Optimizer {
    pub fn new(parameters: HashMap<usize, Matrix>, momentum: f64) -> Self {
        let params = parameters
            .into_iter()
            .map(|(id, x)| (id, (Matrix::full(0.0, x.shape), x)))
            .collect();
        Self {
            parameters: params,
            momentum,
        }
    }

    pub fn export(self) -> HashMap<usize, (Matrix, Matrix)> {
        self.parameters
    }

    pub fn get(&self, id: &usize) -> Result<Matrix, String> {
        let (matrix, _) = self
            .parameters
            .get(id)
            .cloned()
            .ok_or(format!("parameter {id} does not exist"))?;
        Ok(matrix)
    }

    pub fn step(&mut self, gradients: Gradients, lr: f64) -> Result<(), String> {
        for (id, grad) in gradients.make()? {
            if let Some((m, x)) = self.parameters.get_mut(&id) {
                m.broadcast(&grad, |mu, gt| self.momentum * mu - lr * gt)?;
                x.broadcast(m, |theta, vt| theta + vt)?;
            } else {
                return Err(format!("parameter {id} does not exist"));
            }
        }
        Ok(())
    }
}
