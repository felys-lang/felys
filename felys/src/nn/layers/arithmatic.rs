use crate::nn::layers::interface::Differentiable;
use crate::nn::layers::{Layer, Operator};
use crate::nn::matrix::Matrix;

pub struct Add {
    subtree: [Layer; 2],
}

impl Differentiable<2> for Add {
    fn build(input: [Operator; 2]) -> Result<Operator, String> {
        let [mut x, y] = input;
        x.matrix.broadcast(&y.matrix, |x, y| x + y)?;
        let layer = Layer::Add(
            Self {
                subtree: [x.layer, y.layer],
            }
            .into(),
        );
        Ok(Operator::new(x.matrix, layer))
    }

    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; 2], String> {
        let [x, y] = self.subtree.clone();
        let dx = grad.clone();
        let dy = grad.clone();
        Ok([Operator::new(dx, x), Operator::new(dy, y)])
    }
}

pub struct Sub {
    subtree: [Layer; 2],
}

impl Differentiable<2> for Sub {
    fn build(input: [Operator; 2]) -> Result<Operator, String> {
        let [mut x, y] = input;
        x.matrix.broadcast(&y.matrix, |x, y| x - y)?;
        let layer = Layer::Sub(
            Self {
                subtree: [x.layer, y.layer],
            }
            .into(),
        );
        Ok(Operator::new(x.matrix, layer))
    }

    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; 2], String> {
        let [x, y] = self.subtree.clone();
        let dx = grad.clone();
        let mut dy = grad.clone();
        dy.apply(|x| -x)?;
        Ok([Operator::new(dx, x), Operator::new(dy, y)])
    }
}

pub struct Mul {
    subtree: [Layer; 2],
    grad: [Matrix; 2],
}

impl Differentiable<2> for Mul {
    fn build(input: [Operator; 2]) -> Result<Operator, String> {
        let [mut x, y] = input;
        x.matrix.broadcast(&y.matrix, |x, y| x * y)?;
        let layer = Layer::Mul(
            Self {
                subtree: [x.layer, y.layer],
                grad: [y.matrix, x.matrix.clone()],
            }
            .into(),
        );
        Ok(Operator::new(x.matrix, layer))
    }

    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; 2], String> {
        let [x, y] = self.subtree.clone();
        let [mut dx, mut dy] = self.grad.clone();
        dx.broadcast(grad, |x, y| x * y)?;
        dy.broadcast(grad, |x, y| x * y)?;
        Ok([Operator::new(dx, x), Operator::new(dy, y)])
    }
}

pub struct Div {
    subtree: [Layer; 2],
    grad: [Matrix; 2],
}

impl Differentiable<2> for Div {
    fn build(input: [Operator; 2]) -> Result<Operator, String> {
        let [mut x, y] = input;

        let mut dx = x.matrix.clone();
        dx.broadcast(&y.matrix, |_, y| 1.0 / y)?;
        let mut dy = x.matrix.clone();
        dy.broadcast(&y.matrix, |x, y| -x / y.powf(2.0))?;

        x.matrix.broadcast(&y.matrix, |x, y| x / y)?;
        let layer = Layer::Div(
            Self {
                subtree: [x.layer, y.layer],
                grad: [dx, dy],
            }
            .into(),
        );
        Ok(Operator::new(x.matrix, layer))
    }

    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; 2], String> {
        let [x, y] = self.subtree.clone();
        let [mut dx, mut dy] = self.grad.clone();
        dx.broadcast(grad, |x, y| x * y)?;
        dy.broadcast(grad, |x, y| x * y)?;
        Ok([Operator::new(dx, x), Operator::new(dy, y)])
    }
}

pub struct Pow {
    subtree: [Layer; 2],
    grad: [Matrix; 2],
}

impl Differentiable<2> for Pow {
    fn build(input: [Operator; 2]) -> Result<Operator, String> {
        let [mut x, y] = input;

        let mut dx = x.matrix.clone();
        dx.broadcast(&y.matrix, |x, y| y * x.powf(y - 1.0))?;
        let mut dy = x.matrix.clone();
        dy.broadcast(&y.matrix, |x, y| x.powf(y) * x.ln())?;

        x.matrix.broadcast(&y.matrix, |x, y| x.powf(y))?;
        let layer = Layer::Pow(
            Self {
                subtree: [x.layer, y.layer],
                grad: [dx, dy],
            }
            .into(),
        );
        Ok(Operator::new(x.matrix, layer))
    }

    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; 2], String> {
        let [x, y] = self.subtree.clone();
        let [mut dx, mut dy] = self.grad.clone();
        dx.broadcast(grad, |x, y| x * y)?;
        dy.broadcast(grad, |x, y| x * y)?;
        Ok([Operator::new(dx, x), Operator::new(dy, y)])
    }
}

pub struct Neg {
    subtree: [Layer; 1],
}

impl Differentiable<1> for Neg {
    fn build(input: [Operator; 1]) -> Result<Operator, String> {
        let [mut x] = input;
        x.matrix.apply(|x| -x)?;
        let layer = Layer::Neg(Self { subtree: [x.layer] }.into());
        Ok(Operator::new(x.matrix, layer))
    }

    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; 1], String> {
        let [x] = self.subtree.clone();
        let mut dx = grad.clone();
        dx.apply(|x| -x)?;
        Ok([Operator::new(dx, x)])
    }
}

pub struct Dot {
    subtree: [Layer; 2],
    grad: [Matrix; 2],
}

impl Differentiable<2> for Dot {
    fn build(input: [Operator; 2]) -> Result<Operator, String> {
        let [x, y] = input;

        let dx = y.matrix.t()?;
        let dy = x.matrix.t()?;

        let matrix = x.matrix.dot(&y.matrix)?;
        let layer = Layer::Dot(
            Self {
                subtree: [x.layer, y.layer],
                grad: [dx, dy],
            }
            .into(),
        );
        Ok(Operator::new(matrix, layer))
    }

    fn differentiate(&self, grad: &Matrix) -> Result<[Operator; 2], String> {
        let [x, y] = self.subtree.clone();
        let [dx, dy] = &self.grad;
        let dx = grad.dot(&dx)?;
        let dy = dy.dot(grad)?;
        Ok([Operator::new(dx, x), Operator::new(dy, y)])
    }
}
