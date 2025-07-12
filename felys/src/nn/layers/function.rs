use crate::nn::layers::{Differentiable, Layer, Operator};
use crate::nn::matrix::Matrix;
use crate::Fxx;

pub struct ReLU {
    subtree: [Layer; 1],
    grad: [Matrix; 1],
}

impl Differentiable<1> for ReLU {
    fn build(input: [Operator; 1]) -> Result<Operator, String> {
        let [mut x] = input;

        let mut dx = x.matrix.clone();
        dx.apply(|x| if x > 0.0 { 1.0 } else { 0.0 })?;

        x.matrix.apply(|x| if x > 0.0 { x } else { 0.0 })?;
        let layer = Layer::ReLU(
            Self {
                subtree: [x.layer],
                grad: [dx],
            }
            .into(),
        );
        Ok(Operator::new(x.matrix, layer))
    }

    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; 1], String> {
        let [x] = self.subtree.clone();
        let [mut dx] = self.grad.clone();
        dx.broadcast(grad, |x, y| x * y)?;
        Ok([Operator::new(dx, x)])
    }
}

pub struct CrossEntropy {
    subtree: [Layer; 2],
    softmax: Vec<Fxx>,
}

impl Differentiable<2> for CrossEntropy {
    fn build(input: [Operator; 2]) -> Result<Operator, String> {
        let [x, y] = input;
        let mut logits = x.matrix.vec()?;
        let label = y.matrix.item()? as usize;
        if label > logits.len() {
            return Err(format!(
                "label {label} is greater than logits length {}",
                logits.len(),
            ));
        }

        let max = logits.iter().cloned().fold(Fxx::NEG_INFINITY, Fxx::max);
        logits.iter_mut().for_each(|v| *v = (*v - max).exp());
        let sum = logits.iter().sum::<Fxx>();
        logits.iter_mut().for_each(|v| *v /= sum);

        let mut softmax = logits;
        let loss = -softmax[label].ln();
        let matrix = Matrix::new(vec![loss], (1, 1))?;

        softmax[label] -= 1.0;
        let layer = Layer::CrossEntropy(
            Self {
                subtree: [x.layer, y.layer],
                softmax,
            }
            .into(),
        );
        Ok(Operator::new(matrix, layer))
    }

    fn differentiate(&self, _: &Matrix) -> Result<[Operator; 2], String> {
        let [x, y] = self.subtree.clone();
        let dx = Matrix::from(self.softmax.clone());
        let dy = Matrix::empty();
        Ok([Operator::new(dx, x), Operator::new(dy, y)])
    }
}
