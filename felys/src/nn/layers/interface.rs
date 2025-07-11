use crate::nn::layers::function::{CrossEntropy, ReLU};
use crate::nn::layers::{Add, Div, Dot, Mul, Neg, Pow, Sub};
use crate::nn::matrix::Matrix;
use crate::nn::optim::Gradients;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub struct Operator {
    pub matrix: Matrix,
    pub layer: Layer,
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.matrix)?;
        let backward = match self.layer {
            Layer::Add(_) => "Add".to_string(),
            Layer::Sub(_) => "Sub".to_string(),
            Layer::Mul(_) => "Mul".to_string(),
            Layer::Div(_) => "Div".to_string(),
            Layer::Pow(_) => "Pow".to_string(),
            Layer::Neg(_) => "Neg".to_string(),
            Layer::Dot(_) => "Dot".to_string(),
            Layer::ReLU(_) => "ReLU".to_string(),
            Layer::CrossEntropy(_) => "CrossEntropy".to_string(),
            Layer::Learnable(id) => format!("Learnable({id})"),
            Layer::Fixed => "Fixed".to_string(),
        };
        write!(f, "(backward: {backward})")
    }
}

impl Operator {
    pub fn new(matrix: Matrix, layer: Layer) -> Self {
        Self { matrix, layer }
    }

    pub fn backward(&self) -> Result<Gradients, String> {
        let grad = Matrix::full(1.0, self.matrix.shape);
        self.layer.backward(grad)
    }
}

#[derive(Clone)]
pub enum Layer {
    Add(Rc<Add>),
    Sub(Rc<Sub>),
    Mul(Rc<Mul>),
    Div(Rc<Div>),
    Pow(Rc<Pow>),
    Neg(Rc<Neg>),
    Dot(Rc<Dot>),
    ReLU(Rc<ReLU>),
    CrossEntropy(Rc<CrossEntropy>),
    Learnable(usize),
    Fixed,
}

impl Layer {
    pub fn backward(&self, grad: Matrix) -> Result<Gradients, String> {
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
            Layer::Learnable(name) => Ok(Gradients::new(*name, grad)),
            Layer::Fixed => Ok(Gradients::empty()),
        }
    }
}

pub trait Differentiable<const S: usize> {
    fn build(input: [Operator; S]) -> Result<Operator, String>;
    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; S], String>;
    fn backward(&self, grad: Matrix) -> Result<Gradients, String> {
        let mut gradients = Gradients::empty();
        for op in self.differentiate(&grad)? {
            let more = op.layer.backward(op.matrix)?;
            gradients.concat(more);
        }
        Ok(gradients)
    }
}
