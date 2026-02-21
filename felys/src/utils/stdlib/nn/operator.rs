use crate::Object;
use crate::utils::stdlib::nn::tensor::Tensor;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node {
    pub tensor: Tensor,
    pub op: Operator,
}

#[derive(Clone, Debug)]
pub enum Operator {
    Add(Rc<Node>, Rc<Node>),
    Sub(Rc<Node>, Rc<Node>),
    Mul(Rc<Node>, Rc<Node>),
    Div(Rc<Node>, Rc<Node>),
    MatMul(Rc<Node>, Rc<Node>),
    Neg(Rc<Node>),
    Log(Rc<Node>),
    Exp(Rc<Node>),
    ReLU(Rc<Node>),
    Sum(Rc<Node>),
    Parameter(i32, Rc<[usize]>),
    Detached,
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

    pub fn add(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::add)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Add(lhs, rhs),
        }))
    }

    pub fn sub(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::sub)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Sub(lhs, rhs),
        }))
    }

    pub fn mul(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::mul)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Mul(lhs, rhs),
        }))
    }

    pub fn div(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::div)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Div(lhs, rhs),
        }))
    }

    pub fn matmul(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.matmul(&rhs.tensor)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::MatMul(lhs, rhs),
        }))
    }

    pub fn neg(src: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.unary(Tensor::neg);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Neg(src),
        }))
    }

    pub fn relu(src: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.unary(|i| if i > 0.0 { i } else { 0.0 });
        Ok(Rc::new(Node {
            tensor,
            op: Operator::ReLU(src),
        }))
    }

    pub fn ln(src: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.unary(Tensor::ln);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Log(src),
        }))
    }

    pub fn exp(src: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.unary(Tensor::exp);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Exp(src),
        }))
    }

    pub fn sum(src: Rc<Node>, axes: &[usize]) -> Result<Rc<Node>, String> {
        let tensor = src.tensor.sum(axes)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Sum(src),
        }))
    }

    pub fn backward(self: &Rc<Self>) -> Result<HashMap<i32, Tensor>, String> {
        let mut gradients = HashMap::new();
        let ones = Tensor::fill(1.0, self.tensor.shape.clone());
        let mut todo = vec![(self.clone(), ones)];

        while let Some((node, grad)) = todo.pop() {
            let mut push = |child: &Rc<Node>, g: Tensor| -> Result<(), String> {
                let unbroadcasted = g.unbroadcast(child.tensor.shape.clone())?;
                todo.push((child.clone(), unbroadcasted));
                Ok(())
            };

            match &node.op {
                Operator::Add(lhs, rhs) => {
                    push(lhs, grad.clone())?;
                    push(rhs, grad)?;
                }
                Operator::Sub(lhs, rhs) => {
                    push(lhs, grad.clone())?;
                    push(rhs, grad.unary(Tensor::neg))?;
                }
                Operator::Mul(lhs, rhs) => {
                    push(lhs, grad.binary(&rhs.tensor, Tensor::mul)?)?;
                    push(rhs, grad.binary(&lhs.tensor, Tensor::mul)?)?;
                }
                Operator::Div(lhs, rhs) => {
                    let dx = grad.binary(&rhs.tensor, Tensor::div)?;
                    let dy = dx
                        .binary(&lhs.tensor, Tensor::mul)?
                        .binary(&rhs.tensor, |a, b| -a / b)?;
                    push(lhs, dx)?;
                    push(rhs, dy)?;
                }
                Operator::MatMul(lhs, rhs) => {
                    push(lhs, grad.matmul(&rhs.tensor.t())?)?;
                    push(rhs, lhs.tensor.t().matmul(&grad)?)?;
                }
                Operator::Neg(src) => {
                    push(src, grad.unary(Tensor::neg))?;
                }
                Operator::Log(src) => {
                    push(src, grad.binary(&src.tensor, Tensor::div)?)?;
                }
                Operator::Exp(src) => {
                    let dx = node.tensor.binary(&grad, Tensor::mul)?;
                    push(src, dx)?;
                }
                Operator::ReLU(src) => {
                    let dx = grad.binary(&src.tensor, |g, i| if i > 0.0 { g } else { 0.0 })?;
                    push(src, dx)?;
                }
                Operator::Sum(src) => {
                    let ones = Tensor::fill(1.0, src.tensor.shape.clone());
                    let broadcasted = ones.binary(&grad, Tensor::mul)?;
                    push(src, broadcasted)?;
                }
                Operator::Parameter(i, _shape) => match gradients.entry(*i) {
                    Entry::Vacant(entry) => {
                        entry.insert(grad);
                    }
                    Entry::Occupied(mut entry) => {
                        let new = entry.get().binary(&grad, Tensor::add)?;
                        entry.insert(new);
                    }
                },
                Operator::Detached => {}
            }
        }

        Ok(gradients)
    }
}
