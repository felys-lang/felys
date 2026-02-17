use crate::Object;
use crate::utils::stdlib::nn::tensor::Tensor;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Differentiable<const S: usize>: Debug {
    fn compute(inputs: [Rc<Node>; S]) -> Result<Rc<Node>, String>
    where
        Self: Sized;
    fn differentiate(&self, inputs: [&Tensor; S], grad: Tensor) -> Result<[Tensor; S], String>;
}

#[derive(Debug, Clone)]
pub struct Node {
    tensor: Tensor,
    op: Operator,
}

impl TryFrom<Object> for Node {
    type Error = String;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Ok(Self {
            tensor: value.try_into()?,
            op: Operator::Detached,
        })
    }
}

impl Node {
    pub fn attach(&self, i: i32) -> Result<Self, String> {
        if let Operator::Detached = self.op {
            Ok(Self {
                tensor: self.tensor.clone(),
                op: Operator::Parameter(i, self.tensor.shape.clone()),
            })
        } else {
            Err("cannot attach".to_string())
        }
    }

    pub fn backward(&self) -> Result<HashMap<i32, Tensor>, String> {
        let mut gradients = HashMap::new();
        let ones = Tensor::fill(1.0, self.tensor.shape.as_ref());
        let mut todo = vec![(&self.op, ones)];

        while let Some((operator, grad)) = todo.pop() {
            match operator {
                Operator::Binary(x, y, op) => {
                    let [dx, dy] = op.differentiate([&x.tensor, &y.tensor], grad)?;
                    let dx = dx.unbroadcast(x.tensor.shape.as_ref())?;
                    let dy = dy.unbroadcast(y.tensor.shape.as_ref())?;
                    todo.push((&x.op, dx));
                    todo.push((&y.op, dy));
                }
                Operator::Unary(x, op) => {
                    let [dx] = op.differentiate([&x.tensor], grad)?;
                    let dx = dx.unbroadcast(x.tensor.shape.as_ref())?;
                    todo.push((&x.op, dx));
                }
                Operator::Parameter(i, shape) => {
                    let dx = grad.unbroadcast(shape)?;
                    match gradients.entry(*i) {
                        Entry::Vacant(entry) => {
                            entry.insert(dx);
                        }
                        Entry::Occupied(mut entry) => {
                            let new = entry.get().binary(&dx, Tensor::add)?;
                            entry.insert(new);
                        }
                    }
                }
                Operator::Detached => {}
            }
        }

        Ok(gradients)
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Binary(Rc<Node>, Rc<Node>, Rc<dyn Differentiable<2>>),
    Unary(Rc<Node>, Rc<dyn Differentiable<1>>),
    Parameter(i32, Rc<[usize]>),
    Detached,
}

#[derive(Debug)]
pub struct Add;

impl Differentiable<2> for Add {
    fn compute([x, y]: [Rc<Node>; 2]) -> Result<Rc<Node>, String>
    where
        Self: Sized,
    {
        let tensor = x.tensor.binary(&y.tensor, Tensor::add)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Binary(x, y, Rc::new(Self)),
        }))
    }

    fn differentiate(&self, _: [&Tensor; 2], grad: Tensor) -> Result<[Tensor; 2], String> {
        Ok([grad.clone(), grad])
    }
}

#[derive(Debug)]
pub struct Sub;

impl Differentiable<2> for Sub {
    fn compute([x, y]: [Rc<Node>; 2]) -> Result<Rc<Node>, String> {
        let tensor = x.tensor.binary(&y.tensor, Tensor::sub)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Binary(x, y, Rc::new(Self)),
        }))
    }

    fn differentiate(&self, _: [&Tensor; 2], grad: Tensor) -> Result<[Tensor; 2], String> {
        let neg = grad.unary(Tensor::neg);
        Ok([grad, neg])
    }
}

#[derive(Debug)]
pub struct Mul;

impl Differentiable<2> for Mul {
    fn compute([x, y]: [Rc<Node>; 2]) -> Result<Rc<Node>, String> {
        let tensor = x.tensor.binary(&y.tensor, Tensor::mul)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Binary(x, y, Rc::new(Self)),
        }))
    }

    fn differentiate(&self, [x, y]: [&Tensor; 2], grad: Tensor) -> Result<[Tensor; 2], String> {
        let dx = grad.binary(y, Tensor::mul)?;
        let dy = grad.binary(x, Tensor::mul)?;
        Ok([dx, dy])
    }
}

#[derive(Debug)]
pub struct Div;

impl Differentiable<2> for Div {
    fn compute([x, y]: [Rc<Node>; 2]) -> Result<Rc<Node>, String> {
        let tensor = x.tensor.binary(&y.tensor, Tensor::div)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Binary(x, y, Rc::new(Self)),
        }))
    }

    fn differentiate(&self, [x, y]: [&Tensor; 2], grad: Tensor) -> Result<[Tensor; 2], String> {
        let dx = grad.binary(y, Tensor::div)?;
        let dy = dx.binary(x, Tensor::mul)?.binary(y, |a, b| -a / b)?;
        Ok([dx, dy])
    }
}

#[derive(Debug)]
pub struct MatMul;

impl Differentiable<2> for MatMul {
    fn compute([x, y]: [Rc<Node>; 2]) -> Result<Rc<Node>, String> {
        let tensor = x.tensor.matmul(&y.tensor)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Binary(x, y, Rc::new(Self)),
        }))
    }

    fn differentiate(&self, [x, y]: [&Tensor; 2], grad: Tensor) -> Result<[Tensor; 2], String> {
        let dx = grad.matmul(&y.t())?;
        let dy = x.t().matmul(&grad)?;
        Ok([dx, dy])
    }
}

#[derive(Debug)]
pub struct Neg;

impl Differentiable<1> for Neg {
    fn compute([x]: [Rc<Node>; 1]) -> Result<Rc<Node>, String>
    where
        Self: Sized,
    {
        let tensor = x.tensor.unary(Tensor::neg);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Unary(x, Rc::new(Self)),
        }))
    }

    fn differentiate(&self, _: [&Tensor; 1], grad: Tensor) -> Result<[Tensor; 1], String> {
        Ok([grad.unary(Tensor::neg)])
    }
}
