use crate::nn::layers::function::{CrossEntropy, ReLU};
use crate::nn::layers::{Add, Div, Dot, Mul, Neg, Pow, Sub};
use crate::nn::matrix::Matrix;
use crate::nn::optim::Gradients;

pub struct Operator {
    pub matrix: Matrix,
    pub layer: Layer,
}

impl Operator {
    pub fn new(matrix: Matrix, layer: Layer) -> Self {
        Self { matrix, layer }
    }
}

pub enum Layer {
    Add(Box<Add>),
    Sub(Box<Sub>),
    Mul(Box<Mul>),
    Div(Box<Div>),
    Pow(Box<Pow>),
    Neg(Box<Neg>),
    Dot(Box<Dot>),
    ReLU(Box<ReLU>),
    CrossEntropy(Box<CrossEntropy>),
    Learnable(usize),
    Fixed,
}

impl Layer {
    pub fn backward(self, grad: Matrix) -> Result<Gradients, String> {
        match self {
            Layer::Add(x) => x.backward(grad),
            Layer::Sub(x) => x.backward(grad),
            Layer::Mul(x) => x.backward(grad),
            Layer::Div(x) => x.backward(grad),
            Layer::Pow(x) => x.backward(grad),
            Layer::Neg(x) => x.backward(grad),
            Layer::Dot(x) => x.backward(grad),
            Layer::ReLU(x) => x.backward(grad),
            Layer::CrossEntropy(x) => x.backward(grad),
            Layer::Learnable(name) => Ok(Gradients::new(name, grad)),
            Layer::Fixed => Ok(Gradients::empty()),
        }
    }
}

pub trait Differentiable<const S: usize> {
    fn build(input: [Operator; S]) -> Result<Operator, String>;
    fn differentiate(self, grad: &Matrix) -> Result<[Operator; S], String>;
    fn backward(self, grad: Matrix) -> Result<Gradients, String>
    where
        Self: Sized,
    {
        let mut gradients = Gradients::empty();
        for op in self.differentiate(&grad)? {
            let more = op.layer.backward(op.matrix)?;
            gradients.concat(more);
        }
        Ok(gradients)
    }
}
