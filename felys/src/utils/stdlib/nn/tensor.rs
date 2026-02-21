use crate::Object;
use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    data: Rc<[f32]>,
    pub shape: Rc<[usize]>,
}

impl Display for Tensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn dfs(
            f: &mut Formatter<'_>,
            data: &[f32],
            shape: &[usize],
            offset: &mut usize,
        ) -> std::fmt::Result {
            if shape.is_empty() {
                write!(f, "{:?}", data[*offset])?;
                *offset += 1;
                return Ok(());
            }

            write!(f, "[")?;
            let len = shape[0];
            let rest = &shape[1..];

            for i in 0..len {
                if i > 0 {
                    write!(f, ", ")?;
                }
                dfs(f, data, rest, offset)?;
            }
            write!(f, "]")
        }

        let mut offset = 0;

        if self.shape.is_empty() {
            return write!(f, "{}", self.data[0]);
        }

        dfs(f, &self.data, &self.shape, &mut offset)
    }
}

impl TryFrom<Object> for Tensor {
    type Error = String;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        if let Object::Float(x) = value {
            return Ok(Self::fill(x, [].into()));
        }

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

impl Tensor {
    pub fn add(lhs: f32, rhs: f32) -> f32 {
        lhs + rhs
    }

    pub fn sub(lhs: f32, rhs: f32) -> f32 {
        lhs - rhs
    }

    pub fn mul(lhs: f32, rhs: f32) -> f32 {
        lhs * rhs
    }

    pub fn div(lhs: f32, rhs: f32) -> f32 {
        lhs / rhs
    }

    pub fn neg(x: f32) -> f32 {
        -x
    }

    pub fn ln(x: f32) -> f32 {
        x.ln()
    }

    pub fn exp(x: f32) -> f32 {
        x.exp()
    }
}

impl Tensor {
    pub fn fill(x: f32, shape: Rc<[usize]>) -> Self {
        let size = shape.iter().product();
        Self {
            data: Rc::from(vec![x; size]),
            shape,
        }
    }

    pub fn binary<F>(&self, other: &Tensor, op: F) -> Result<Self, String>
    where
        F: Fn(f32, f32) -> f32,
    {
        if self.shape == other.shape {
            let data = self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(&l, &r)| op(l, r))
                .collect::<Vec<_>>();
            return Ok(Self {
                data: Rc::from(data),
                shape: self.shape.clone(),
            });
        }

        let shape = broadcast(&self.shape, &other.shape)?;
        let rank = shape.len();

        let lhs = strides(&self.shape, rank);
        let rhs = strides(&other.shape, rank);
        let steps = (lhs[rank - 1], rhs[rank - 1]);

        let size = shape.iter().product();
        let inner = shape[rank - 1];

        let mut indices = vec![0; rank.saturating_sub(1)];
        let mut li = 0;
        let mut ri = 0;
        let mut data = Vec::with_capacity(size);

