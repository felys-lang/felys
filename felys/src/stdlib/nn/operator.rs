use crate::Object;
use crate::stdlib::nn::tensor::Tensor;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Node {
    tensor: Tensor,
    op: Operator,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.tensor, self.op)
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add(_, _) => write!(f, "Add"),
            Operator::Sub(_, _) => write!(f, "Sub"),
            Operator::Mul(_, _) => write!(f, "Mul"),
            Operator::Div(_, _) => write!(f, "Div"),
            Operator::MatMul(_, _) => write!(f, "MatMul"),
            Operator::Neg(_) => write!(f, "Neg"),
            Operator::Log(_) => write!(f, "Log"),
            Operator::Exp(_) => write!(f, "Exp"),
            Operator::ReLU(_) => write!(f, "ReLU"),
            Operator::Sum(_, _) => write!(f, "Sum"),
            Operator::Parameter(parameter_id, _) => write!(f, "Parameter<{parameter_id}>"),
            Operator::Detached => write!(f, "Detached"),
        }
    }
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
    Sum(Rc<Node>, Rc<[usize]>),
    Parameter(i32, Rc<[usize]>),
    Detached,
}

impl Operator {
    fn pruned(self) -> Self {
        match &self {
            Operator::Add(lhs, rhs)
            | Operator::Sub(lhs, rhs)
            | Operator::Mul(lhs, rhs)
            | Operator::Div(lhs, rhs)
            | Operator::MatMul(lhs, rhs) => {
                if lhs.fixed() && rhs.fixed() {
                    return Operator::Detached;
                }
            }
            Operator::Neg(input)
            | Operator::Log(input)
            | Operator::Exp(input)
            | Operator::ReLU(input)
            | Operator::Sum(input, _) => {
                if input.fixed() {
                    return Operator::Detached;
                }
            }
            Operator::Parameter(_, _) => {}
            Operator::Detached => {}
        }
        self
    }
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

impl Default for Node {
    fn default() -> Self {
        Self {
            tensor: Tensor::fill(0.0, [].into()),
            op: Operator::Detached,
        }
    }
}

impl Node {
    pub fn new(shape: Rc<[usize]>) -> Self {
        Self {
            tensor: Tensor::new(shape),
            op: Operator::Detached,
        }
    }

    pub fn attach(&self, i: i32) -> Result<Self, String> {
        if let Operator::Detached = self.op {
            let shape = self.tensor.shape.clone();
            Ok(Self {
                tensor: self.tensor.clone(),
                op: Operator::Parameter(i, shape),
            })
        } else {
            Err("cannot attach".to_string())
        }
    }

    pub fn fixed(&self) -> bool {
        matches!(self.op, Operator::Detached)
    }

