use crate::builder::common::s2c;
use crate::philia093::Intern;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct BufVec<T, const N: usize> {
    buf: Rc<[T; N]>,
    vec: Vec<T>,
}

impl<T, const N: usize> BufVec<T, N> {
    pub fn new(buf: [T; N], vec: Vec<T>) -> Self {
        Self {
            buf: Rc::new(buf),
            vec,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.buf.iter().chain(self.vec.iter())
    }

    pub fn len(&self) -> usize {
        self.vec.len() + 1
    }
}

impl<const N: usize> BufVec<usize, N> {
    pub fn squeeze(&self, intern: &Intern) -> String {
        self.iter()
            .map(|c| s2c(intern.get(c).unwrap()))
            .collect::<String>()
    }
}
