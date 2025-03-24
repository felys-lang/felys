#[derive(Clone, Debug)]
pub struct Id(usize);

impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Id> for usize {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl From<&Id> for usize {
    fn from(value: &Id) -> Self {
        value.0
    }
}
