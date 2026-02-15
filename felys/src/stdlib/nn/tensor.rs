use crate::Object;
use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    data: Rc<[f32]>,
    shape: Rc<[usize]>,
}

impl TryFrom<Object> for Tensor {
    type Error = String;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        let mut shape = Vec::new();

        let mut cursor = &value;
        while let Object::List(list) = cursor
            && !list.is_empty()
        {
            shape.push(list.len());
            cursor = &list[0];
        }

        let size = shape.iter().product();
        let mut data = Vec::with_capacity(size);

        let mut todo = vec![(0, &value)];
        while let Some((depth, object)) = todo.pop() {
            match object {
                Object::List(list) if depth < shape.len() && shape[depth] == list.len() => {
                    for obj in list.iter().rev() {
                        todo.push((depth + 1, obj));
                    }
                }
                Object::Float(x) if depth == shape.len() => {
                    data.push(*x);
                }
                _ => return Err("tensor conversion error".to_string()),
            }
        }

        Ok(Self {
            data: Rc::from(data),
            shape: Rc::from(shape),
        })
    }
}

impl Display for Tensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?}>", self.shape)
    }
}

impl Tensor {
    pub fn binary<F>(&self, other: &Tensor, op: F) -> Result<Self, String>
    where
        F: Fn(f32, f32) -> f32,
    {
        let shape = broadcast(&self.shape, &other.shape)?;
        let rank = shape.len();
        let last = rank - 1;

        let lhs = strides(&self.shape, rank);
        let rhs = strides(&other.shape, rank);
        let steps = (lhs[last], rhs[last]);

        let size = shape.iter().product::<usize>();
        let inner = shape[last];
        let iterations = size / inner;

        let mut index = vec![0; rank.saturating_sub(1)];
        let mut li = 0;
        let mut ri = 0;
        let mut data = Vec::with_capacity(size);

        for _ in 0..iterations {
            match steps {
                (1, 1) => {
                    for (&l, &r) in self.data[li..li + inner]
                        .iter()
                        .zip(other.data[ri..ri + inner].iter())
                    {
                        data.push(op(l, r));
                    }
                }
                (1, 0) => {
                    let r = other.data[ri];
                    for &l in self.data[li..li + inner].iter() {
                        data.push(op(l, r))
                    }
                }
                (0, 1) => {
                    let l = self.data[li];
                    for &r in other.data[ri..ri + inner].iter() {
                        data.push(op(l, r))
                    }
                }
                (0, 0) => {
                    let l = self.data[li];
                    let r = other.data[ri];
                    data.push(op(l, r))
                }
                _ => return Err("binary error".to_string()),
            }

            if rank > 1 {
                for j in (0..rank - 1).rev() {
                    index[j] += 1;
                    if index[j] < shape[j] {
                        li += lhs[j];
                        ri += rhs[j];
                        break;
                    }
                    index[j] = 0;
                    li -= lhs[j] * (shape[j] - 1);
                    ri -= rhs[j] * (shape[j] - 1);
                }
            }
        }

        Ok(Self {
            data: Rc::from(data),
            shape: Rc::from(shape),
        })
    }

    pub fn unary<F>(&self, op: F) -> Self
    where
        F: Fn(f32) -> f32,
    {
        Self {
            data: self.data.iter().copied().map(op).collect(),
            shape: self.shape.clone(),
        }
    }

    pub fn t(&self) -> Self {
        let rank = self.shape.len();

        if rank < 2 {
            return self.clone();
        }

        let mut shape = self.shape.to_vec();
        let rows = shape[rank - 2];
        let cols = shape[rank - 1];
        shape[rank - 2] = cols;
        shape[rank - 1] = rows;

        let iterations: usize = self.shape[..rank - 2].iter().product();
        let size = rows * cols;
        let mut data = Vec::with_capacity(self.data.len());

        for i in 0..iterations {
            let offset = i * size;
            for c in 0..cols {
                for r in 0..rows {
                    data.push(self.data[offset + r * cols + c]);
                }
            }
        }

        Self {
            data: Rc::from(data),
            shape: Rc::from(shape),
        }
    }
}

fn strides(shape: &[usize], rank: usize) -> Vec<usize> {
    let mut strides = vec![0; rank];
    let offset = rank - shape.len();

    let mut s = 1;
    for i in (0..shape.len()).rev() {
        if shape[i] != 1 {
            strides[i + offset] = s;
            s *= shape[i];
        }
    }
    strides
}

fn broadcast(lhs: &[usize], rhs: &[usize]) -> Result<Vec<usize>, String> {
    let rank = max(lhs.len(), rhs.len());
    let mut shape = vec![0; rank];
    let mut lhs = lhs.iter().rev();
    let mut rhs = rhs.iter().rev();
    for i in (0..rank).rev() {
        let l = lhs.next().copied().unwrap_or(1);
        let r = rhs.next().copied().unwrap_or(1);
        if l != 1 && r != 1 && l != r {
            return Err("broadcast error".to_string());
        }
        shape[i] = max(l, r);
    }
    Ok(shape)
}
