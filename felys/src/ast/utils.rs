#[derive(Clone, Debug)]
pub struct BufVec<T, const N: usize> {
    buf: Box<[T; N]>,
    vec: Vec<T>,
}

impl<T, const N: usize> BufVec<T, N> {
    pub fn new(buf: [T; N], vec: Vec<T>) -> Self {
        Self {
            buf: Box::new(buf),
            vec,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buf.iter().chain(self.vec.iter())
    }
}

impl<T: Clone, const N: usize> BufVec<T, N> {
    pub fn vec(&self) -> Vec<T> {
        let mut vec = self.buf.to_vec();
        vec.extend(self.vec.clone());
        vec
    }
}