        for _ in 0..size / inner {
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
                    indices[j] += 1;
                    if indices[j] < shape[j] {
                        li += lhs[j];
                        ri += rhs[j];
                        break;
                    }
                    indices[j] = 0;
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

        let iterations = self.shape[..rank - 2].iter().product();
        let mut data = Vec::with_capacity(self.data.len());

        for i in 0..iterations {
            let offset = i * rows * cols;
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

    pub fn matmul(&self, other: &Tensor) -> Result<Self, String> {
        if self.shape.len() < 2 || other.shape.len() < 2 {
            return Err("matmul requires at least 2 dimensions".to_string());
        }

        let ls = self.shape.as_ref();
        let rs = other.shape.as_ref();

        let m = ls[ls.len() - 2];
        let k = ls[ls.len() - 1];
        let n = rs[rs.len() - 1];

        if k != rs[rs.len() - 2] {
            return Err("matmul dimension mismatch".to_string());
        }

        let lb = &ls[..ls.len() - 2];
        let rb = &rs[..rs.len() - 2];

        let mut shape = broadcast(lb, rb)?;
        let rank = shape.len();
        let size = shape.iter().product();

        let lhs = strides(lb, rank);
        let rhs = strides(rb, rank);

        let mut indices = vec![0; rank];
        let mut li = 0;
        let mut ri = 0;
        let mut data = Vec::with_capacity(size * m * n);

        for _ in 0..size {
            let lbi = li * m * k;
            let rbi = ri * k * n;

            for i in 0..m {
                for j in 0..n {
                    let mut sum = 0.0;
                    for l in 0..k {
                        sum += self.data[lbi + i * k + l] * other.data[rbi + l * n + j];
                    }
                    data.push(sum);
                }
            }

            for j in (0..rank).rev() {
                indices[j] += 1;
                if indices[j] < shape[j] {
                    li += lhs[j];
                    ri += rhs[j];
                    break;
                }
                indices[j] = 0;
                li -= lhs[j] * (shape[j] - 1);
                ri -= rhs[j] * (shape[j] - 1);
            }
        }

        shape.push(m);
        shape.push(n);

        Ok(Self {
            data: Rc::from(data),
            shape: Rc::from(shape),
        })
    }

    pub fn unbroadcast(&self, target: Rc<[usize]>) -> Result<Self, String> {
        if self.shape == target {
            return Ok(self.clone());
        }

        if self.shape.as_ref() != broadcast(&self.shape, target.as_ref())? {
            return Err("unbroadcast failed".to_string());
        }

        let rank = self.shape.len();
        let size = target.iter().product();
        let strides = strides(target.as_ref(), rank);

        let mut indices = vec![0; rank];
        let mut data = vec![0.0; size];

        for x in self.data.iter() {
            let mut index = 0;
            for i in 0..rank {
                index += indices[i] * strides[i];
            }
            data[index] += x;

            if rank > 0 {
                for j in (0..rank).rev() {
                    indices[j] += 1;
                    if indices[j] < self.shape[j] {
                        break;
                    }
                    indices[j] = 0;
                }
            }
        }

        Ok(Self {
            data: Rc::from(data),
            shape: target,
        })
    }

    pub fn sum(&self, axes: &[usize]) -> Result<Self, String> {
        if axes.is_empty() {
            return Ok(self.clone());
        }

        let mut shape = self.shape.to_vec();
        let mut target = Vec::new();
        let rank = shape.len();

        for axis in axes {
            if *axis >= rank {
                return Err("axes must be less than rank".to_string());
            }
        }

        for (i, length) in shape.iter_mut().enumerate() {
            if axes.contains(&i) {
                *length = 1;
            } else {
                target.push(*length);
            }
        }

        let mut tensor = self.unbroadcast(shape.into())?;
        tensor.shape = target.into();
        Ok(tensor)
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

#[cfg(test)]
mod tests {
    use super::*;

    impl Tensor {
        fn range(shape: Vec<usize>, offset: usize) -> Self {
            let length = shape.iter().product::<usize>();
            Self {
                data: (offset..length + offset).map(|x| x as f32).collect(),
                shape: shape.into(),
            }
        }
    }

    #[test]
    fn binary() {
        let lhs = Tensor::range(vec![], 0);
        let rhs = Tensor::range(vec![], 1);
        let res = lhs.binary(&rhs, Tensor::add).unwrap();
        assert_eq!(res.shape.as_ref(), &[]);
        assert_eq!(res.data.as_ref(), &[1.0]);

        let lhs = Tensor::range(vec![], 10);
        let rhs = Tensor::range(vec![3], 0);
        let res = lhs.binary(&rhs, Tensor::add).unwrap();
        assert_eq!(res.shape.as_ref(), &[3]);
        assert_eq!(res.data.as_ref(), &[10.0, 11.0, 12.0]);

        let lhs = Tensor::range(vec![3], 0);
        let rhs = Tensor::range(vec![2, 3], 10);
        let res = lhs.binary(&rhs, Tensor::add).unwrap();
        assert_eq!(res.shape.as_ref(), &[2, 3]);
        assert_eq!(res.data.as_ref(), &[10.0, 12.0, 14.0, 13.0, 15.0, 17.0]);

        let lhs = Tensor::range(vec![2, 3], 0);
        let rhs = Tensor::range(vec![2, 1], 10);
        let res = lhs.binary(&rhs, Tensor::add).unwrap();
        assert_eq!(res.shape.as_ref(), &[2, 3]);
        assert_eq!(res.data.as_ref(), &[10.0, 11.0, 12.0, 14.0, 15.0, 16.0]);

        let lhs = Tensor::range(vec![1, 3], 0);
        let rhs = Tensor::range(vec![2, 1], 10);
        let res = lhs.binary(&rhs, Tensor::add).unwrap();
        assert_eq!(res.shape.as_ref(), &[2, 3]);
        assert_eq!(res.data.as_ref(), &[10.0, 11.0, 12.0, 11.0, 12.0, 13.0]);

        let lhs = Tensor::range(vec![2, 1], 0);
        let rhs = Tensor::range(vec![2, 1], 10);
        let res = lhs.binary(&rhs, Tensor::add).unwrap();
        assert_eq!(res.shape.as_ref(), &[2, 1]);
        assert_eq!(res.data.as_ref(), &[10.0, 12.0]);

        let lhs = Tensor::range(vec![2, 3, 1], 0);
        let rhs = Tensor::range(vec![1, 1, 4], 10);
        let res = lhs.binary(&rhs, Tensor::add).unwrap();
        assert_eq!(res.shape.as_ref(), &[2, 3, 4]);
        assert_eq!(
            res.data.as_ref(),
            &[
                10.0, 11.0, 12.0, 13.0, 11.0, 12.0, 13.0, 14.0, 12.0, 13.0, 14.0, 15.0, 13.0, 14.0,
                15.0, 16.0, 14.0, 15.0, 16.0, 17.0, 15.0, 16.0, 17.0, 18.0
            ]
        );

        let lhs = Tensor::range(vec![2], 0);
        let rhs = Tensor::range(vec![3], 0);
        assert!(lhs.binary(&rhs, Tensor::add).is_err());

        let lhs = Tensor::range(vec![2, 3], 0);
        let rhs = Tensor::range(vec![3, 2], 0);
        assert!(lhs.binary(&rhs, Tensor::add).is_err());
    }

    #[test]
    fn t() {
        let t = Tensor::range(vec![], 5);
        let res = t.t();
        assert_eq!(res.shape.as_ref(), &[]);
        assert_eq!(res.data.as_ref(), &[5.0]);

        let t = Tensor::range(vec![3], 0);
        let res = t.t();
        assert_eq!(res.shape.as_ref(), &[3]);
        assert_eq!(res.data.as_ref(), &[0.0, 1.0, 2.0]);

        let t = Tensor::range(vec![2, 3], 0);
        let res = t.t();
        assert_eq!(res.shape.as_ref(), &[3, 2]);
        assert_eq!(res.data.as_ref(), &[0.0, 3.0, 1.0, 4.0, 2.0, 5.0]);

        let t = Tensor::range(vec![2, 3, 4], 0);
        let res = t.t();
        assert_eq!(res.shape.as_ref(), &[2, 4, 3]);
        assert_eq!(
            res.data.as_ref(),
            &[
                0.0, 4.0, 8.0, 1.0, 5.0, 9.0, 2.0, 6.0, 10.0, 3.0, 7.0, 11.0, 12.0, 16.0, 20.0,
                13.0, 17.0, 21.0, 14.0, 18.0, 22.0, 15.0, 19.0, 23.0
            ]
        );
    }

    #[test]
    fn matmul() {
        let lhs = Tensor::range(vec![2, 3], 0);
        let rhs = Tensor::range(vec![3, 2], 6);
        let res = lhs.matmul(&rhs).unwrap();
        assert_eq!(res.shape.as_ref(), &[2, 2]);
        assert_eq!(res.data.as_ref(), &[28.0, 31.0, 100.0, 112.0]);

        let lhs = Tensor::range(vec![2, 2, 3], 0);
        let rhs = Tensor::range(vec![2, 3, 4], 12);
        let res = lhs.matmul(&rhs).unwrap();
        assert_eq!(res.shape.as_ref(), &[2, 2, 4]);
        assert_eq!(
            &res.data.as_ref(),
            &[
                56.0, 59.0, 62.0, 65.0, 200.0, 212.0, 224.0, 236.0, 596.0, 617.0, 638.0, 659.0,
                848.0, 878.0, 908.0, 938.0
            ]
        );

        let lhs = Tensor::range(vec![2, 3], 0);
        let rhs = Tensor::range(vec![2, 3], 0);
        assert!(lhs.matmul(&rhs).is_err());

        let lhs = Tensor::range(vec![3], 0);
        let rhs = Tensor::range(vec![3, 2], 0);
        assert!(lhs.matmul(&rhs).is_err());
    }

    #[test]
    fn sum() {
        let t = Tensor::range(vec![2, 3], 0);
        let res = t.sum(&[1]).unwrap();
        assert_eq!(res.shape.as_ref(), &[2]);
        assert_eq!(res.data.as_ref(), &[3.0, 12.0]);

        let t = Tensor::range(vec![2, 3], 0);
        let res = t.sum(&[0]).unwrap();
        assert_eq!(res.shape.as_ref(), &[3]);
        assert_eq!(res.data.as_ref(), &[3.0, 5.0, 7.0]);

        let t = Tensor::range(vec![2, 3], 0);
        let res = t.sum(&[0, 1]).unwrap();
        assert_eq!(res.shape.as_ref(), &[]);
        assert_eq!(res.data.as_ref(), &[15.0]);

        let t = Tensor::range(vec![2, 3, 4], 0);
        let res = t.sum(&[2]).unwrap();
        assert_eq!(res.shape.as_ref(), &[2, 3]);
        assert_eq!(res.data.as_ref(), &[6.0, 22.0, 38.0, 54.0, 70.0, 86.0]);

        let t = Tensor::range(vec![2, 3, 4], 0);
        let res = t.sum(&[1, 2]).unwrap();
        assert_eq!(res.shape.as_ref(), &[2]);
        assert_eq!(res.data.as_ref(), &[66.0, 210.0]);
    }
}
