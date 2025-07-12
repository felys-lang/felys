use crate::Fxx;
use std::fmt::{Display, Formatter};

pub struct Matrix {
    linear: Vec<Fxx>,
    pub shape: (usize, usize),
}

impl From<Vec<Fxx>> for Matrix {
    fn from(value: Vec<Fxx>) -> Self {
        let length = value.len();
        Self {
            linear: value,
            shape: (1, length),
        }
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Self {
            linear: self.linear.clone(),
            shape: self.shape,
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for row in 0..self.shape.0 {
            let start = row * self.shape.1;
            let end = start + self.shape.1;
            write!(f, "  ")?;
            for val in &self.linear[start..end] {
                write!(f, "{val:.5} ")?;
            }
            writeln!(f, ";")?;
        }
        write!(f, "]")
    }
}

impl Matrix {
    pub fn new(data: Vec<Fxx>, shape: (usize, usize)) -> Result<Self, String> {
        if data.len() != shape.0 * shape.1 {
            return Err(format!("{shape:?} does not match data"));
        }
        Ok(Self {
            linear: data,
            shape,
        })
    }

    pub fn full(fill: Fxx, shape: (usize, usize)) -> Self {
        let length = shape.0 * shape.1;
        Self {
            linear: vec![fill; length],
            shape,
        }
    }

    pub fn empty() -> Self {
        Self {
            linear: vec![],
            shape: (0, 0),
        }
    }

    pub fn vec(self) -> Result<Vec<Fxx>, String> {
        if self.shape.0 != 1 {
            return Err(format!("{:?} is not a vector", self.shape));
        }
        Ok(self.linear)
    }

    pub fn item(self) -> Result<Fxx, String> {
        if (self.shape) != (1, 1) {
            return Err(format!("{:?} is not a scalar", self.shape));
        }
        Ok(self.linear[0])
    }

    pub fn t(&self) -> Result<Self, String> {
        let (rows, cols) = self.shape;
        let mut transposed = vec![0.0; self.linear.len()];

        for r in 0..rows {
            for c in 0..cols {
                transposed[c * rows + r] = self.linear[r * cols + c];
            }
        }

        Ok(Self {
            linear: transposed,
            shape: (cols, rows),
        })
    }

    pub fn dot(&self, rhs: &Matrix) -> Result<Self, String> {
        if self.shape.1 != rhs.shape.0 {
            return Err(format!(
                "cannot multiply {:?} with {:?}",
                self.shape, rhs.shape
            ));
        }

        let (m, n) = self.shape;
        let (_, p) = rhs.shape;

        let mut result = vec![0.0; m * p];
        for i in 0..m {
            for j in 0..p {
                let mut sum = 0.0;
                for k in 0..n {
                    sum += self.linear[i * n + k] * rhs.linear[k * p + j];
                }
                result[i * p + j] = sum;
            }
        }

        Ok(Self {
            linear: result,
            shape: (m, p),
        })
    }

    pub fn apply<F>(&mut self, f: F) -> Result<&mut Self, String>
    where
        F: Fn(Fxx) -> Fxx,
    {
        self.linear.iter_mut().for_each(|x| *x = f(*x));
        Ok(self)
    }

    pub fn broadcast<F>(&mut self, value: &Matrix, f: F) -> Result<&mut Self, String>
    where
        F: Fn(Fxx, Fxx) -> Fxx,
    {
        if self.shape != value.shape {
            return Err(format!(
                "cannot broadcast {:?} with {:?}",
                self.shape, value.shape
            ));
        }
        self.linear
            .iter_mut()
            .zip(value.linear.iter())
            .for_each(|(a, b)| *a = f(*a, *b));
        Ok(self)
    }
}