    pub fn add(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::add)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Add(lhs, rhs).pruned(),
        }))
    }

    pub fn sub(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::sub)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Sub(lhs, rhs).pruned(),
        }))
    }

    pub fn mul(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::mul)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Mul(lhs, rhs).pruned(),
        }))
    }

    pub fn div(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.binary(&rhs.tensor, Tensor::div)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Div(lhs, rhs).pruned(),
        }))
    }

    pub fn matmul(lhs: Rc<Node>, rhs: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = lhs.tensor.matmul(&rhs.tensor)?;
        Ok(Rc::new(Node {
            tensor,
            op: Operator::MatMul(lhs, rhs).pruned(),
        }))
    }

    pub fn neg(input: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = input.tensor.unary(Tensor::neg);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Neg(input).pruned(),
        }))
    }

    pub fn relu(input: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = input.tensor.unary(|i| if i > 0.0 { i } else { 0.0 });
        Ok(Rc::new(Node {
            tensor,
            op: Operator::ReLU(input).pruned(),
        }))
    }

    pub fn ln(input: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = input.tensor.unary(Tensor::ln);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Log(input).pruned(),
        }))
    }

    pub fn exp(input: Rc<Node>) -> Result<Rc<Node>, String> {
        let tensor = input.tensor.unary(Tensor::exp);
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Exp(input).pruned(),
        }))
    }

    pub fn sum(input: Rc<Node>, axes: &[usize], keepdim: bool) -> Result<Rc<Node>, String> {
        let tensor = input.tensor.sum(axes, keepdim)?;
        let mut shape = input.tensor.shape.to_vec();
        for &axis in axes {
            shape[axis] = 1;
        }
        Ok(Rc::new(Node {
            tensor,
            op: Operator::Sum(input, shape.into()).pruned(),
        }))
    }

    pub fn mean(input: Rc<Node>, axes: &[usize], keepdim: bool) -> Result<Rc<Node>, String> {
        let sum = Self::sum(input.clone(), axes, keepdim)?;
        let shape = input.tensor.shape.as_ref();
        let mut element_count = 1;
        for &axis in axes {
            element_count *= shape[axis];
        }
        let denominator = Rc::new(Node {
            tensor: Tensor::fill(element_count as f32, [].into()),
            op: Operator::Detached,
        });
        Self::div(sum, denominator)
    }

    pub fn backward(self: &Rc<Self>) -> Result<HashMap<i32, Rc<Node>>, String> {
        let mut grads = HashMap::new();
        let ones = Tensor::fill(1.0, self.tensor.shape.clone());
        let mut work_queue = vec![(self.clone(), ones)];

        while let Some((node, grad)) = work_queue.pop() {
            let mut push_child_grad =
                |child: &Rc<Node>, child_grad: Tensor| -> Result<(), String> {
                    let unbroadcasted = child_grad.unbroadcast(child.tensor.shape.clone())?;
                    work_queue.push((child.clone(), unbroadcasted));
                    Ok(())
                };

            match &node.op {
                Operator::Add(left, right) => {
                    push_child_grad(left, grad.clone())?;
                    push_child_grad(right, grad)?;
                }
                Operator::Sub(left, right) => {
                    push_child_grad(left, grad.clone())?;
                    push_child_grad(right, grad.unary(Tensor::neg))?;
                }
                Operator::Mul(left, right) => {
                    push_child_grad(left, grad.binary(&right.tensor, Tensor::mul)?)?;
                    push_child_grad(right, grad.binary(&left.tensor, Tensor::mul)?)?;
                }
                Operator::Div(left, right) => {
                    let left_grad = grad.binary(&right.tensor, Tensor::div)?;
                    let right_grad = left_grad
                        .binary(&left.tensor, Tensor::mul)?
                        .binary(&right.tensor, |a, b| -a / b)?;
                    push_child_grad(left, left_grad)?;
                    push_child_grad(right, right_grad)?;
                }
                Operator::MatMul(left, right) => {
                    push_child_grad(left, grad.matmul(&right.tensor.t())?)?;
                    push_child_grad(right, left.tensor.t().matmul(&grad)?)?;
                }
                Operator::Neg(src) => {
                    push_child_grad(src, grad.unary(Tensor::neg))?;
                }
                Operator::Log(src) => {
                    push_child_grad(src, grad.binary(&src.tensor, Tensor::div)?)?;
                }
                Operator::Exp(src) => {
                    let src_grad = node.tensor.binary(&grad, Tensor::mul)?;
                    push_child_grad(src, src_grad)?;
                }
                Operator::ReLU(src) => {
                    let src_grad =
                        grad.binary(&src.tensor, |g, i| if i > 0.0 { g } else { 0.0 })?;
                    push_child_grad(src, src_grad)?;
                }
                Operator::Sum(src, shape) => {
                    let ones = Tensor::fill(1.0, src.tensor.shape.clone());
                    let mut grad = grad;
                    grad.shape = shape.clone();
                    let broadcasted = ones.binary(&grad, Tensor::mul)?;
                    push_child_grad(src, broadcasted)?;
                }
                Operator::Parameter(param_id, _shape) => match grads.entry(*param_id) {
                    Entry::Vacant(entry) => {
                        entry.insert(grad);
                    }
                    Entry::Occupied(mut entry) => {
                        let new_grad = entry.get().binary(&grad, Tensor::add)?;
                        entry.insert(new_grad);
                    }
                },
                Operator::Detached => {}
            }
        }

        Ok(grads
            .into_iter()
            .map(|(param_id, grad_tensor)| {
                (
                    param_id,
                    Node {
                        tensor: grad_tensor,
                        op: Operator::Detached,
                    }
                    .into(),
                )
            })
            .collect())
    }
}
