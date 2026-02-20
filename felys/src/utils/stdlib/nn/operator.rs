use crate::utils::stdlib::nn::tensor::Tensor;
use crate::Object;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

trait Differentiable<const S: usize> {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); S], String>;

    fn backward(&self, grad: Tensor, todo: &mut Vec<(Operator, Tensor)>) -> Result<(), String> {
        for (node, tensor) in self.differentiate(grad)? {
            let op = node.op.clone();
            let unbroadcasted = tensor.unbroadcast(node.tensor.shape.clone())?;
            todo.push((op, unbroadcasted));
        }
        Ok(())
    }
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
    pub fn attach(self, i: i32) -> Result<Self, String> {
        if let Operator::Detached = self.op {
            let shape = self.tensor.shape.clone();
            Ok(Self {
                tensor: self.tensor,
                op: Operator::Parameter(i, shape),
            })
        } else {
            Err("cannot attach".to_string())
        }
    }

    pub fn backward(&self) -> Result<HashMap<i32, Tensor>, String> {
        let mut gradients = HashMap::new();
        let ones = Tensor::fill(1.0, self.tensor.shape.clone());
        let mut todo = vec![(self.op.clone(), ones)];

        while let Some((operator, grad)) = todo.pop() {
            match operator {
                Operator::Add(x) => x.backward(grad, &mut todo)?,
                Operator::Sub(x) => x.backward(grad, &mut todo)?,
                Operator::Mul(x) => x.backward(grad, &mut todo)?,
                Operator::Div(x) => x.backward(grad, &mut todo)?,
                Operator::MatMul(x) => x.backward(grad, &mut todo)?,
                Operator::Log(x) => x.backward(grad, &mut todo)?,
                Operator::Exp(x) => x.backward(grad, &mut todo)?,
                Operator::Neg(x) => x.backward(grad, &mut todo)?,
                Operator::Sum(x) => x.backward(grad, &mut todo)?,
                Operator::ReLU(x) => x.backward(grad, &mut todo)?,
                Operator::Parameter(i, shape) => {
                    let dx = grad.unbroadcast(shape.clone())?;
                    match gradients.entry(i) {
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
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    MatMul(MatMul),
    Neg(Neg),
    Log(Ln),
    Exp(Exp),
    ReLU(ReLU),
    Sum(Sum),
    Parameter(i32, Rc<[usize]>),
    Detached,
}

#[derive(Clone, Debug)]
pub struct Add {
    lhs: Rc<Node>,
    rhs: Rc<Node>,
}

impl Add {
    pub fn compute(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::add)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Add(Add { lhs, rhs }),
        }))
    }
}

impl Differentiable<2> for Add {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 2], String> {
        let lhs = (self.lhs.clone(), grad.clone());
        let rhs = (self.rhs.clone(), grad.clone());
        Ok([lhs, rhs])
    }
}
#[derive(Clone, Debug)]
pub struct Sub {
    lhs: Rc<Node>,
    rhs: Rc<Node>,
}

impl Sub {
    pub fn compute(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::sub)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Sub(Sub { lhs, rhs }),
        }))
    }
}

impl Differentiable<2> for Sub {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 2], String> {
        let dy = grad.unary(Tensor::neg);
        Ok([(self.lhs.clone(), grad), (self.rhs.clone(), dy)])
    }
}
#[derive(Clone, Debug)]
pub struct Mul {
    lhs: Rc<Node>,
    rhs: Rc<Node>,
}

impl Mul {
    pub fn compute(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::mul)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Mul(Mul { lhs, rhs }),
        }))
    }
}

impl Differentiable<2> for Mul {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 2], String> {
        let dx = grad.binary(&self.rhs.tensor, Tensor::mul)?;
        let dy = grad.binary(&self.lhs.tensor, Tensor::mul)?;
        Ok([(self.lhs.clone(), dx), (self.rhs.clone(), dy)])
    }
}
#[derive(Clone, Debug)]
pub struct Div {
    lhs: Rc<Node>,
    rhs: Rc<Node>,
}

impl Div {
    pub fn compute(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::div)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Div(Div { lhs, rhs }),
        }))
    }
}

impl Differentiable<2> for Div {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 2], String> {
        let dx = grad.binary(&self.rhs.tensor, Tensor::div)?;
        let dy = dx
            .binary(&self.lhs.tensor, Tensor::mul)?
            .binary(&self.rhs.tensor, |a, b| -a / b)?;

        Ok([(self.lhs.clone(), dx), (self.rhs.clone(), dy)])
    }
}
#[derive(Clone, Debug)]
pub struct MatMul {
    lhs: Rc<Node>,
    rhs: Rc<Node>,
}

impl MatMul {
    pub fn compute(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.matmul(&rhs.tensor)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::MatMul(MatMul { lhs, rhs }),
        }))
    }
}

impl Differentiable<2> for MatMul {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 2], String> {
        let dx = grad.matmul(&self.rhs.tensor.t())?;
        let dy = self.lhs.tensor.t().matmul(&grad)?;
        Ok([(self.lhs.clone(), dx), (self.rhs.clone(), dy)])
    }
}
#[derive(Clone, Debug)]
pub struct Neg {
    src: Rc<Node>,
}

impl Neg {
    pub fn compute(src: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.unary(Tensor::neg);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Neg(Neg { src }),
        }))
    }
}

impl Differentiable<1> for Neg {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 1], String> {
        Ok([(self.src.clone(), grad.unary(Tensor::neg))])
    }
}
#[derive(Clone, Debug)]
pub struct ReLU {
    src: Rc<Node>,
}

impl ReLU {
    pub fn compute(src: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.unary(|i| if i > 0.0 { i } else { 0.0 });
        Ok(Rc::new(Node {
            tensor,
            op: Operator::ReLU(ReLU { src }),
        }))
    }
}

impl Differentiable<1> for ReLU {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 1], String> {
        let dx = grad.binary(&self.src.tensor, |g, i| if i > 0.0 { g } else { 0.0 })?;
        Ok([(self.src.clone(), dx)])
    }
}

#[derive(Clone, Debug)]
pub struct Ln {
    src: Rc<Node>,
}

impl Ln {
    pub fn compute(src: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.unary(Tensor::ln);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Log(Ln { src }),
        }))
    }
}

impl Differentiable<1> for Ln {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 1], String> {
        let dx = grad.binary(&self.src.tensor, Tensor::div)?;
        Ok([(self.src.clone(), dx)])
    }
}

#[derive(Clone, Debug)]
pub struct Exp {
    src: Rc<Node>,
    cache: Tensor,
}

impl Exp {
    pub fn compute(src: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.unary(Tensor::exp);
        Ok(Rc::new(Node {
            tensor: tensor.clone(),
            op: Operator::Exp(Exp { src, cache: tensor }),
        }))
    }
}

impl Differentiable<1> for Exp {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 1], String> {
        let dx = self.cache.binary(&grad, Tensor::mul)?;
        Ok([(self.src.clone(), dx)])
    }
}

#[derive(Clone, Debug)]
pub struct Sum {
    src: Rc<Node>,
}

impl Sum {
    pub fn compute(src: Rc<Node>, axes: &[usize]) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.sum(axes)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Sum(Sum { src }),
        }))
    }
}

impl Differentiable<1> for Sum {
    fn differentiate(&self, grad: Tensor) -> Result<[(Rc<Node>, Tensor); 1], String> {
        let ones = Tensor::fill(1.0, self.src.tensor.shape.clone());
        let broadcasted = ones.binary(&grad, Tensor::mul)?;
        Ok([(self.src.clone(), broadcasted)])
    }
}
