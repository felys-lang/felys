use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct BufVec<T, const N: usize> {
    buf: Rc<[T; N]>,
    vec: Vec<T>,
}

impl<T: Clone, const N: usize> BufVec<T, N> {
    pub fn new(buf: [T; N], vec: Vec<T>) -> Self {
        Self {
            buf: Rc::new(buf),
            vec,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.buf.iter().chain(self.vec.iter())
    }

    pub fn into_iter(self) -> impl Iterator<Item=T> {
        self.buf.to_vec().into_iter().chain(self.vec.into_iter())
    }

    pub fn len(&self) -> usize {
        self.buf.len() + self.vec.len()
    }

    pub fn buffer(&self) -> &[T; N] {
        &self.buf
    }
}

impl<T: Clone, const N: usize> BufVec<T, N> {
    pub fn vec(&self) -> Vec<T> {
        let mut vec = self.buf.to_vec();
        vec.extend(self.vec.clone());
        vec
    }
}
